#![feature(test)]

extern crate test;

use bio::alignment::pairwise::*;
use test::Bencher;

// 5,000 random nucleotides, GC content = .55
static STR_1: &'static [u8] = b"ATCTAACTATTCCCTGTGCCTTATGGGGGCCTGCGCTATCTGCCTGT\
CGAACCATAGGACTCGCGCCAGCGCGCAGGCTTGGATCGAGGTGAAATCTCCGGGGCCTAAGACCACGAGCGTCTGGCG\
TCTTGGCTAACCCCCCTACATGCTGTTATAGACAATCAGTGGAAACCCGGTGCCAGGGGGTGGAGTGACCTTAAGTCAG\
GGACGATATTAATCGGAAGGAGTATTCAACGCAATGAAGCCGCAGGGTTGGCGTGGGAATGGTGCTTCTGTCCAAGCAG\
GTAAGGGCATGAGGCCGCAACCGTCCCCCAAGCGTACAGGGTGCACTTTGCAACGATTTCGGAGTCCGGAGACTCGCTG\
TTTTCGAAATTTGCGCTCAAGGGCGGGTATTGAACCAGGCTTACGCCCAAGAACGTAGCAAGGTGACTCAAACAAGGTA\
CATCTTGCCCGCGTTTCACACGAATCAAGTTGGAGGTTATGGAGCATAGTAACACGTGGGCGGCCAGTGGTCGGTTGCT\
ACACCCCTGCCGCAACGTTGAAGGTCCCGGATTAGACTGGCTGGACCCATGCCGTGACACCCGTCACACTCCATTACCG\
TCTGCGGGTCACGGCTTGTTGTGGACTGGATTGCCATTCTCTCAGTGTATTACGCAGGCCGGCGCGCGGGTCCCATGTA\
AACCTGTCATAGCTTACCTGACTCTACTTGGAAGTGTGGCTAGGCCTTTGCCCACGCACCTGGTCGGTCCTCGTTTGCT\
TTTTAGGACCGGATGAACTACAGAGCGCTGCAAGAATCTCTACCTGCTTTACAAAGCGCTGGGTCCTACTCCAGCGGGA\
TGTTTTATCTAAACACGATGAGAGGAGTATTCGTCAGGCCACATGGCTTTCTTGTCCTGGTCGGATCCATCGTTGGCGC\
CCGACCCCCCCACTCCGTAGTGAGTTCTTCGTCCGAGCCATTGCATGCCAGATCGGCAGACAGATAGCGGATCCAGTAT\
ATCCCTGGAAGCTATAGACGCACAGGTTGGAATCCTAAGCGAAGTCGCGCGTCCGAACCCAGCTCTACTTTAGTGGCCA\
CGGGTTCTGGTCCCCCCGGGCCGCGGAACCGATTAGGGCCATGTACAACAATACTTATTAGTCACCTTTCAGACACGAT\
CTCCCTGCTCAGTGGTATATGGTTCCTGCTATAATTAGCCACCCTCATAAGTTGCACTACTTCTGCGACCCAAGTGCAC\
CCTTACCACGAAGACAGGATTGTCCGATCCCATACTGCGGCCTTGGCAGGGGGTTCGCAAGTCCCACCCCAAACGATGC\
TGAAGGCTCAGGTTACACAGGCACAAGTGCTATATACGCGAGTTCCCGCTCTTAACCTGGACCGAATGCGGGATCATGC\
ATCGTACCACTGTGTTCGTGTCATCTAGGACGGGCGCAAAGGATACATAGTTCAATCAAGAATACCTTGTATTATTGTA\
CACCTACCGGTCACCAGCCAACAATGTGCGGACGGCGTTGCGACTTGCTGGGCCTGATCTCACCGCCCTAGATACCGCA\
CACTGGGCAATACGAGGTAAAGCCAGTCACCCAGTGTCGATCAACAGCTGACGTAACGGTAAGAGGCTCACAAAATCGC\
ACCGCCGGCGTCCCCTGGGTATTTTACGTCAGCATCGGGTGGACTGGCATGAATCTTTACTCCCAGGCGGAAACGGGTG\
CGTGGACAAGCGAGCAGCAAACGAAAATTCCTGGCCTGCTTGGTGTCTCGTATCCCTCTTGGAGATCGAGGAAATGTTT\
CACGACCAAGGGAAAGGTCGCCCTACGAAATAGATTTGCGCTACTGTCCGCATAAGGAGTCCGGTGTAGCGAAGGATGA\
AGGCGACCCTAGGTAGCAACCGCCGGCTTCGGCGGTAAGGTATCACTCAGGAAGCAGGCACGGAAAGACACGGTCTAGC\
AGACCGTCTATCGGCTAGGTCAAATAGGGTGCTTTGATATCAGCATGTCCAGCCTTAGAATTCAGTTCAGCGCGCTGGT\
CTGGGTCGAGATAAAATCACCAGTACCCAAGACCAGGCGGGCTCGCCGCGTTGGCTAATCCTGGTACATCTTGTAATCA\
ATGTTCAGAAGAAAATCTGTGTTAGAGGGACGAGTCACCACGTACCAATAGCGACAACGATCGGTCGGACTATTCATCG\
TGGTGGTGACGCTCGGATTACGCGGGAAAGGTGCTTGTGTCCCGACAGGCTAGGATATAATGCTGAGGCGCTGCCCCAA\
CCGTTCAGCGTGGGGTTTGCTACAACTTCCGAGTGCTACGTGTGCGAGACCATGTTATGTATGCACAAGGCCGACAATA\
GGACGTAGCCTTCGAGTTAGTACGTAGCGTGGTCGCACAAGCACAGTAGATCCTCCCCGCGCATCCTATTTATTAAGTT\
AATTCTATAGCAATACGATCACATGCGGATGGGCAGTGGCCGGTAGTCACACGCCTACCGCGGTGCTCAATGACCGGGA\
CTAGAGAGGCGAAGATTATGGCGTGTGACCCGTTATGCTCGAGTTCGGTCAGAGCGTCATTGCGAGTAGTCGATTGCTT\
TCCCAATCTCCGAGCGATTTAGCGTGACAGCCCCAGGGAACCCACAAAATGCGATCGCAGTCCACCCGATCGTACACAG\
AAAGGAGGGTCCCCATACGCCGACGCACCTGTTCGCACGTCGTATGCATAAACGAGCCGCACGAACCAGAGAGCATAAA\
GAGGACCTCTAGCTCCTTTACAAAGTACAGGTTCGCCTGCCGCCGGGATGCCTTACCTAGACGCAATGACGGACGTATT\
CCTCTGGCCTCAACGGTTCCTGCTTCCGCTGGGATCCAAGATTGGCGGCCGAAGCCGCCTTTCCAAAGTGAGTCCTTCG\
TCTGTGACTAACTGTGCCAGATCGTCTTGCAAACTCCCGATCCAGTTTAACTCACCAAACTATAGCCGTACAGACCCAA\
ATCTTAAGTCATATCACGCGACTAGCCTCTGCTCAATTTCTGTGCTCAAGGGTTTTGGTCCGCCCGAGCGGTGCAGCCG\
ATTAGGACCATCTAATGCACTTGTTACAAGACTTCTTTTAAACACTTTCTTCCTGCCCAGTGGCGGATGATAATGGTTG\
TTGCCAGCCGGCGTGGAAGGTAACAGCACCGGTGCGAGCCTAATGTGCCGTCTCCACCAACACAGGGCTGTCCGGTCGT\
ATAATAGGACTCCGCAATGGGGTTAGCAAGTGGCAGCCTAAACGATGTCGGGGACTCGCGATGTACATGCTCTGGTTCA\
ATACATACGTGACCCGGCAGTTATCCTGCATCGGAACGTCAATCGTGCATCGGGCCAGCGTAATCGTGTCATCTGGGAG\
GCGGCCGTAGGATAAATAATTCAATAAAGATGTCGTTTTGCTAGTATACGCCTAGGCGTCACCCGCCATCTCTGTGCAG\
GTGGGCCGACGAGACACTGCCCCTGATTTCTCCGCTACTAATAGCACACACGGGGCAATACCAGCACAAGCCAGTCTCG\
CGGGAACGCTCGTCAGCATACGAAAGAGCTTGAGGCACGCCAATTCGCACTGTCGGGGTCGCTTGGGTGTTTTGCACTA\
CCGTCAGGTACGCTAGTATGCGTCCTTCCTTCCAGGGGTATGTGGCTGCGTGGTCAAAAGTGCGGCATTCGTATTTGCT\
CCCCGTGCTTGCTCTCACGAACTTGACCTGGAGATCAAGGAGATGCTTCTTGTGGAACCGGACAGCGCATCAACGCAAC\
GGATCTACGTTACAGCGTGCATAGCGAGAACGGAGTTGCCGACGACGAAAGCGACACTGGGATCTGTCCGTCGTCATTC\
GCGGAAAGCATCCGCTCACGAGGCGGACACTGATTGACACGGTTTTGCAGAAGGTTAGGGGAATAGGTCAAATTGAGTG\
GCTTAAAAACGCTATGTCTGGGATTAAAGTGTAGTAAACTGCGGTCAACGGAGACGGTTTTAAGACAGGAGTTCGCAAA\
ACCAGGCGGGGTCGCCACGACGGCTATTCCTGGTGGTTTAGGCGTACAATGTCCTGAAGAATATTTAAGAAAGAAGCAC\
CCCTCGTCGCCTAGAATTACCTACCGCGGTCGACCATACCTTCGATTGTCGCGCCCACCCTCCCATTAGTCGGCAGAGG\
TGGTTGTGTTGCGATAGCCCAGCATGATATCCTAAGGCGTTACGCCGATGGATATCCCACGGAATTGCCATAGGCGCTG\
AACGCTACACGGACGATACGAACTTATGTATGGAGCGGGTCATCGAAAGGTCATACCCTTGTAGTTAACATGTAGCCCG\
GCCCTATTAGTACAGCAGTGCCTTGAGCGGCATTCTCATTATTAAGTTTTCTCTACAGCCAAACGACCAAGTGCACTTC\
CGCGGAGCGCGGTGGAGACTCGTCCACCCGGCAGCTCTGTAATAGGGACTAAAAGAGTGATGATAATCATGAGTGCCGC\
GTTATGGTGGTGTCGGAACAGAGCGGTCTTACGGCCAGTCGTATCCCTTCTCGAGTTCCGTCCGGTTAAGCGTGACACT\
CCCAGTGTACCCGCAAACCGTGATGGCTGTGCTTGGGGTCAATCGCATGTAGGATGGTCTCCAGACACCGGGGCACCAG\
TTTTCACGCCCAAAGCATAAACGACGAGCAGTCATGAGAGTCTTAGAACTGGACGTGCCGTTTCTCTGCGAACAACACC\
TCGAGCTGTACCGTTGTTGCGCTGCCTAGATGCAGTGCCGCTCCTATCACATTTGCCTCGACGACTGCCGCCTTCGCTG\
TTTCCCTAGACACTCAACAGTAAGCGCCTTTTGTAGGCAGGGGCACCCCCTGTCAGTGGCTGCGCCAAAACGTCTTCGG\
ATCCCCTTGTCCAATCAAACTGACCGAATTCTTTCATTTAAGACCCTAATATGACATCATTAGTGACTAAATGCCACTC\
CCAAAATTCTGCCCAGAAGCGTTTAAGTTCGCCCCACTAAAGTTGTCTAAAACGA";

// 5,000 random nucleotides, GC content = .45
static STR_2: &'static [u8] = b"CTAAAGTGGCGAAATTTATGGTGTGTGACCCGTTATGCTCCATTTCG\
GTCAGTGGGTCATTGCTAGTAGTCGATTGCATTGTCATTCTCCGAGTGATTTAGCGTGACAGCCGCAGGGAACCCATAA\
AATGTAATCGTAGTCCATCTGATCGTACTTAGAAATGAAGGTCCCCTTTTACCCACGCACCTGTTTACTCGTCGTTTGC\
TTTTAAGAACCGCACGAACCACAGAGCATAAAGAGAACCTCTAGTTCCTTTACAAAGTACTGGTTCCCTTTTCAGCAAG\
ATGCCTTATCTAAATGCAATGACAGACGTATTCCTCAGGCCACATCGCTTCCTACTTTCGCTGGGATCCATCATTGGCA\
GCTGAAACCGCCATTCCATAGTGAGTCCTTCGTCTGTGTCTTTCTGTGCCAAATCGTCTAGCAAATTGCTGATCCAGTT\
TATCTCACGAAATTATAGTCATACAGACCGAAATTTTAAATCAAATCACGCGACTAGGCTCAGCTTTATTTTAGTGGTC\
ATGGGTTTTGGTCCGCCCGAGCGGTGCAACCGATTAGGACCATGTAAAACATTTGTTACAAGTCTTCTTTTAAATACAA\
TCTTCCTGCTCAGTAGCGCATGATTATCGTTGTTGCTAGCCAGTGTGGTAAGTAACAGCACCACTGCGAGCCTAATGTG\
CCCTTTCCACGAACACAAGGCTATCCGATCCTATATTAGGATTCCGCAATGGGGTTAGCAAATCGCACCCTAAACGATA\
TTGAAGACTTGCGATGTACATGCTTTGGTACAATACATACGTGTTCCAGTTGTTATCCTGTATCGGAACTTCAATTATG\
CATCGCACCAGCATATTCATGTCATCTAGGAAGAGCGCGTAGGATAAATAATTCAATTAAGATGTCGTTATGCTAGTAT\
ACGTCTACCCGTCACCGGCCATCTGTGTGCAGATGGGGCGACGAGTTATTGACCCTGATTTCTCCACTTCTAATACCAC\
ACACTGGGCAATACGAGCTCAAGCTAGTCTCGCAGTAACGCTCATCAGCTAACGAAAGAGTTAAAGGCTCGCTAATTCG\
CACTGTCAGGGTCTCTTGGGTGTTTTGCACTAGCGTCAGGTAGGCTAGTATGTGTTTTTCCTTCCAGAGGTATGTGGCT\
GCGTGGTCAAATGTGCAGCATACGTATTTGCTCGACGTGTTTAGTCTCTCATACTTCTCCTGGAGATCAAGGAAATGTT\
TCTTGTCCAAGTGGACAACGGTTCTACGGAATGGATCTACGTTACTGCCTGCATAAAGAAAACGGAGTTGCTAAGGACG\
AAAGCGACTTTAGGTTCTAACTGTTGACTTTGGCGGAAAAGTTTCATTCAGGAAGCAGACACTGATTGACACGGTTTAG\
CAGAACGTTTGAGGATTAGGTTAAATTGAGTGGTTTAATATTGGTATGTCTGGGATTAAAATATAGTATAGTGTGTTAA\
TCGGAGACGAATTAAAGACACGAGTTCCCAAAATCAAGCGGGCTCATTACAACGGTTAATCCTGGTAGTTTACGTGAAC\
AATGTTCTGAAGAAAATTTATGAAAAAAGGACCCGTCATCGCCTACAATTACCTACAACGGTCGACCATACCTTCGATT\
ATCGTGGCCACTCTCGGATTACACGGCAGAGGTGGTTGTGTTCCGATAGGCCAGTATATTATTCTAAGGCGTTACCCTA\
ATCATTTTTCATCGGATTTGCTATAGCCCTTGAACGCTACATGCACGAAACCAAATTATGTATACACTGGGTCATCAAT\
AGGATATAGTCTTGTAGTTAACATGTAGCCCGGCCGTATTAGTACAGTAGAGCCTTCATTGACATTCTGTTTATTAAAT\
TATTTCTACAGCAAAACGATCATATGCAAATCCACAGTGCGCGATAGAGATACATTCACTCGGCTGCTCTGTAATAGGG\
ACTAAAAAAGTGATGATTATCATGAGTGCCCCGTTATGGTCGTGTTCGATCAGAGCGCTCTTACGAGCAGTCGTATACT\
TTCTCGAATTCCGTGCAGTTAAGCGTGACAGTCCCAGTGAACCCACAAAACGTGATGGCAGTCCATGCAATCATACGCA\
AGAAGGATGGTCTCCAGACACCGGCGCACCAGTTTTCACGCCGAAAGCATAAACGAGGAGCACAAATGAAAGTGTTTGA\
ACTGGACCTGTAGTTTCTCTACGAAAAATACCTTGAGCTGTTGCGTTGTTGCGCTGCCTAGATGCAGTGTTGCACATAT\
CACTTTTGCTTCAACGACTGCTGCTTTCGCTGTAACCCTAGACAGACAACAATAAGCGCTTTTTGTAGGCAAGAGCTCC\
GCCTATGACTAACTGCGCCAAAACATCTTCCAATCCCCTTATCCAATTTAATTCATCGAATTCTTACAATTTAGACCCT\
AATATCACATCATTAGACATTAATTGCCTCTGCCAAAATTCTGTCTACAAATGTTTTAGTTCGCTCCAGTAAAGTTGTT\
AATAACGACTACTAAATCCGCATGTTACGGGATTTCTTATTAATTCTTTTTTCGTAAGGAACAGCGGATCTTAATGGAT\
GGCGCCAGGTGGTATGGAAGCTAATAGCGCGGGTGAGAGGGTAATTAGCCGTCTTCACCAACACAACGCTATCGGGTCA\
TACTATAAGATTCCACAATGCGACTACTTATAAGATGTCTTAACGGTATCCGCAACTTGTGATGTGCCTACTATGCTTA\
AATGCATATCTCGCTCAGTAACTTTCCAATATGAGAGCATCAATTGTAGATCGGGCCGAGATAATCATGTCGTCACGGA\
ACTTATTGTAAGAGTAATAATTTAAAAGAGATGTCAGTTTGCTGGTTCACGTAAAGGTTCCTCACACTACCTCTAAATA\
AGTGAGCGGTCGTGACATTATCCCTGATTTTCTCACTACTATTAGTACTCACGACACAATTCTACCACAGCCTTGTTTC\
GCCAGAATGCCAGTCAGCATAAAGAAGAGCTCAAGGCAGGTCAACTCGCATTGTGAGAGTTACATGAACGTTCGGCACT\
ACCGACACGAACCTCAGTTAGCGTACATCCTACCAGAGGTCTGTGGCCCCGTGGTCAAAAGTGCGGATTTCGTATTTGC\
TGCTCGTCAGTACTTTCAGAATCATGACCTGCACGGTAAAAAGACGCTTATTATGGAGTTCGACATGGCAATAACGCGA\
CGAATCTACGTCATGACGAGAATAGTATAAACAAAACTGCTGACGGCAGAAGCGTCAAAGAAGTCTGTAAATTGTTATT\
CGCGAAAAACATCCGTCTCCGTGGGGGATAATCACCGACGCCATTTTATAGAAGCCTAGGGGAACAGATTGGTTTAATT\
AGCTTAAGAAAGTAAATTCTGGGATTATACTGTAGTAATCACTAATTTACGGTGAGGGTTTTATGGCGGATTTTTACAA\
ATTCAAACCAGGTGATTTCAACAAATTTTGTTGACGATTTAGGCGCACTATCCCCTAAACTACAAATTAAAAAATAGCG\
TTCCTTGACGGCTAGAATTACTTACCGGCCTTCACCATACCTTCGATATTCGCGCCCACTCTCCCATTAATCCGTACAA\
GTGGATGTAATGCGATTGTCCGCTAAGATATTCTAACGTGTAACGTAGATAAGTATTTTACAGAGTTGCCGTACGCGTT\
GAACACTTCACAGATGATAGGAATTTGCGTATAGAGCGTGTTATTGAGGAGTTATACACCCGTAGACTACAATGGGCCC\
AACTCAATCAGAACTCGAGTGCCTTGAATAACATACTCATCACTAAACATTCTCAACAATCAATCGAGCAAGTCCATTA\
TCAACGAGTGTGTTGCAGTTTTATTCTCTTGCCAGCATTGTAATAGGCACTAAAAGAATGATGATAGTCATGAGTACTG\
AGCTAAGACGGCGTCGATGCATAGCGGACTTTCGGTCAATCACAATTCCTCACGAGACTCGTCCTGTTGAGCGTATCAC\
TCTCAATGTACAAGCAACCCAAGAAGGCTGTGCCTGGACTCAACTGGATGCAGGATGAACTCCAGACACGGGGTCACTA\
CTCTTCATACATAAAGCAAGAACGTCGAACAGTCATGAAAGTCTTAGTACCGCACGTACCATCTTACTGTGAATATTGC\
TTGAAGCTGTACCGTTATTGGGGGGCAAAGATGAAGTTCTCTTCTTTTCATAATTGTACTGACGACAGTCGTGTTCTCG\
GTTTCTTCAAAGGTTAAAGAATAAAGGCTTATTGTAGGCAGAGGAACGCCCTTTTAGTGGCTGGCGTTAAGTATCTTCG\
GACCCCCTTGTCTATCCAGATTAATCGAATTCTCTCATTTAGGACCTTAGTAAGTCATCATTGGTATTTGAATGCGACC\
TTGAAGAAACCGCTTAAAAATGTCAATGGTTGATCCACTAAACTTCATTTAATTAACTCCTAAATCAGCGCGATAGGCT\
ATTAGAGGTTTAATTTTGTATAGCAAGGTACTTCCGATCTTAATGAATGGCCGGAAAAGGTACGGACGCGATATGCGAG\
GGTGAAAGGGCAAATAGACAGGTTCGTCTTTGTCACGCTAGGAGGCAATTCTATAAGAATGCATATTGCATCGATACAT\
AAAATGTCTCGATCGCATGCGCAATTTGTGAAGTGTCTATTATCCCTAAGCCCATTTCCCGCATAATAACCCCTGATTG\
TATCCGCATTTGATGCTACCCAGGTTGAGTTAGCGTCGAGCTCGCGGAACTTATTGCATGAGTAGAGTTGAGTAAGAGC\
TGTTAGATGGCTCGCTGAACTAATAGTTGTCCACAGAACGTCAAGATTAGAAAACGGTTGTAGCATTATCGGAGGTTCT\
CTAACTACTATCAATACCCGTGTCTTGACTCTGCTGCGGCTACCTATCGCCTGAAAACCAGTTGGTGTTAAGGGATGCT\
CTGTCCAGGACGCCACATGTAGTGAAACTTACATGTTCGTTGGGTTCACCCGACT";

#[bench]
fn bench_aligner_wc_local(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = Aligner::with_capacity(STR_1.len(), STR_2.len(), -5, -1, &score);
    b.iter(|| aligner.local(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_global(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = Aligner::with_capacity(STR_1.len(), STR_2.len(), -5, -1, &score);
    b.iter(|| aligner.global(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_semiglobal(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = Aligner::with_capacity(STR_1.len(), STR_2.len(), -5, -1, &score);
    b.iter(|| aligner.semiglobal(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_global_myers_miller(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| aligner.global(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_semiglobal_myers_miller(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| aligner.semiglobal(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_local_myers_miller(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| aligner.local(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_global_fast(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = fast::Aligner::new(-5, -1, &score);
    b.iter(|| aligner.global(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_global_fast_unsafe(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = fast::Aligner::new(-5, -1, &score);
    b.iter(|| aligner.global_unsafe(STR_1, STR_2));
}

#[bench]
fn bench_aligner_wc_global_cost_only_no_push_no_pq_by_col(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| {
        let (cc, dd) = aligner.cost_only_no_pq_by_col(STR_1, STR_2, false, -5);
    });
}

#[bench]
fn bench_aligner_wc_global_cost_only_no_push_no_pq_by_col_swap(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| {
        let (cc, dd) = aligner.cost_only_no_pq_by_col(STR_2, STR_1, false, -5);
    });
}

#[bench]
fn bench_aligner_wc_global_cost_only_no_push_no_pq_by_row(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| {
        let (cc, dd) = aligner.cost_only_no_pq_by_row(STR_1, STR_2, false, -5);
    });
}

#[bench]
fn bench_aligner_wc_global_cost_only_no_push_no_pq_by_row_swap(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| {
        let (cc, dd) = aligner.cost_only_no_pq_by_row(STR_2, STR_1, false, -5);
    });
}

#[bench]
fn bench_aligner_wc_global_cost_only_no_push_pq(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = myers_miller::Aligner::new(-5, -1, &score);
    b.iter(|| {
        let (cc, dd) = aligner.cost_only_pq(STR_1, STR_2, false, -5);
    });
}
