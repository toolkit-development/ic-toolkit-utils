use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use std::fmt;

impl fmt::Display for GovernanceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            GovernanceType::None => "governance::none".to_string(),
            GovernanceType::Permission => "governance::permission".to_string(),
            GovernanceType::Proposal(proposal_type) => match &proposal_type {
                ProposalType::Member => "governance::proposal::member".to_string(),
                ProposalType::Token(principal) => {
                    format!("governance::proposal::token::{}", principal)
                }
            },
        };
        write!(f, "{}", s)
    }
}

impl GovernanceType {
    /// Parses a string to create a Governance instance.
    pub fn from_string(input: &str) -> Self {
        let parts: Vec<&str> = input.split("::").collect();
        match parts.as_slice() {
            ["governance", "none"] => GovernanceType::None,
            ["governance", "permission"] => GovernanceType::Permission,
            ["governance", "proposal", "member"] => GovernanceType::Proposal(ProposalType::Member),
            ["governance", "proposal", "token", principal] => {
                if let Ok(parsed_principal) = Principal::from_text(principal) {
                    GovernanceType::Proposal(ProposalType::Token(parsed_principal))
                } else {
                    GovernanceType::None
                }
            }
            _ => GovernanceType::None, // Fallback for unexpected input
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum GovernanceType {
    #[default]
    None,
    Permission,
    Proposal(ProposalType),
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProposalType {
    Member,
    Token(Principal),
}

impl fmt::Display for ProposalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProposalType::Member => write!(f, "member"),
            ProposalType::Token(principal) => write!(f, "token::{}", principal),
        }
    }
}
