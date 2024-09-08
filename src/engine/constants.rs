use phf::phf_map;

// pub static CLEAR_RANK: phf::Map<&'static str, &'static u64> = phf_map! {
//     "RANK1" => &18446744073709551360,
//     "RANK2" => &18446744073709486335,
//     "RANK3" => &18446744073692839935,
//     "RANK4" => &18446744069431361535,
//     "RANK5" => &18446742978492891135,
//     "RANK6" => &18446463698244468735,
//     "RANK7" => &18374967954648334335,
//     "RANK8" => &72057594037927935
// };

pub static MASK_RANK: phf::Map<&'static str, &'static u64> = phf_map! {
    "RANK1" => &255,
    "RANK2" => &65280,
    "RANK3" => &16711680,
    "RANK4" => &4278190080,
    "RANK5" => &1095216660480,
    "RANK6" => &280375465082880,
    "RANK7" => &71776119061217280,
    "RANK8" => &18374686479671623680,
};

pub static CLEAR_FILE: phf::Map<&'static str, &'static u64> = phf_map! {
    "FILE1" => &9187201950435737471,
    "FILE2" => &13816973012072644543,
    "FILE3" => &16131858542891098079,
    "FILE4" => &17289301308300324847,
    "FILE5" => &17868022691004938231,
    "FILE6" => &18157383382357244923,
    "FILE7" => &18302063728033398269,
    "FILE8" => &18374403900871474942,
};

// pub static MASK_FILE: phf::Map<&'static str, &'static u64> = phf_map! {
//     "FILE1" => &9259542123273814144,
//     "FILE2" => &4629771061636907072,
//     "FILE3" => &2314885530818453536,
//     "FILE4" => &1157442765409226768,
//     "FILE5" => &578721382704613384,
//     "FILE6" => &289360691352306692,
//     "FILE7" => &144680345676153346,
//     "FILE8" => &72340172838076673
// };
