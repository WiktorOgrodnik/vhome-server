use serde::{Serialize, Deserialize};
use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AuthorizeLevel {
    None,
    Show,
    Edit,
    Manage,
}

#[derive(Debug, Clone, Copy, EnumString, Serialize, Deserialize, PartialEq)]
pub enum Roles {
    Guest,
    Member,
    Admin,
}

impl Roles {
    pub fn has_authority(&self, level: AuthorizeLevel) -> bool {
        match level {
            AuthorizeLevel::None |
            AuthorizeLevel::Show => true,
            AuthorizeLevel::Edit => matches!(self, Roles::Member | Roles::Admin),
            AuthorizeLevel::Manage => matches!(self, Roles::Admin),
        } 
    }
}


