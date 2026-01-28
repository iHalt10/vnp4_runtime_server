use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;

use p4runtime::p4::config::v1::match_field::Match as P4RuntimeMatch;
use p4runtime::p4::config::v1::match_field::MatchType as P4RuntimeMatchType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchType {
    Exact,
    Lpm,
    Ternary,
    Range,
}

impl Display for MatchType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = match self {
            MatchType::Exact => "exact",
            MatchType::Lpm => "lpm",
            MatchType::Ternary => "ternary",
            MatchType::Range => "range",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for MatchType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "exact" => Ok(MatchType::Exact),
            "lpm" => Ok(MatchType::Lpm),
            "ternary" => Ok(MatchType::Ternary),
            "range" => Ok(MatchType::Range),
            _ => Err(format!("Invalid match type: {}", s)),
        }
    }
}

impl MatchType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MatchType::Exact => "exact",
            MatchType::Lpm => "lpm",
            MatchType::Ternary => "ternary",
            MatchType::Range => "range",
        }
    }

    pub fn as_match(&self) -> P4RuntimeMatch {
        match self {
            MatchType::Exact => P4RuntimeMatch::MatchType(P4RuntimeMatchType::Exact as i32),
            MatchType::Lpm => P4RuntimeMatch::MatchType(P4RuntimeMatchType::Lpm as i32),
            MatchType::Ternary => P4RuntimeMatch::MatchType(P4RuntimeMatchType::Ternary as i32),
            MatchType::Range => P4RuntimeMatch::MatchType(P4RuntimeMatchType::Range as i32),
        }
    }
}
