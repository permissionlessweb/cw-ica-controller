//! # Commands
//!
//! commands related to powering headstash deployment workflow
use anybuf::Anybuf;
use cosmwasm_std::{to_json_binary, CosmosMsg, Empty};

use crate::types::ContractError;

/// Defines the msg to upload the secret_headstash.wasm contract.
pub fn upload_headstash_contract_msg(sender: ::cosmwasm_std::Addr) -> CosmosMsg {
    // define headstash wasm binary
    let headstash_bin = include_bytes!("secret_headstash.wasm");
    // format Stargate CosmWasm MsgUpload

    #[allow(deprecated)]
    CosmosMsg::Stargate {
        type_url: "/secret.compute.v1beta1.MsgStoreCode".into(),
        value: Anybuf::new()
            .append_string(1, sender.clone()) // sender (DAO)
            .append_bytes(2, &headstash_bin) // updated binary of transfer msg.
            .into_vec()
            .into(),
    }
}

/// Defines the msg to instantiate the headstash contract
pub fn instantiate_headstash_contract_msg(
    code_id: u64,
    scrt_headstash_msg: secret_headstash::msg::InstantiateMsg,
) -> Result<CosmosMsg, ContractError> {
    Ok(CosmosMsg::<Empty>::Wasm(
        cosmwasm_std::WasmMsg::Instantiate {
            admin: None,
            code_id,
            label: "Secret Headstash Airdrop Contract: Terp Network".into(),
            msg: to_json_binary(&scrt_headstash_msg)?,
            funds: vec![],
        },
    ))
}
