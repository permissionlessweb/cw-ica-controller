use secret_cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("prost encoding error: {0}")]
    ProstEncodeError(#[from] cosmos_sdk_proto::prost::EncodeError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("ica information is not set")]
    IcaInfoNotSet {},

    #[error("unknown reply id: {0}")]
    UnknownReplyId(u64),
}
