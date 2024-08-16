use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::enums::language::Language::{DA, DE, EN, ES, JA, KO, NL};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Language {
    EN,
    DE,
    JA,
    ES,
    DA,
    NL,
    KO,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            EN => "English",
            DE => "Deutsch",
            JA => "日本語",
            ES => "Español",
            DA => "Deens",
            NL => "Nederlands",
            KO => "한국인"
        })
    }
}

impl Language {
    pub fn language_code(&self) -> &str {
        match self {
            EN => "en",
            DE => "de",
            JA => "ja",
            ES => "es",
            DA => "da",
            NL => "nl",
            KO => "ko"
        }
    }
}
#[derive(Debug)]
pub enum LanguageError {
    UnknownLanguage(String)
}

impl Display for LanguageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
               match self {
                   LanguageError::UnknownLanguage(value) => format!("Unknown language, {value}")
               })
    }
}

impl Error for LanguageError {}
impl FromStr for Language {
    type Err = LanguageError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let result = match value.to_lowercase().as_str() {
            "en" => EN,
            "de" => DE,
            "ja" => JA,
            "es" => ES,
            "da" => DA,
            "nl" => NL,
            "ko" => KO,
            _ => Err(LanguageError::UnknownLanguage(value.to_string()))?
        };
        Ok(result)
    }
}
