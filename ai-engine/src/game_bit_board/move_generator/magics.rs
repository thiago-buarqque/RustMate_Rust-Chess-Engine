use std::collections::HashMap;

use rand::Rng;

use crate::game_bit_board::{enums::PieceType, utils::utils::estimate_memory_usage_in_bytes};

pub const ROOK_MAGICS: [u64; 64] = [
    1188968168698150945,
    2359903802296901633,
    108103985927323658,
    11709376623483715588,
    144119594712834064,
    2377909678521974786,
    288231553845202952,
    4755804513766703140,
    1189091039659442178,
    2882444636448620674,
    1159817710264131588,
    9224075793020747904,
    2306124501655488514,
    9262215602163023944,
    583779660044174337,
    72620561181771842,
    9331463100838182976,
    4908925243642351744,
    5768619641500925968,
    5188288058109202433,
    9373259111287760898,
    288793878008692738,
    612493947385825424,
    5190699837389046020,
    10484450312000143392,
    432380767931204224,
    2666166165923954816,
    578721494243344400,
    793196527322399760,
    4621256176226010128,
    4611969984493596680,
    189152292451139748,
    2594143763261554825,
    144185694263185416,
    6917810640107094032,
    144255994359187457,
    4904983082370601032,
    2307532170492661764,
    9223373171867004946,
    16285593841954717988,
    126101339863285770,
    4611756524878970883,
    288793876936917008,
    155392054778789896,
    182397983998885952,
    585470150648561792,
    13835060288942243841,
    72128239824666628,
    10376363912353296512,
    72127963320025152,
    513832707442082944,
    2625616175077294208,
    1301681046978822784,
    216313536782139520,
    10522238325476098176,
    1159817645076251008,
    4612322646398607874,
    14123306577672814593,
    578194686972280913,
    207447195409190913,
    324822814749099010,
    1153765946717372929,
    9799835022811791492,
    1226109407608447234,
];
pub const ROOK_SHIFTS: [u64; 64] = [
    52, 53, 53, 53, 53, 53, 53, 52, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52,
];

pub const BISHOP_MAGICS: [u64; 64] = [
    1443412546467005456,
    2603089383137771522,
    9224506735220278304,
    3474538176891715585,
    11601571982684463114,
    4613111020465459344,
    433058082260911105,
    9511639830956871712,
    180430150444384772,
    288553804507714368,
    400838044997005440,
    144405597662478464,
    2614343999485411363,
    2449958816033820688,
    9223939668594133024,
    144120137086291992,
    2386925429187444880,
    2310443401567043968,
    5585589446570541065,
    1592023035248904212,
    193799936702152736,
    9223512778648584456,
    2324033333887524872,
    576495992526934080,
    4847034335964430608,
    614776533994768915,
    144679238086688896,
    2314868901374083204,
    11529498720135578640,
    378584953958465664,
    2882871384403628096,
    4918001728785254401,
    288548204018926651,
    2344202873963448840,
    288797870483310593,
    649646447487615104,
    162130136878678048,
    2317111959721873473,
    1229487167722553472,
    4622948403009167873,
    4648287248754804737,
    72172029415003176,
    216454532540928002,
    81652757408252417,
    9224506737153803268,
    585472367122188304,
    9297683664459023424,
    166633740712280336,
    9229080735662475266,
    2882339530072985601,
    2305985946833158184,
    4647785459614236688,
    153122456603870336,
    288476821929984640,
    1450515326209949697,
    5189281538956723264,
    5188710012809396737,
    4918098198443069460,
    2306968909724649480,
    216736625429584898,
    1225190754767539202,
    9512728356132487296,
    4613973561235865728,
    145276581597708416,
];
pub const BISHOP_SHIFTS: [u64; 64] = [
    58, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 58,
];

#[inline(always)]
fn is_collision(actual: &[u64], index: usize, attacks: u64) -> bool {
    actual[index] != 0 && actual[index] != attacks
}

fn generate_candidate_magic(rng: &mut rand::rngs::ThreadRng) -> u64 {
    loop {
        let magic = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();
        // Ensure certain bits are set
        if (magic & 0xFF00000000000000) != 0 && (magic & 0x00000000000000FF) != 0 {
            return magic;
        }
    }
}

pub fn find_magics(hash_table: &HashMap<(u8, u64), u64>, relevant_squares: [u64; 64]) {
    let mut rng = rand::thread_rng();

    let mut magics = [0; 64];
    let mut shifts = [0; 64];
    let mut table_size = 0;

    for square in 0..64 {
        let relevant_squares = relevant_squares[square];

        let shift = 64 - relevant_squares.count_ones();

        let mut keys = hash_table
            .keys()
            .filter(|key| key.0 == square as u8)
            .collect::<Vec<&(u8, u64)>>();

        keys.sort();

        'runs: for _ in 0..1_000_000 {
            let magic = generate_candidate_magic(&mut rng);

            if (relevant_squares.wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
                continue;
            }

            let mut actual = vec![0; keys.len()];

            for key in &keys {
                let index = (key.1.wrapping_mul(magic) >> shift) as usize;

                let attacks = hash_table.get(*key).unwrap();

                if is_collision(&actual, index, *attacks) {
                    continue 'runs;
                }

                actual[index] = *attacks;
            }

            magics[square] = magic;
            shifts[square] = shift;

            table_size += estimate_memory_usage_in_bytes::<u64>(keys.len());

            break;
        }
    }

    println!("const MAGIC_NUMBERS: [u64; 64] = {:?};", magics);
    println!("const SHIFTS: [u64; 64] = {:?};", shifts);
    println!("Total lookup table size {:?}kb", table_size / 1024);
}

pub fn fill_magics_lookup_table(
    hash_table: &HashMap<(u8, u64), u64>, lookup_table: &mut [Vec<u64>; 64], magics: [u64; 64],
    shifts: [u64; 64], piece_type: PieceType,
) {
    for square in 0..64 {
        let magic = magics[square];
        let shift = shifts[square];

        let mut keys = hash_table
            .keys()
            .filter(|key| key.0 == (square as u8))
            .collect::<Vec<&(u8, u64)>>();

        keys.sort();

        lookup_table[square].reserve_exact(keys.len());

        lookup_table[square] = vec![0; keys.len()];

        for key in keys {
            let index = (key.1.wrapping_mul(magic) >> shift) as usize;

            if lookup_table[square][index] != 0 {
                panic!(
                    "({} conflict) Index: {} Key: {:?} Value: {} stored: {}",
                    piece_type,
                    index,
                    key,
                    *hash_table.get(key).unwrap(),
                    lookup_table[square][index]
                );
            }

            lookup_table[square][index] = *hash_table.get(key).unwrap();
        }
    }
}
