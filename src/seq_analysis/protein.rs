//! The data is based on Gasteiger _et al_ (2005) i.e. in accordance with
//! [ExPASy's ProtParam Tool](https://web.expasy.org/protparam/)
//! # References
//! - Gasteiger E., Hoogland C., Gattiker A., Duvaud S., Wilkins M.R., Appel R.D., Bairoch A.
//!   Protein Identification and Analysis Tools on the ExPASy Server, in _The Proteomics Protocols
//!   Handbook_, Humana Press (2005). pp. 571-607

use crate::utils::TextSlice;

use super::data::protein::{AMINO_ACID_FLEX, AMINO_ACID_MASS, AMINO_ACID_MASS_MONOISOTOPIC};
use phf::phf_map;
use std::collections::BTreeMap;
use std::fmt;

type AminoAcidCount = BTreeMap<u8, u32>;
type AminoAcidPercentage = BTreeMap<u8, f32>;

// -------------- used for calculating isoelectric point -------------------
const N_TERM_PKA_DEFAULT: f32 = 7.5;
const C_TERM_PKA_DEFAULT: f32 = 3.55;

enum Charge {
    Positive,
    Negative,
}

static PKA: phf::Map<u8, (f32, Charge)> = phf_map! {
    b'K'=> (10.0, Charge::Positive),
    b'R'=> (12.0, Charge::Positive),
    b'H'=> (5.98, Charge::Positive),
    b'D'=> (4.05, Charge::Negative),
    b'E'=> (4.45, Charge::Negative),
    b'C'=> (9.00, Charge::Negative),
    b'Y'=> (10.0, Charge::Negative),
};

static PKA_N_TERM: phf::Map<u8, f32> = phf_map! {
    b'A' => 7.59,
    b'M' => 7.00,
    b'S' => 6.93,
    b'P' => 8.36,
    b'T' => 6.82,
    b'V' => 7.44,
    b'E' => 7.70,
};

static PKA_C_TERM: phf::Map<u8, f32> = phf_map! {
    b'D' => 4.55,
    b'E' => 4.75,
};

// --------------------------------

#[derive(Debug)]
pub struct ProteinSeqAnalysisResult<'a> {
    pub seq: TextSlice<'a>,
    pub aa_count: AminoAcidCount,
    pub aa_percentages: AminoAcidPercentage,
    pub flexibility: Vec<f32>,
    pub isoelectric_point: f32,
    pub molar_extinction_coefficient: (u32, u32),
    pub molecular_weight: f64,
    pub molecular_weight_monoisotopic: f64,
}

#[derive(Debug)]
pub struct ProteinSeqAnalysis<'a> {
    pub seq: TextSlice<'a>,
    pub aa_count: AminoAcidCount,
}

impl<'a> ProteinSeqAnalysis<'a> {
    pub fn new(seq: TextSlice<'a>) -> Self {
        ProteinSeqAnalysis {
            seq,
            aa_count: Self::count_aa(seq),
        }
    }

    pub fn analyze(seq: TextSlice<'a>) -> ProteinSeqAnalysisResult {
        let analyzer = Self::new(seq);
        let aa_count = analyzer.aa_count.clone();
        let isoelectric_point = analyzer.isoelectric_point();
        let molar_extinction_coefficient = analyzer.molar_extinction_coefficient();
        let aa_percentages = analyzer.aa_percentages();
        let weights = analyzer.molecular_weights();
        let molecular_weight = weights.0;
        let molecular_weight_monoisotopic = weights.1;
        let flexibility = analyzer.flexibility();
        ProteinSeqAnalysisResult {
            seq,
            aa_count,
            aa_percentages,
            flexibility,
            isoelectric_point,
            molar_extinction_coefficient,
            molecular_weight_monoisotopic,
            molecular_weight,
        }
    }

    pub fn count_aa(seq: TextSlice) -> AminoAcidCount {
        let mut res: BTreeMap<u8, u32> = BTreeMap::new();
        for &aa in seq {
            let count = res.entry(aa).or_insert(0);
            *count += 1;
        }
        res
    }

    pub fn aa_percentages(&self) -> AminoAcidPercentage {
        let mut percentages: AminoAcidPercentage = BTreeMap::new();
        let len = self.seq.len();
        for (&aa, &count) in self.aa_count.iter() {
            percentages.insert(aa, count as f32 / len as f32);
        }
        percentages
    }

    /// Calculate the molecular weights.
    pub fn molecular_weights(&self) -> (f64, f64) {
        let mut mw = 0f64;
        let mut mw_monoisotopic = 0f64;
        for (aa, &count) in self.aa_count.iter() {
            match AMINO_ACID_MASS.get(aa) {
                Some(w) => mw += count as f64 * w,
                None => panic!(format!("Unknown amino acid '{}'", aa)),
            }
            match AMINO_ACID_MASS_MONOISOTOPIC.get(aa) {
                Some(w) => mw_monoisotopic += count as f64 * w,
                _ => unreachable!(),
            }
        }
        (mw, mw_monoisotopic)
    }

    /// Calculate the molar extinction coefficient (at 280 nm)
    ///
    /// Calculates the molar extinction coefficient assuming cysteines (reduced) and cystines residues (oxidised)
    pub fn molar_extinction_coefficient(&self) -> (u32, u32) {
        let mut mec_reduced = 0;
        if let Some(n) = self.aa_count.get(&b'W') {
            mec_reduced += 5500 * n;
        }
        if let Some(n) = self.aa_count.get(&b'Y') {
            mec_reduced += 1490 * n;
        }
        let mut mec_oxidised = mec_reduced;
        if let Some(n) = self.aa_count.get(&b'C') {
            mec_oxidised += n / 2 * 125;
        }
        (mec_reduced, mec_oxidised)
    }

    /// Calculate the flexibility according to Vihinen (1994)
    pub fn flexibility(&self) -> Vec<f32> {
        let window_size = 9usize;
        let weights = [0.25f32, 0.4375, 0.625, 0.8125, 1.0];
        let len_minus_window_size = self.seq.len() - window_size;
        let mut scores = Vec::with_capacity(len_minus_window_size);
        for i in 0..len_minus_window_size {
            let subsequence = &self.seq[i..i + window_size];
            let mut score = 0.0;
            for j in 0..window_size / 2 {
                let front = subsequence[j];
                let back = subsequence[window_size - j - 1];
                score += (AMINO_ACID_FLEX.get(&front).unwrap()
                    + AMINO_ACID_FLEX.get(&back).unwrap())
                    * weights[j];
            }
            let middle = subsequence[window_size / 2 + 1];
            score += AMINO_ACID_FLEX.get(&middle).unwrap();
            scores.push(score / 5.25)
        }
        scores
    }

    /// Estimate the isoelectric point of a polypeptide chain based on its primary structure.
    pub fn isoelectric_point(&self) -> f32 {
        self.pi_recursive(4.05, 12.0, 7.775)
    }

    fn pi_recursive(&self, mut x1: f32, mut x2: f32, xmid: f32) -> f32 {
        if x2 - x1 < 0.0001 {
            return xmid;
        }
        let charge = self.charge_at_pH(xmid);
        if charge > 0.0 {
            x1 = xmid;
        } else {
            x2 = xmid;
        }
        self.pi_recursive(x1, x2, (x1 + x2) / 2.0)
    }
    #[allow(non_snake_case)]
    pub fn charge_at_pH(&self, pH: f32) -> f32 {
        let mut charge = 0f32;
        for (aa, &count) in self.aa_count.iter() {
            if let Some((pKa, positivity)) = PKA.get(aa) {
                match positivity {
                    Charge::Negative => {
                        let partial_charge = 1.0 / (10f32.powf(pKa - pH) + 1.0);
                        charge -= partial_charge * count as f32;
                    }
                    Charge::Positive => {
                        let partial_charge = 1.0 / (10f32.powf(pH - pKa) + 1.0);
                        charge += partial_charge * count as f32;
                    }
                }
            }
        }
        let n_term_pKa = if let Some(&pKa) = PKA_N_TERM.get(&self.seq[0]) {
            pKa
        } else {
            N_TERM_PKA_DEFAULT
        };
        charge += 1.0 / (10f32.powf(pH - n_term_pKa) + 1.0);
        let c_term_pKa = if let Some(&pKa) = PKA_C_TERM.get(&self.seq[self.seq.len() - 1]) {
            pKa
        } else {
            C_TERM_PKA_DEFAULT
        };
        charge -= 1.0 / (10f32.powf(c_term_pKa - pH) + 1.0);
        charge
    }
}

impl<'a> fmt::Display for ProteinSeqAnalysisResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Number of amino acids: {}", self.seq.len())?;
        writeln!(f, "Molecular weight: {:.3}", self.molecular_weight)?;
        writeln!(
            f,
            "Molecular weight (monoisotopic): {:.3}",
            self.molecular_weight_monoisotopic
        )?;
        writeln!(f, "Theoretical pI: {}", self.isoelectric_point)?;
        writeln!(
            f,
            "Molar extinction coefficient at 280 nm measured in water:"
        )?;
        writeln!(
            f,
            "     {}, assuming all Cys residues are reduced",
            self.molar_extinction_coefficient.0
        )?;
        writeln!(
            f,
            "     {}, assuming all pairs of Cys residues form cystines",
            self.molar_extinction_coefficient.1
        )?;
        writeln!(f, "┏━━━━━━━━━━━━━━━━━━━━━━━━┓")?;
        writeln!(f, "┃{}┃", "Residue│count│percentage")?;
        writeln!(f, "┠———————┼—————┼——————————┨")?;
        for ((&aa, count), (_, percentage)) in self.aa_count.iter().zip(self.aa_percentages.iter())
        {
            writeln!(
                f,
                "┃{:^7}┆{:>5}┆{:9.2}%┃",
                aa as char,
                count,
                percentage * 100.0
            )?;
        }
        writeln!(f, "┗━━━━━━━┷━━━━━┷━━━━━━━━━━┛")
    }
}

#[cfg(test)]
mod tests {
    use super::ProteinSeqAnalysis;
    use lazy_static::lazy_static;

    static S1: &[u8;152] = &b"MAEGEITTFTALTEKFNLPPGNYKKPKLLYCSNGGHFLRILPDGTVDGTRDRSDQHIQLQLSAESVGEVYIKSTETGQYLAMDTSGLLYGSQTPSEECLFLERLEENHYNTYTSKKHAEKNWFVGLKKNGSCKRGPRTHYGQKAILFLPLPV";

    lazy_static! {
        static ref A1: ProteinSeqAnalysis<'static> = ProteinSeqAnalysis::new(S1);
    }

    #[test]
    fn test_aa_count() {
        assert_eq!(A1.aa_count.get(&b'A'), Some(&6u32))
    }

    #[test]
    fn test_aa_percentages() {
        assert!((A1.aa_percentages().get(&b'A').unwrap() - 0.03947).abs() < 0.0001)
    }

    #[test]
    fn test_isoelectric_point() {
        assert!((A1.isoelectric_point() - 7.72).abs() < 0.01)
    }

    #[test]
    fn test_molar_extinction_coefficient() {
        assert_eq!(A1.molar_extinction_coefficient(), (17420, 17545))
    }
}
