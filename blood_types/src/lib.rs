use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(Self::O),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "AB" => Ok(Self::AB),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Positive),
            "-" => Ok(Self::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = if s.starts_with("AB") {
            ("AB", &s[2..])
        } else {
            (&s[..1], &s[1..])
        };
        Ok(Self {
            antigen: antigen_str.parse()?,
            rh_factor: rh_str.parse()?,
        })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::O => "O",
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
        };
        let rh = if self.rh_factor == RhFactor::Positive { "+" } else { "-" };
        write!(f, "{}{}", antigen, rh)
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.antigen.clone(), self.rh_factor.clone())
            .cmp(&(other.antigen.clone(), other.rh_factor.clone()))
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        use Antigen::*;
        let antigen_ok = matches!(
            (&self.antigen, &other.antigen),
            (O, O)
                | (A, O) | (A, A)
                | (B, O) | (B, B)
                | (AB, _)
        );
        let rh_ok = self.rh_factor == RhFactor::Positive
            || (self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Negative);
        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        Self::all().into_iter().filter(|b| self.can_receive_from(b)).collect()
    }

    pub fn recipients(&self) -> Vec<Self> {
        Self::all().into_iter().filter(|b| b.can_receive_from(self)).collect()
    }

    fn all() -> Vec<Self> {
        use Antigen::*;
        use RhFactor::*;
        [O, A, B, AB]
            .iter()
            .flat_map(|a| [Negative, Positive].iter().map(move |r| Self {
                antigen: a.clone(),
                rh_factor: r.clone(),
            }))
            .collect()
    }
}
