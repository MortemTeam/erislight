pub fn hex2num(hexa: &str) -> i32 {
    i32::from_str_radix(hexa, 16).unwrap()
}

pub fn GetRedPart(hexa: &str) -> i32 {
    hex2num(&hexa[1..3])
}

pub fn GetGreenPart(hexa: &str) -> i32 {
    hex2num(&hexa[3..5])
}

pub fn GetBluePart(hexa: &str) -> i32 {
    hex2num(&hexa[5..7])
}
