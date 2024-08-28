use cosmwasm_std::{Instantiate2AddressError, StdError};
use thiserror::Error;

use cw_ica_controller::types::ContractError as CwIcaControllerError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("error when computing the instantiate2 address: {0}")]
    Instantiate2AddressError(#[from] Instantiate2AddressError),
    #[error("error : {0}")]
    CwIcaControllerError(#[from] CwIcaControllerError),

    #[error("unauthorized")]
    Unauthorized {},
    #[error("snip token not set")]
    SnipTokenNotSet {},

    #[error("ica information is not set")]
    IcaInfoNotSet {},
    #[error("ica information is not set, headstash")]
    NoIcaInfo {},
    #[error("headstash information is not set")]
    NoHeadstashInfo {},
}
