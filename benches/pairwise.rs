#![feature(test)]

extern crate test;

use bio::alignment::pairwise::*;
use bio::scores::blosum62;
use test::Bencher;

// 5,000 random nucleotides, GC content = .55
static DNA_1: &'static [u8] = b"ATCTAACTATTCCCTGTGCCTTATGGGGGCCTGCGCTATCTGCCTGT\
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
static DNA_2: &'static [u8] = b"CTAAAGTGGCGAAATTTATGGTGTGTGACCCGTTATGCTCCATTTCG\
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

// 5,000 random amino acids
static PROTEIN_1: &'static [u8] = b"TASPMHGKVQEYKETAQKINLYHFLRLPDEXAFDFFAQLXYYA\
QVTTAKNZESMKEGMEQGACKYWCKLWZTQYFMYWTVHFLVKBQYCMRGYAPXIXBPTLPFYXAYBBDMWDCKYKQDBP\
DDQASWHAPDTDMTBQVRVQYARBKYBPGAZIBZDKQENGDKLENWMCCYXLNDFTBYTYFHGHTDBEPSWYNKYZMIC\
EGISYVGHELVBFYLVTARGHNMQGKPAVVIBYQRDKMVITHWFEERKHXNRHNBDMFFXDBWEDDNAXYPYHSDDQWT\
DXHKPKSZCAIATVKMRZEWSBIQEBYQWKNDCKMWGXWEKLXCIBMHSKKZRBNEEAGFTALNDVTYGDRPTFDABGS\
HBFICEHCWSRQIBYWHCXRTSMFYVVHGAPKHEZLSTTSSVXINVLHEGQXSSWRIXSWTPEGKTFSRNXWGFSHXZK\
NXEMBXZRWFNMPSDHVLFBQHZKQZIYKETQLDSKRNKHBDHXBRCSHPERPITHZYZKRZXWCWDKPRDDEFIDAQT\
CNEFTWEBKHLDDZIBVRNPRAEYFMNWGQECWVASMKAHGEXXGFDMHDWLGCEQSYPVMGWMTDHZKLPANEBNRVP\
MFKDPRIAKTEHIBRKTPCEINIDDZRHKZEPMERYMZHBMPDWPLGTIZWWIHQEWYREDHSXDVWQGIDHTSLKCYT\
SLBDKGVIVBMMXFGXZDQDRYLDBRAYPVZESYEBCXBKKHLTIXWPIIAHGQBQRIBANFZSQDNDHXKZYYEFVPR\
KNYBQLHKDXVNESRDWVWTVIWLRXEPTBGFDRHMHAYXAKCYGITMEQTDDPLTVFMPYAWGRQHREWABHPFSAER\
YEFWNXLCNRZRQWVWKZNAQBENVHZTIHBSKEMAXGPVIIYAAVLQPMTSEHESGEALSPEHCXQWKDIFPAQNCQF\
VWMECHCIANQGXHXEXCNFZLIRRWRLQMDFNVQFZWAVCSYMERMWWNYSAIYCLRQYKBYMNIDWRDZPXECGTXW\
CTCHPGSMHZRAPYKAQSWTHKFLIEFCXFPNSBDFHYWYBDQETPCNBCQVZNRRMVEVYERBYKIVTWTFWPHRWMK\
LEYSGRSWNBLSRTNEWIWMWFSLFXPTFCXQSQYKRAVDZVMWXCBQWVDEXCIRXDIKXFALBKCBPFRQIVHMBRD\
EIRYHZBRHSLVZCYQLEMBCWDVGGBDNIKADHBXAMRXLDSEWXLLEGREVNCDFMPWPLCGDKBBCSYCATZLHKN\
BZECYRARMQZYNWXTACLTQGLVFAIYRBKBRQPFIGFEMZRZWQRWWLHSLKQZQLTZIEIGLEENMHNCHQBMZEE\
IAMTNILDCXHKBRKSBQPLMXWFEIRYKLMPCCGSBSLKTBDKHPSPTLKPLIDDKNIFZZWWZTTWNWPIYAGHLCK\
ILSYXXZCCNFEMTAGDDDACLCCSKDQYLWSRGXKLBAHEQGEAIBPRNYQANRLBZQMHKVGFQBQVXHDNNZBAPV\
ASQVKACRHWZPGWBYPNBWDYDAPHPFNACLSQCZTFDEWNEQAVPGMNQSMCPFRCQNBZFHRPQZAHLHKCTYXEX\
WIAQXXZBCARZQXKAEBNPKYRXQHPWCPVGNHINPIHGFEMDSPBCBFRPVHZVMGLKBRATDYLDKKGICMQQKMB\
LALLRNTQWPSAEAZBAMFNYIRYRCPHTTWXPQPLIXACPWFNXDBAPCLMXFWGNMXRDXKHHFAQEXPEAKADVSF\
RXGSRLWSDGATCXREYFAESDAPRGNFKRSCFMSNWFQRHNIVCSYALRRRPKSICCGCVAXXCZZAHHNPPDSIRWP\
BQMPQFMBVHFLMFICTDGNARTXPAVHYWAEGRLCWLLQDFATGBEBWMLGIKDQZGMPNGWQGTFPDTHGMGNZAKI\
ZMPXLCVAZZGTQVMWAVEZMNGCMWNXEYHXPGPGHRHCFSALHNQGQFHHTNHZXIRZVYEFFNWWIWPKQWASSMT\
ZNVCLWNINSMNHFASWYREKGRAZIGPKQCNFRWFWMTVATVEEZYSDAZBELTAPTQBKXRHCBSVQBHCZRDQHLX\
KVMLEYLNQAFCSEFKZSEIZISYPFWLDLNSVHQXFSGCGFSTZGWDBQWCQEPXEMNACXTGDHTWZNCISWQZEYG\
NQVSCXKXYCRHSPDTSSSWPAQFAAXVQAFLXHEDYDCYPYKKNVRPVFMCFRIMHXENFAPHIAZHWGWHBAVWHBT\
TLKWHATZBWKDIHMXZTGYPCZXAYYEEGLIMKKGLTGWIZGDQHTPLBLFEDPMIKDQCZLBFIHDILVZAFZMRHM\
EXFBSFGAIFIGVDPCFRXBRGEYZLVVSSVSCELTNFQQMBZKVCZDRLLXGLRCWHSXGWYNHAIFRMKNSLXHYEP\
WVHDQSWMBQSINMMDQDPSECQEVMHVDANSHERHADDNCLCVLWCYANTEEMIRIETBVKKMEQMEYMLLTNRTYXH\
YFFPXGETFGIMHKYPIQVESYDHZQVQKLGKVZTFXBWQMVLQAYDGBAHLDDYBDMPDNEHFPZDCDSCAZDEASRV\
NPWSLVMIQFKKBCGWGSXWIEWTFMYQWBENVFFVGRXXVTKNSFMBCFATGTWGVGQPIRPHYAMVBGAWBEZWRQZ\
DIMWKCLAIAGGRBZEKBBYTAZLWYIQWETBKFAAYANVLQZRCSZXXFYHEIZSIPEDKAMFDRELWAIYTBQLDMV\
ETEGMWAFFLBMZRYMPSYPRDRAEZXFHBGECNRDSHSVZWTYMYZZACZNRIZKQVBWSAKMYQFIYRDIIXPQVHX\
HEHSRLHTLBWGZPPXEMRYXRHBVNTQGKSKYLSAPYEWXWGVTGDWMMMBQXCGFACREQVPEMPITRRAWXWLCVA\
TRZIQQYHMKHARMLTLGSWYHNVQEFTRCWRCHDZNPMLGRVGLZQRDDQQDANCVKCMPWMFAQCKZMYXAZRHIRN\
WPKQXAYVTTYWWASSLPHAWGXKEXXEQWINBPGLMXXGKQCCTEAPAQDHGDKCNMEYTWFBMBHCGMYRTCNPYHX\
BXEQKWZHVDPKYWLFHCQGEEVSYPGDYBABQXXYSAPKPPFZSWXZLGAXVKVLEVVHAYSCQVMMYZNQAGNBDIA\
IWWRNXQSFHEWGLGKVTAKWILRSHPDEXCHPCWVPEIQYNAYTDDLRKZDKBRVMQNIPLFHDRIHHVRRRGLZASZ\
PLSEMMQZXHHNLSTIRTAWKRQBNLNHPLNXSENRMGPEHSVHWPHGRYFDTIZIBLRZNGAAEMZZPVXQSHZIXFB\
ZPSZFDMKSFKVNCGVEVRZFWEEBNKLSWRNPTMKLCLKRXSVGMTPATBSGFMDWWRZLXGCRQFDIHWAMZPVBZY\
RVSCCVDLYLNQLQYNLTTVGBZQTPIAITESAGZLBZNPYTTYYCTVQMWSHDBYBBLWANXICEMWENXCPYSDEFZ\
YPPFQVHMMVHBEALAYKPXHHBIHCMFBFBVHRSRZZBXHBHFFNPQBLRHTAHANBVQAPKVKARFYQGCBHYTAQS\
ACNMWMAMWGFPWXKFAFGAVKVKALLNDVHDZDRMCVLESVIYYMTSYXDBGWXIKFECGVCCCSZRMHKISDNWGSH\
TDLASQMGNWRCFRMSGCWVPYIHMDSREGVXHALWKHHAKHKNFTFKGYXVWCQHNKCZHXRDSEFRYXXKXQNYXRE\
VNFNQBLXXXNCVZYWWREXVYGDXQZGPKHBGAIPYPGAFGLPPGHZANSTNIAAPZFMLSVYNXNTKWLNBYZBZYD\
SDQSMLQZCIXNQMFKLTDWYFWFVIWHLAPDANGIZQQRLXHAQWVNYSFPDVXCXTXVNPNZSCLMKWVRMNFHNPR\
TCBZZYBSQTFRNYMGPSISCNEEADIGZKRAGSVCRSLQTVLSFCKDXKKTXTRYRDGPMAZAPLLFNFCIMSVDWBM\
WWKDYBFEBXVCCGWPIZRYHZZVYLKNHCFAZFFTIENTPQVWFNVPQCPGPPSZGYPMDLERVEQSQSMWEFIMVZK\
FPABYFRXZXKWWSYFBQMZEXRIHYKDCHVABEAHVKVAWKVPRFTTGBFHKSFVPISPSZGYKFWMKZRLNVWRTCT\
VSLMXPCPLNMMNGBPDDMNYFSGWYQNHNYLLHMVCEMKVTNDKGIXVPWLXFQYHRZPPVRLRLXBGQYWQCHXBGF\
WGPESLQTNIZTLXCXEFQRLETXLLELXIHRQVFGAFMGAWFLXMTNAQWIFEFQCGEPVSXEKMKZPELDNKNMXPM\
PEKGGESFHTXPFASKFWDCESGRATYKTDXBWNZZVLQQPTYRQZVZRNYYCCFBELZSFHTGQLADACFPASETBTW\
LNDNARXDEYECGFIKKNWENIZZKKBFYFQIFYQPHYZDPYYBHNXPFECLGYISHWRBDCZXDGWYITNBFEQBHVX\
VZCNHGYGVNISNMTKDSTIBTGCWHDHFHKBNMMVKSCESQZNSNILNHFQACCMTRAXMKQLLBCIYYNDSBEAEHI\
YXVCMCAGZCHVRXVKDMAZPLMSZHPIQIKGCAPTHTGKQGXYSZDGWPFBQFMZAIPMMDVHBAZRRVDGYPRTNAY\
ZGAVZRFQWZQVHYKTXNERFLILMEWETXZAXXCNZDYFXFQDKLKHEPWXHFQDGPXNRNNNHXXQCDNADXSXBPZ\
WPKNKTEGLILALNSWSXXNMZNDMPYQGHHNKNCCETPVERGLQIFRFSNCDZYEGEQGPCEQINIGBCHLZZVKHSF\
ASHWLGFRPZLAGELYILGVLYPZMSIMBSNCZVKCLCQNIRCHKTNKTZKBWGPDMSRLABCRDAITEVFKXIPRFSI\
DARCHTLVYTFYXQSZHFITARQQHSGXXKCWYQFSINKSVFQVCCASXVXTGAEBIQQVBTTGBECWEWBVNBZVTXW\
SANCZZCGGMTXDDBVPVQLKNPAHHGRTERIXVQRZZACWTPACSCGLWCTXLYFYHKRBPLADTYBVTWKSCNTBXD\
GKSIKEDXILRVMQQTESBSRWCDQSSMEKBNWTIHEGFNXRTBBSFYRLVGEDQCARLRFZBAZRANRKGQYWPHBWF\
WQRACRAZQMTFGQQHQYDTMTZZLIAZPRFMCCWERMNXRRMDNPFXZMHZKWBNDV";

static PROTEIN_2: &'static [u8] = b"TTASPMGKVKQQYKEYIFTAKINLYKKAFLLFZDNEXAFDIFF\
BVGLXYYQVTKAXKNZEMKGMEVQGACKDAWCKLWQYFMYGVVFLYVKBQYEMCRGZYAYPXIXPQLLPFYKXNMADRK\
BDFMWDCYYSXWXPBPDDQASCPDTDTBPQVRVQKYARAXYYLWNBPLBVXXBKKENGDCKEWMCYHXNDQINYTYHHI\
TBEPSWNKRQLMICGWISYHVGHBALBFYLBVAREGHNMQGKPTAVIQQVXRFDLKMITIWFEIEIKHXNRKNBDMPXF\
FXGQDBWEDKAAYXYPSDDZQWETDXKPKDLSCCAICGATVKMZEWSQBZYPKVNDCKYMWGXENKLACBIBHSKKZRR\
NEEXAPGHDTAALNDTGDTVRPTDGAFKBGASHBFZLCEHCWSFQIBHKBYWCXRCFTSMFPZVCYDVHAPKDEZCLPW\
TSXYSRVXNVLHIEGQQXHSWRISIWPEPGKFNXWGSHXXKPNBXEBVZFPSDHVELIFBHZKKDCZRYKETMQLDSAD\
KYNHMDHRSXSZRPCSPPLLPTHZYZKRZIXNWCZWDKPDDEIBMDAEQTCWNTEFWEBKHLDDZIAHABVRVNNPIAM\
NBWBGXECMVASMKAFSGEXXGFDTHYSWLCEQWHSYPVMDDGMTDIHZQHIPANEBPRVPMFKDPRIAKTEYBYHIBR\
THQLPEIGNIDKEPMERVMMHXNMPVDPILTIZWWLICQCYRDHEXFQWWHDSNLKCYTLYGVSIVBMMXFGXZLMDQV\
RLDBXFAYPESCCXBKRKHLFIVXWKAIIVHTQBQRBANFZSCDNDDHKZKYFVPRNYBQLHPKDADBVZANESERDWV\
RTVIWLRXLLEVPTBGFLPDHRMHAAKCYITEVTDDPTMRYZHTVFMPYAGRQYRGEWBYSAERYFWHXLCNCRVBVWV\
EZNAQZMICVRHVVTIBSKEFAXGVYXIIYAADLQMSQEFHESELRSPEHCXQWKDIFPAQNCQFVWMAECHCIANQQG\
HXCEXCZANFFSZLKINRRLVMDNVQFLWACSYMZERWKWTYIAIYLRQYPBYCMNIDWNDZPICXXWYCTHGSNMHZF\
PYKAQTYHFKFBPIEALFRXKFQPHNSHYWBDHYETPCNBCVZRREMVYREHYRSQKIVTTZFSWHRWMLXILEYPSGR\
YSWNBLWRTNDEVWIMWFSLLXPTCXQQYKRAVZHLMNWXHCBNBCDEXCIRTXKDIKXFALBKCHZPFRQIFSMBXDI\
LRYHBRSVZPYQLEMFBCWXGVHGGNNICSKAWZQDHBHXAMMVXLDSELLEREDCDFPGMIPPLLGDKBBSCATZLHK\
DNQBZGIECBRARMPZYNWETRCLKTQGSDLLFAIRBKRQFIGQEZRYZWQRWNVHSKQZAQKLTDIGLEMNCKNHFBP\
MEEATNLDCHHKBRKSBQPMXWFERIRYKLPPCCGWIBDSELKWDKAHPSPTLKHPCIDNFZZQWRWZMTTWNPIYKAK\
GIHCXKIISYXXZCPNFEMTQFDDDCLCCQKDNCQYFGLWSXEKSMLBAHEQGYABIBRHNYQNRMWLBZVLMHWKHWG\
FKBVXHDNBAPVASQKCHWZDPWBYPQBDYDPVHQGPFNACQPZFDINEQAVGMNQSMCPRFFQNFBZEFRSZLPQZAH\
LHCTBKYXEGXRWDAXXZBBCARZQKQXVEBNPYPXIQHPECVVNHHHNIKHQGLEMGSPBCBFRPVHZVMLKVYRATD\
LKICMMQQKMBLAQLLRCNQWPGSSAAKZBAMMNVEYYTIRRCPHFTTYWFPVQPLGCPFCXBSFCVXGHXDXLKIHHF\
AQEGXPEABDDFRXGSYRILDGWTCCXZQEYMFAMESDAPLZRRGFKRBSGFMSNWFQRHNIECSWYARRRAPKSIBBC\
CGCVAXXQCZZANPPDSRWPBQMVQFMBHFLMFIACTDCNAKXYPRCVVHYAENRLCWLQDFATBEGWLGIKZEGRPFW\
QGTEPTGMINZAAKHIHXLHZAGTQVMWVAEBZMGCMWVXERYHPGPGRKHCFSPLHNNQGQEFHHITNWZXRDYELFX\
PWIWPKQKQMWSMTZNVCLWPINSMXNHFFATAGWYLRKRBHAZIGPEKSQCKFVBWBIMTVATDEEYTYADZNBELTA\
PQKBZXVRHNBQSVAQBBCZRDQHLXYEKVMLPEFYLNQAFCCSEKZSEKPIZRSYWPMLRLNZSVQXFASGKGFSTZG\
WDQWEPXBMSCGDNTWZZIPPESWFQZEYGNZQSBXKMEYNCSSPDZSSSWTPQFAAXVQAFLXDTDPKKNQREAFZLC\
BFRHXEFAPHQKAZASWFWHBWBNTLKKWATZCZBWWKDMDIMSZTTAYPLPCZXAYYEPKLYZKKLGWIGQHTPBFQE\
DPHMKDWQLBFICDTXIQLVNZAFMHEMEXFBSSAIFIGVRDPFYDTQRGVEZLVKSVHSPCELVTNFQQMBCZDRLXG\
LRTLCWHSDWGMYNHZAIIFRNEKNGXHYSEPWVXHDKQSMBQSNYMMQGDQDSECYEVRMHVDSRVNSTEREFHKADD\
CLCVLWCYANTKEEMFREIYBVQKEQMZYDLTNRYBTYYXHYFPXGETFGIMLKYPEIQESDKHQVNQLGKXVZRTFXB\
VVIYLAYBXWGBAHDYAMBDMTSPDNHFZDANCFDSCAQSDPASRHVNPKWWSLVMIQYFCZCGWSXIEWFTFMYQWBE\
MENVFZFTVGRHBYVKTCNMSFMMYCFAGWGGQWLBIHYAMVBFVAWZWRECZDMWKCLQAEGRBZEKBEKYBAZLWYD\
IQWVEBKFAAYANKEQQZRCSXXYHEIZSISPEDVKTSGMFDELWADIYTBQBLDETIEGMWAFLMWXZRYMIPNQSYP\
RDRAZXFHQBPGECNRDSHISZWYMVWHZZRAKCZYNKXIQVBWSPMXQIPPRIYPQVHXHESHGRRLFVTLSBGPZPP\
XMRYXRHBVNWTQGSXKHSAPHEWXWGVTFGKWMMMBNQXIWGSCFCERANEPQVPMPIQITAXWCAQHCWVGTRZIQH\
MKARMLTLGSNYHVQCTTRCHDZNTSPMLBRVBGMLZQZRDDQQDANCVCMBWMFTAMQCZKAMYAZRHPNFRKYWPQX\
ALYVLTMYWWASLPAWKEXERPYNBXMXXMSGKQCATEAQDHEGDZSCNBRTEEYTWBMBCGYCNPYDHXBVCXEMQHK\
WZHMPDPKYWLFQGEEVSYPSGDYBABWXGXYSLKPPFZSWAXZSAXXVKVQLCVVHAYSWFFCQHVMVZZNCANBRDI\
AWDBWWRXQWHEWGLVTAKQILLMYRSHPFEXGCHTASHCCWVYPIHIYYAYTDEDLRAKZDVCEBRMTHNWPLFEIHV\
RRDRGLITSZPLSQERMMQNCXHHVNLATILRMWLMKQBNTLNHFVNLNXNRMVGPECHWFHFYISLDDIZVBLRGZQA\
AEMZPVXSMHZIFBZPSFDKNFKKVNGVEVSHRZXFEEBNPLSKWFMNPRTMKLCLXSHCMITATSBSGMDBERWRDZL\
XGCPVQFIHYXMPVBTYRXVCISCVLYLKFQLQYKNLTTVGBGEQTDIATICSAGLKZNPYTTEYYCTVQEMSHRTDBE\
WGBLWANXICEMWLNCPYSDEFZYTPPFVHMVHBYQALKXHQHDBIHCMFBFVRRZZBXHBHFFSFXQBLRZAHNANFB\
VAPKVKARFYAWQGCBHYAQSACNPLMWEKAXQPYWKATZKFKALNDVVHQWDZDXPMCTVEVILYYMTSYXDCWXIFE\
ECSHXGCCCPZRXMHKISPEEHHMTNSGDQMGVHNRRCPRMSGCWPDBMPYIHBDCKSREGVXHMALKHHYAKHWBNFT\
NFKYVWCEQHNKTZHXRDSEFRKXQNRXREVLNXFQRAYBXPXIMCVKZWWRZGEXVGDXQZGPWKHBVGAVIYPGATG\
LCPIHZANSENRYAHEGPZMISHVYNXNTYALNHXBYZBYYADSDQBMNLZCIVXNCQMFVKXLXNTDWKYWFVIQHGT\
NAPDANDHFIZWQQRXHAQQAVNYSPFVIVFXTYXVNPNZLSLMEWVVMNFCYHNPTBZZHRSXQTFFRNZYTMPSSCN\
EEADIGZCKTRGGCRSQBNHTVLYSFHKDXPKKKXTREDPMGDSVAAPLLLFNCMAVDWBMWWKDYBZERBCXCGGWQD\
PIZMRZVYLHCFAZFTSIWNTPQWFINBEDPGNPYSZGQPLERSVRVEQMQSZWAEFIPLKHFPABYFRZXKMWSIYBQ\
MZERIHYGKDCHVARBVKLEQPHAVKVGAYVPMRFCVTBGBHLKKDESXFVPIMSPGZGYPFMLZRLNVWQRTRXIXSL\
MXPCPWVLCQNGLXBRDDMBXSGWQLRNHNNLLHMVCEMSGKVTNDKGVPWLXFAQYZRZPPRLRRPLXBLGQWQCHYG\
FHKPSSLQTPITLXCXTNGPQLVAETXLGLEXWIHRQPVFGAAFMAWFLTXMTNAAQWIEFQCGEVSEKMNPEDNKNMF\
ZKGDEIFHTXPZYSINKFAGWDCDFSKXRTYKFDXBXWNZVLQQSTYRQVZGCRPNYYCCFMYFBLZLSFHTGQADRIP\
ACFPRSETBTTZWLSERXDYYECGFKNWEPIZWKBFYFSIPHVGEDPYYBHNFECHHPGYISLBFCLZXDWYPITDRFE\
VWQBHVYXVKLCNHGGZIMMSAXPTKDEKSTECWLHDHFHBNIKVSESDZNSNLNQAACBRNAXMKLLBCIEYZNBALM\
AEHIMYXCMCACTRXPVKKTMINAPZPLMSRZHPYQIRCAXHTGQGXYHISZADWPFFMBAIPMMDVHXBIAZGRGWDG\
VYPRTNYZHXPGAVZFRFWQWZQGVVHYTNERRLILYEWTXWXHCNZDFSXFQMLKGHRPWXHFDPXQRHNNNHXQCDR\
NAXSXSXBPZYVAWPKKNKTWEGHLDALNSSZXBXMZNDMPYQGHNFKNCETPKVERGMLQQFRFSNCDZYEGGPREQN\
MIGBCHZZVKFASZWLGFRPZDHAIRHBEFWILPGVYVVMWHSMIMBXSNCZVKDLCQNRCKTDNPZKBAHWCWGPDSR\
LACFRDSTSEFKZXEDPRSIDRHIILNVYTFYBXAQSHFARQGGHSNGKPXKWQFSINSVFTCCLASRXMKKGAMEBSQ\
QVBTCBNECWBMVNNBZETXSANZCXCGGGDVTTTITXDDVPIQLKNAHDGRIDERIZXVSQRZZACWBTACSCGLWCT\
TXLYCLYYHKRVPFBGLADNTYBQVWSCCNTDHXDYGVSIAKTELDXIXVVMQQTEABNRWDQSSMVEBNWTIEGFTGY\
NXRBBGVFYVRRLLVEWQCERFHRFBAGZANEKGQWPWFWRPVXCRAZMTCIFQHQDTTZGIQEVAPRFMCDWEMNEXR\
DMWNPFNZMHKWBTDVMLNSSZXBXMZNDMPYQGHNFKNCETPKVERGMKWQFSINSV";

#[bench]
fn bench_aligner_wc_local_dna(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = Aligner::with_capacity(DNA_1.len(), DNA_2.len(), -5, -1, &score);
    b.iter(|| aligner.local(DNA_1, DNA_2));
}

#[bench]
fn bench_aligner_wc_global_dna(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = Aligner::with_capacity(DNA_1.len(), DNA_2.len(), -5, -1, &score);
    b.iter(|| aligner.global(DNA_1, DNA_2));
}

#[bench]
fn bench_aligner_wc_semiglobal_dna(b: &mut Bencher) {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = Aligner::with_capacity(DNA_1.len(), DNA_2.len(), -5, -1, &score);
    b.iter(|| aligner.semiglobal(DNA_1, DNA_2));
}

#[bench]
fn bench_aligner_wc_global_protein_blosum62(b: &mut Bencher) {
    let score = &blosum62;
    let mut aligner = Aligner::with_capacity(PROTEIN_1.len(), PROTEIN_2.len(), -5, -1, score);
    b.iter(|| aligner.global(PROTEIN_1, PROTEIN_2));
}

#[bench]
fn bench_aligner_wc_semiglobal_protein_blosum62(b: &mut Bencher) {
    let score = &blosum62;
    let mut aligner = Aligner::with_capacity(PROTEIN_1.len(), PROTEIN_2.len(), -5, -1, score);
    b.iter(|| aligner.semiglobal(PROTEIN_1, PROTEIN_2));
}

#[bench]
fn bench_aligner_wc_local_protein_blosum62(b: &mut Bencher) {
    let score = &blosum62;
    let mut aligner = Aligner::with_capacity(PROTEIN_1.len(), PROTEIN_2.len(), -5, -1, score);
    b.iter(|| aligner.local(PROTEIN_1, PROTEIN_2));
}
