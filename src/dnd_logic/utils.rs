pub fn stat_to_modifier(stat: i32) -> i32 {
    ((stat - 10) as f32 / 2.0).floor() as i32  
}

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_to_modifier() {
        assert_eq!(stat_to_modifier(1), -5);
        assert_eq!(stat_to_modifier(2), -4);
        assert_eq!(stat_to_modifier(3), -4);
        assert_eq!(stat_to_modifier(4), -3);
        assert_eq!(stat_to_modifier(5), -3);
        assert_eq!(stat_to_modifier(6), -2);
        assert_eq!(stat_to_modifier(7), -2);
        assert_eq!(stat_to_modifier(8), -1);
        assert_eq!(stat_to_modifier(9), -1);
        assert_eq!(stat_to_modifier(10), 0);
        assert_eq!(stat_to_modifier(11), 0);
        assert_eq!(stat_to_modifier(12), 1);
        assert_eq!(stat_to_modifier(13), 1);
        assert_eq!(stat_to_modifier(14), 2);
        assert_eq!(stat_to_modifier(15), 2);
        assert_eq!(stat_to_modifier(16), 3);
        assert_eq!(stat_to_modifier(17), 3);
        assert_eq!(stat_to_modifier(18), 4);
        assert_eq!(stat_to_modifier(19), 4);
        assert_eq!(stat_to_modifier(20), 5);
        assert_eq!(stat_to_modifier(21), 5);
        assert_eq!(stat_to_modifier(22), 6);
        assert_eq!(stat_to_modifier(23), 6);
        assert_eq!(stat_to_modifier(24), 7);
        assert_eq!(stat_to_modifier(25), 7);
        assert_eq!(stat_to_modifier(26), 8);
        assert_eq!(stat_to_modifier(27), 8);
        assert_eq!(stat_to_modifier(28), 9);
        assert_eq!(stat_to_modifier(29), 9);
        assert_eq!(stat_to_modifier(30), 10);
    }
}

pub fn proficiency_bonus(level: i32) -> i32 {
    match level {
        1..=4 => 2,
        5..=8 => 3,
        9..=12 => 4,
        13..=16 => 5,
        17..=20 => 6,
        _ => 0,
    }
}

pub fn exp_to_lvl(exp: i32) -> i32 {
    match exp {
        0..=299 => 1,
        300..=899 => 2,
        900..=2699 => 3,
        2700..=6499 => 4,
        6500..=13999 => 5,
        14000..=22999 => 6,
        23000..=33999 => 7,
        34000..=47999 => 8,
        48000..=63999 => 9,
        64000..=84999 => 10,
        85000..=99999 => 11,
        100000..=119999 => 12,
        120000..=139999 => 13,
        140000..=164999 => 14,
        165000..=194999 => 15,
        195000..=224999 => 16,
        225000..=264999 => 17,
        265000..=304999 => 18,
        305000..=354999 => 19,
        355000..=999999 => 20,
        _ => 0,
    }
}

pub fn exp_needed_to_lvl(level: i32) -> i32 {
    match level {
        1 => 0,
        2 => 300,
        3 => 900,
        4 => 2700,
        5 => 6500,
        6 => 14000,
        7 => 23000,
        8 => 34000,
        9 => 48000,
        10 => 64000,
        11 => 85000,
        12 => 100000,
        13 => 120000,
        14 => 140000,
        15 => 165000,
        16 => 195000,
        17 => 225000,
        18 => 265000,
        19 => 305000,
        20 => 355000,
        _ => 0,
    }
}