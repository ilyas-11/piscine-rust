use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc: (d).to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,   
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let func = self
            .flags
            .get(input)
            .ok_or("Flag not found".to_string())?;

        func(argv[0], argv[1]).map_err(|e| e.to_string())

    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f64>()?;
    let b = b.parse::<f64>()?;

    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = a.parse::<f64>()?;
    let b = b.parse::<f64>()?;

    Ok((a % b).to_string())
}
