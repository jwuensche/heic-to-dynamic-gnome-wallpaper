
pub fn to_rem_hours(secs: u16) -> u16 {
    secs / 60 / 60
}

pub fn to_rem_min(secs: u16) -> u16 {
    secs / 60 % 60
}

pub fn to_rem_sec(secs: u16) -> u16 {
    secs % 60
}
