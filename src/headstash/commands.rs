//! # Commands
//!
//! commands related to powering headstash deployment workflow
use anybuf::Anybuf;
use cosmwasm_std::{CosmosMsg, StdError};

/// Defines the msg to upload the secret_headstash.wasm contract.
pub fn upload_contract_msg(
    sender: ::cosmwasm_std::Addr,
    wasm: &str,
) -> Result<CosmosMsg, StdError> {
    // define headstash wasm binary
    let headstash_bin = match wasm {
        "cw-headstash" => include_bytes!("secret_headstash.wasm").to_vec(),
        "snip120u" => include_bytes!("snip120u.wasm").to_vec(),
        "scrt-headstash-circuitboard" => include_bytes!("headstash_circuitboard.wasm").to_vec(),
        _ => return Err(StdError::generic_err("bad contract upload")),
    };

    Ok(
        #[allow(deprecated)]
        CosmosMsg::Stargate {
            type_url: "/secret.compute.v1beta1.MsgStoreCode".into(),
            value: Anybuf::new()
                .append_string(1, sender.clone()) // sender (DAO)
                .append_bytes(2, &headstash_bin) // updated binary of transfer msg.
                .into_vec()
                .into(),
        },
    )
}
