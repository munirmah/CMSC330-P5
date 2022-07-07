#[derive(Debug, PartialEq)]
pub enum Command {
    Power(bool, i32),    // [Increase/Decrease] power by [number].
    Missiles(bool, i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),        // Turn [On/Off] the shield.
    Try,                 // Try calling pepper.
    Invalid,             // [anything else]
}

/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method

    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str(&self) -> String {
        match self {
            Self::Power(true, n) => format!(
                "{}{}{}",
                String::from("Power increased by "),
                n,
                String::from("%")
            ),
            Self::Power(false, n) => format!(
                "{}{}{}",
                String::from("Power decreased by "),
                n,
                String::from("%")
            ),
            Self::Missiles(true, n) => format!("{}{}", String::from("Missiles increased by "), n),
            Self::Missiles(false, n) => format!("{}{}", String::from("Missiles decreased by "), n),
            Self::Shield(true) => String::from("Shield turned on"),
            Self::Shield(false) => String::from("Shield turned off"),
            Self::Try => String::from("Call attempt failed"),
            Self::Invalid => String::from("Not a command"),
        }
    }
}

/**
    Complete this method that converts a string to a command
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/
pub fn to_command(s: &str) -> Command {
    let mut st = s.split_whitespace();
    match st.next().unwrap() {
        "power" => match st.next().unwrap() {
            "inc" => Command::Power(true, st.next().unwrap().parse::<i32>().unwrap()),
            "dec" => Command::Power(false, st.next().unwrap().parse::<i32>().unwrap()),
            _ => panic!("Error in match"),
        },
        "add" => Command::Missiles(true, st.next().unwrap().parse::<i32>().unwrap()),
        "fire" => Command::Missiles(false, st.next().unwrap().parse::<i32>().unwrap()),
        "shield" => match st.next().unwrap() {
            "on" => Command::Shield(true),
            "off" => Command::Shield(false),
            _ => panic!("Error in match"),
        },
        "try" => Command::Try,
        _ => Command::Invalid,
    }
}
