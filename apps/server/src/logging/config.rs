#![allow(non_upper_case_globals)]

pub struct Colors;
pub struct ForegroundColors;
pub struct BackgroundColors;

impl Colors {
    pub const Reset: &str = "\x1b[0m";
    pub const Bold: &str = "\x1b[1m";
    pub const Faint: &str = "\x1b[2m";
    pub const Italic: &str = "\x1b[3m";
    pub const Underline: &str = "\x1b[4m";
}

impl ForegroundColors {
    pub const Black: &str = "\x1b[30m";
    pub const Red: &str = "\x1b[31m";
    pub const Green: &str = "\x1b[32m";
    pub const Yellow: &str = "\x1b[33m";
    pub const Blue: &str = "\x1b[34m";
    pub const Magenta: &str = "\x1b[35m";
    pub const Cyan: &str = "\x1b[36m";
    pub const Light_Gray: &str = "\x1b[37m";
    pub const Gray: &str = "\x1b[90m";
    pub const Light_Red: &str = "\x1b[91m";
    pub const Light_Green: &str = "\x1b[92m";
    pub const Light_Yellow: &str = "\x1b[93m";
    pub const Light_Blue: &str = "\x1b[94m";
    pub const Light_Magenta: &str = "\x1b[95m";
    pub const Light_Cyan: &str = "\x1b[96m";
    pub const White: &str = "\x1b[97m";
}

impl BackgroundColors {
    pub const Black: &str = "\x1b[40m";
    pub const Red: &str = "\x1b[41m";
    pub const Green: &str = "\x1b[42m";
    pub const Yellow: &str = "\x1b[43m";
    pub const Blue: &str = "\x1b[44m";
    pub const Magenta: &str = "\x1b[45m";
    pub const Cyan: &str = "\x1b[46m";
    pub const Light_Gray: &str = "\x1b[47m";
    pub const Gray: &str = "\x1b[100m";
    pub const Light_Red: &str = "\x1b[101m";
    pub const Light_Green: &str = "\x1b[102m";
    pub const Light_Yellow: &str = "\x1b[103m";
    pub const Light_Blue: &str = "\x1b[104m";
    pub const Light_Magenta: &str = "\x1b[105m";
    pub const Light_Cyan: &str = "\x1b[106m";
    pub const White: &str = "\x1b[107m";
}
