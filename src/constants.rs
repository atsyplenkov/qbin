// Coordinate borders
pub(crate) const MAX_LONGITUDE: f64 = 180.0;
pub(crate) const MIN_LONGITUDE: f64 = -180.0;
pub(crate) const MAX_LATITUDE: f64 = 89.0;
pub(crate) const MIN_LATITUDE: f64 = -89.0;

// Resolution
// https://docs.carto.com/data-and-analysis/analytics-toolbox-for-postgresql/key-concepts/spatial-indexes#quadbin
pub(crate) const MAX_RESOLUTION: u8 = 26;

// Area estimation
pub(crate) const REF_AREA: f64 = 508164597540055.75;
pub(crate) const AREA_FACTORS: &[f64; 12] = &[
    1.0,
    1.003_741_849_761_155,
    1.897_097_273_904_830_4,
    2.711_808_583_954_8,
    3.034_250_040_669_436_4,
    3.123_101_473_513_554,
    3.145_758_804_577_431_6,
    3.151_449_027_223_487,
    3.152_873_167_713_691_4,
    3.153_229_301_352_465_7,
    3.153_318_340_910_941_8,
    3.153_340_601_184_773_6,
];
pub(crate) const AF_LEN: u8 = AREA_FACTORS.len() as u8;

// Quadbin cell
pub(crate) const HEADER: u64 = 0x4000_0000_0000_0000;
pub(crate) const FOOTER: u64 = 0x000F_FFFF_FFFF_FFFF;
pub(crate) const B: &[u64; 6] = &[
    0x5555_5555_5555_5555,
    0x3333_3333_3333_3333,
    0x0F0F_0F0F_0F0F_0F0F,
    0x00FF_00FF_00FF_00FF,
    0x0000_FFFF_0000_FFFF,
    0x0000_0000_FFFF_FFFF,
];
pub(crate) const S: &[u8; 5] = &[1, 2, 4, 8, 16];

use std::sync::OnceLock;

pub(crate) fn deinterleave_table() -> &'static [[u8; 256]; 2] {
    static DEINTERLEAVE_TABLE: OnceLock<[[u8; 256]; 2]> = OnceLock::new();
    DEINTERLEAVE_TABLE.get_or_init(|| {
        let mut even = [0u8; 256];
        let mut odd = [0u8; 256];

        for i in 0..256 {
            let mut v = i as u8;
            let mut even_val = 0u8;
            let mut odd_val = 0u8;

            for j in 0..4 {
                even_val |= ((v & 1) << j) as u8;
                v >>= 1;
                odd_val |= ((v & 1) << j) as u8;
                v >>= 1;
            }

            even[i] = even_val;
            odd[i] = odd_val;
        }

        [even, odd]
    })
}


// Precompute interleaved values for 8-bit chunks
pub(crate) fn interleave_table() -> &'static [u16; 256] {
    static INTERLEAVE_TABLE: OnceLock<[u16; 256]> = OnceLock::new();
    INTERLEAVE_TABLE.get_or_init(|| {
        let mut table = [0u16; 256];
        for i in 0..256 {
            let mut x = i as u16;
            x = (x | (x << 4)) & 0x0F0F;
            x = (x | (x << 2)) & 0x3333;
            x = (x | (x << 1)) & 0x5555;
            table[i] = x;
        }
        table
    })
}