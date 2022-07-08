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
                *n,
                String::from("%")
            ),
            Self::Power(false, n) => format!(
                "{}{}{}",
                String::from("Power decreased by "),
                *n,
                String::from("%")
            ),
            Self::Missiles(true, n) => format!("{}{}", String::from("Missiles increased by "), *n),
            Self::Missiles(false, n) => format!("{}{}", String::from("Missiles decreased by "), *n),
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
            "inc" => match st.next() {
                Some(s) => match s.parse::<i32>() {
                    Ok(n) => match st.next() {
                        _s => match st.next() {
                            None => Command::Power(true, n),
                            _ => Command::Invalid,
                        },
                    },
                    _ => Command::Invalid,
                },
                _ => Command::Invalid,
            },
            "dec" => match st.next() {
                Some(s) => match s.parse::<i32>() {
                    Ok(n) => match st.next() {
                        _s => match st.next() {
                            None => Command::Power(false, n),
                            _ => Command::Invalid,
                        },
                    },
                    _ => Command::Invalid,
                },
                _ => Command::Invalid,
            },
            _ => Command::Invalid,
        },
        "add" => match st.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(n) => match st.next() {
                    _s => match st.next() {
                        None => Command::Missiles(true, n),
                        _ => Command::Invalid,
                    },
                },
                _ => Command::Invalid,
            },
            _ => Command::Invalid,
        },
        "fire" => match st.next() {
            Some(s) => match s.parse::<i32>() {
                Ok(n) => match st.next() {
                    _s => match st.next() {
                        None => Command::Missiles(false, n),
                        _ => Command::Invalid,
                    },
                },
                _ => Command::Invalid,
            },
            None => Command::Invalid,
        },
        "shield" => match st.next().unwrap() {
            "on" => match st.next() {
                None => Command::Shield(true),
                _ => Command::Invalid,
            },
            "off" => match st.next() {
                None => Command::Shield(false),
                _ => Command::Invalid,
            },
            _ => Command::Invalid,
        },
        "try" => match st.next().unwrap() {
            "calling" => match st.next().unwrap() {
                "Miss" => match st.next().unwrap() {
                    "Potts" => match st.next() {
                        None => Command::Try,
                        _ => Command::Invalid,
                    },
                    _ => Command::Invalid,
                },
                _ => Command::Invalid,
            },
            _ => Command::Invalid,
        },
        _ => Command::Invalid,
    }
}
