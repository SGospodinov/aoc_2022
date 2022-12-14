use std::fs::read_to_string;

pub mod constants {
    pub const EMPTY_LINE: &str = "\n\n";
    pub const SPACE: &str = " ";
    pub const COMMA: &str = ",";
    pub const HYPHEN: &str = "-";
}

pub fn load_day_input(day: u8) -> String {
    read_to_string(format!("inputs/day_{}", day))
        .expect(format!("Could not find input for day {}", day).as_str())
}
