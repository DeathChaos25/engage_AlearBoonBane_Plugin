

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoonBaneType 
{
    Hp = 0,
    Str,
    Mag,
    Skl,
    Spd,
    Lck,
    Def,
    Res,
}

pub struct PersonDataStats 
{
    pub bases: [i8; 9],
    pub caps: [i8; 9],
    pub growths: [u8; 9],
}

pub fn return_as_boon_bane_type(value: i32) -> BoonBaneType 
{
    match value 
    {
        0 => BoonBaneType::Hp,
        1 => BoonBaneType::Str,
        2 => BoonBaneType::Mag,
        3 => BoonBaneType::Skl,
        4 => BoonBaneType::Spd,
        5 => BoonBaneType::Lck,
        6 => BoonBaneType::Def,
        7 => BoonBaneType::Res,
        _ => BoonBaneType::Hp,
    }
}

// BOONS - Index 0-7 by BoonBaneType
static mut BOON_STATS: [PersonDataStats; 8] = [
    // Index 0 - HP Boon
    PersonDataStats {
        bases: [3, 0, 0, 0, 0, 0, 0, 0, 0],
        caps: [0, 1, 1, 0, 0, 2, 2, 2, 0],
        growths: [15, 0, 0, 0, 0, 0, 5, 5, 0],
    },
    // Index 1 - STR Boon
    PersonDataStats {
        bases: [0, 2, 0, 0, 0, 0, 0, 0, 0],
        caps: [0, 4, 0, 2, 0, 0, 2, 0, 0],
        growths: [0, 15, 0, 5, 0, 0, 5, 0, 0],
    },
    // Index 2 - MAG Boon
    PersonDataStats {
        bases: [0, 0, 3, 0, 0, 0, 0, 0, 0],
        caps: [0, 0, 4, 0, 2, 0, 0, 2, 0],
        growths: [0, 0, 20, 0, 5, 0, 0, 5, 0],
    },
    // Index 3 - SKL Boon
    PersonDataStats {
        bases: [0, 0, 0, 3, 0, 0, 0, 0, 0],
        caps: [0, 2, 0, 4, 0, 0, 2, 0, 0],
        growths: [0, 5, 0, 25, 0, 0, 5, 0, 0],
    },
    // Index 4 - SPD Boon
    PersonDataStats {
        bases: [0, 0, 0, 0, 2, 0, 0, 0, 0],
        caps: [0, 0, 0, 2, 4, 2, 0, 0, 0],
        growths: [0, 0, 0, 5, 15, 5, 0, 0, 0],
    },
    // Index 5 - LCK Boon
    PersonDataStats {
        bases: [0, 0, 0, 0, 0, 3, 0, 0, 1],
        caps: [0, 2, 2, 0, 0, 4, 0, 0, 2],
        growths: [0, 5, 5, 0, 0, 25, 0, 0, 15],
    },
    // Index 6 - DEF Boon
    PersonDataStats {
        bases: [0, 0, 0, 0, 0, 0, 1, 0, 0],        
        caps: [0, 0, 0, 0, 0, 2, 4, 2, 0],
        growths: [0, 0, 0, 0, 0, 5, 10, 5, 0],
    },
    // Index 7 - RES Boon
    PersonDataStats {
        bases: [0, 0, 0, 0, 0, 0, 0, 1, 0],
        caps: [0, 0, 2, 0, 2, 0, 0, 4, 0],
        growths: [0, 0, 5, 0, 5, 0, 0, 10, 0],
    },
];

// BANES - Index 0-7 by BoonBaneType
static mut BANE_STATS: [PersonDataStats; 8] = [
    // Index 0 - HP Bane
    PersonDataStats {
        bases: [2, 0, 0, 0, 0, 0, 0, 0, 0],
        caps: [0, 1, 1, 0, 0, 1, 1, 1, 0],
        growths: [10, 0, 0, 0, 0, 0, 5, 5, 0],
    },
    // Index 1 - STR Bane
    PersonDataStats {
        bases: [0, 1, 0, 0, 0, 0, 0, 0, 0],
        caps: [0, 3, 0, 1, 0, 0, 1, 0, 0],
        growths: [0, 10, 0, 5, 0, 0, 5, 0, 0],
    },
    // Index 2 - MAG Bane
    PersonDataStats {
        bases: [0, 0, 2, 0, 0, 0, 0, 0, 0],
        caps: [0, 0, 3, 0, 1, 0, 0, 1, 0],
        growths: [0, 0, 15, 0, 5, 0, 0, 5, 0],
    },
    // Index 3 - SKL Bane
    PersonDataStats {
        bases: [0, 0, 0, 2, 0, 0, 0, 0, 0],
        caps: [0, 1, 0, 3, 0, 0, 1, 0, 0],
        growths: [0, 5, 0, 20, 0, 0, 5, 0, 0],
    },
    // Index 4 - SPD Bane
    PersonDataStats {
        bases: [0, 0, 0, 0, 1, 0, 0, 0, 0],
        caps: [0, 0, 0, 1, 3, 1, 0, 0, 0],
        growths: [0, 0, 0, 5, 10, 5, 0, 0, 0],
    },
    // Index 5 - LCK Bane
    PersonDataStats {
        bases: [0, 0, 0, 0, 0, 2, 0, 0, 0],
        caps: [0, 1, 1, 0, 0, 3, 0, 0, 1],
        growths: [0, 5, 5, 0, 0, 20, 0, 0, 3],
    },
    // Index 6 - DEF Bane
    PersonDataStats {
        bases: [0, 0, 0, 0, 0, 0, 1, 0, 0],
        caps: [0, 0, 0, 0, 0, 1, 3, 1, 0],
        growths: [0, 0, 0, 0, 0, 5, 10, 5, 0],
    },
    // Index 7 - RES Bane
    PersonDataStats {
        bases: [0, 0, 0, 0, 0, 0, 0, 1, 0],
        caps: [0, 0, 1, 0, 1, 0, 0, 3, 0],
        growths: [0, 0, 5, 0, 5, 0, 0, 10, 0],
    },
];

pub fn get_boon_stats_by_type(stat_type: i32) -> &'static PersonDataStats 
{
    unsafe 
    {
        &BOON_STATS[stat_type as usize]
    }
}

pub fn get_bane_stats_by_type(stat_type: i32) -> &'static PersonDataStats 
{
    unsafe 
    {
        &BANE_STATS[stat_type as usize]
    }
}

pub fn combine_stats(
    boon: &PersonDataStats,
    backup: &PersonDataStats,
    bane: &PersonDataStats,
) -> PersonDataStats
{
    let mut bases = [0i8; 9];
    let mut caps = [0i8; 9];
    let mut growths = [0u8; 9];
    for i in 0..9
    {
        bases[i] = boon.bases[i] + backup.bases[i] - bane.bases[i];
        caps[i] = boon.caps[i] + backup.caps[i] - bane.caps[i];
        growths[i] = boon.growths[i] + backup.growths[i] - bane.growths[i];
    }
    PersonDataStats { bases, caps, growths }
}