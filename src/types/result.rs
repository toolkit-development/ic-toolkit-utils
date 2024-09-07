use candid::{CandidType, Deserialize};

use crate::api_error::ApiError;

pub type CanisterResult<T> = Result<T, ApiError>;

#[derive(CandidType, Deserialize)]
pub enum CanisterCallResult<T> {
    Ok(T),
    Err(ApiError),
}

impl<T> From<CanisterCallResult<T>> for CanisterResult<T> {
    fn from(val: CanisterCallResult<T>) -> Self {
        match val {
            CanisterCallResult::Ok(value) => Ok(value),
            CanisterCallResult::Err(err) => Err(err),
        }
    }
}
