use std::collections::HashMap;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(String, String) -> Result<String, String>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.short_hand.clone(), flag.long_hand.clone()), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some((_, func)) = self.flags.iter().find(|((s, l), _)| s == input || l == input) {
            match func(argv[0].to_string(), argv[1].to_string()) {
                Ok(res) => Ok(res),
                Err(err) => Err(err),
            }
        } else {
            Err("Flag not found".to_string())
        }
    }
}

pub fn div(a: String, b: String) -> Result<String, String> {
    let a = a.parse::<f64>().map_err(|_| "invalid float literal".to_string())?;
    let b = b.parse::<f64>().map_err(|_| "invalid float literal".to_string())?;
    Ok((a / b).to_string())
}

pub fn rem(a: String, b: String) -> Result<String, String> {
    let a = a.parse::<f64>().map_err(|_| "invalid float literal".to_string())?;
    let b = b.parse::<f64>().map_err(|_| "invalid float literal".to_string())?;
    Ok((a % b).to_string())
}
