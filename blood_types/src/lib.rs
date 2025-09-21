#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen, rh_factor) = s.split_at(1);
        let antigen = antigen.parse().unwrap();
        let rh_factor = rh_factor.parse().unwrap();
        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = match self.antigen { 
            Antigen::A=>"A", 
            Antigen::B=>"B", 
            Antigen::AB=>"AB", 
            Antigen::O=>"O" 
        };
        let r = match self.rh_factor { 
            RhFactor::Positive=>"+", 
            RhFactor::Negative=>"-" 
        };
        write!(f, "{}{}", a, r)
    }
}


impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        use Antigen::*;
        let antigen_ok = matches!(
            (self.antigen.clone(), other.antigen.clone()),
            (O, O) |
            (A, O) | (A, A) |
            (B, O) | (B, B) |
            (AB, _)
        );
        let rh_ok = matches!(self.rh_factor, RhFactor::Positive) || self.rh_factor == RhFactor::Negative && other.rh_factor == RhFactor::Negative;
        antigen_ok && rh_ok
    }
    pub fn donors(&self) -> Vec<Self> {
        Self::all().into_iter().filter(|b| self.can_receive_from(b)).collect()
    }
    pub fn recipients(&self) -> Vec<Self> {
        Self::all().into_iter().filter(|b| b.can_receive_from(self)).collect()
    }
    fn all() -> Vec<Self> {
        use Antigen::*; use RhFactor::*;
        vec![
            (O,Negative),(O,Positive),
            (A,Negative),(A,Positive),
            (B,Negative),(B,Positive),
            (AB,Negative),(AB,Positive),
        ].into_iter().map(|(a,r)| Self{antigen:a,rh_factor:r}).collect()
    }
}
