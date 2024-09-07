#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use secret_cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config_w, ContractState, INIT_ID};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-ica-owner";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = if let Some(admin) = msg.owner {
        deps.api.addr_validate(&admin)?
    } else {
        info.sender
    };

    let state = ContractState::new(
        admin,
        msg.ica_controller_code_id,
        msg.ica_controller_code_hash,
        msg.bloom_plaintxt_msg,
        msg.headstash_addr,
    );
    config_w(deps.storage).save(&state)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateIcaContract {
            channel_open_init_options,
        } => execute::create_ica_contract(deps, env, info, channel_open_init_options),
        ExecuteMsg::IbcBloom {
            ica_id,
            to_address,
            eth_pubkey,
            eth_sig,
        } => execute::send_ibc_bloom(deps, info, ica_id, to_address, eth_pubkey, eth_sig),
        ExecuteMsg::ReceiveIcaCallback(callback_msg) => {
            execute::ica_callback_handler(deps, info, callback_msg)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractState {} => to_binary(&query::state(deps)?),
        QueryMsg::GetIcaContractState { ica_id } => to_binary(&query::ica_state(deps, ica_id)?),
        QueryMsg::GetIcaCount {} => to_binary(&query::ica_count(deps)?),
    }
}

mod execute {
    use anybuf::Anybuf;
    use cw_ica_controller::helpers::CwIcaControllerContract;
    use cw_ica_controller::ibc::types::packet::IcaPacketData;
    use cw_ica_controller::types::callbacks::IcaControllerCallbackMsg;
    use cw_ica_controller::types::cosmos_msg::convert_to_proto_any;
    use cw_ica_controller::types::msg::{CallbackInfo, ExecuteMsg as IcaControllerExecuteMsg};
    use cw_ica_controller::types::state::{ChannelState, ChannelStatus};
    use cw_ica_controller::{
        helpers::CwIcaControllerCode, ibc::types::metadata::TxEncoding,
        types::msg::options::ChannelOpenInitOptions,
    };

    use cosmos_sdk_proto::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin};
    use cosmos_sdk_proto::Any;
    use secret_cosmwasm_std::{Addr, CosmosMsg};

    use crate::cosmos_msg::ExampleCosmosMessages;
    use crate::state::{
        self, addr_to_ica_id_r, config_r, ica_count_r, ica_states_r, ica_states_w, BloomSnip120u,
    };

    use super::*;

    pub fn create_ica_contract(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        channel_open_init_options: ChannelOpenInitOptions,
    ) -> Result<Response, ContractError> {
        let state = config_r(deps.storage).load()?;
        if state.admin != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        let ica_code = CwIcaControllerCode::new(
            state.ica_controller_code_id,
            state.ica_controller_code_hash.clone(),
        );

        let instantiate_msg = cw_ica_controller::types::msg::InstantiateMsg {
            owner: Some(env.contract.address.to_string()),
            channel_open_init_options,
            send_callbacks_to: Some(CallbackInfo {
                address: env.contract.address.to_string(),
                code_hash: state.ica_controller_code_hash,
            }),
        };

        let ica_count = ica_count_r(deps.storage).load().unwrap_or(0);

        let label = format!("icacontroller-{}-{}", env.contract.address, ica_count);

        let cosmos_msg: secret_cosmwasm_std::CosmosMsg = ica_code
            .instantiate(
                instantiate_msg,
                label,
                Some(env.contract.address.to_string()),
            )
            .map_err(|e| {
                ContractError::Std(secret_cosmwasm_std::StdError::generic_err(e.to_string()))
            })?;

        let submsg = secret_cosmwasm_std::SubMsg::reply_always(cosmos_msg, 420);

        Ok(secret_cosmwasm_std::Response::new().add_submessage(submsg))
    }

    /// Sends a predefined action to the ICA host.
    pub fn send_ibc_bloom(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        ica_id: u64,
        to_address: String,
        eth_pubkey: String,
        eth_sig: String,
        snip120us: Vec<BloomSnip120u>,
    ) -> Result<Response, ContractError> {
        let state = config_r(deps.storage).load()?;

        // custom checks to broadcast msg, returns the snip20 owner addr, derived from the eth pubkey
        let owner = state.verify_headstash_bloom(
            &deps,
            info.sender.clone(),
            eth_pubkey,
            eth_sig,
            state.bloom_plaintxt_msg.clone(),
            to_address.clone(),
        )?;

        let ica_state = ica_states_r(deps.storage).load(&ica_id.to_le_bytes())?;

        let ica_info = if let Some(ica_info) = ica_state.ica_state {
            ica_info
        } else {
            return Err(ContractError::IcaInfoNotSet {});
        };

        let cw_ica_contract = CwIcaControllerContract::new(
            secret_cosmwasm_std::Addr::unchecked(&ica_state.contract_addr),
            state.ica_controller_code_hash.to_string(),
        );

        for snip in snip120us {
            let contract = env.contract.address;
            let transfer_from = snip20_reference_impl::msg::ExecuteMsg::TransferFrom {
                owner: owner.to_string(),
                recipient: contract.to_string(),
                amount: cosm,
                memo: None,
                decoys: None,
                entropy: None,
                padding: None,
            };
        }

        let ica_packet = match ica_info.tx_encoding {
            TxEncoding::Protobuf => {
                let predefined_proto_message = CosmosMsg::Stargate {
                    type_url: "/secret.compute.v1beta1.MsgExecuteWasm".into(),
                    value: Anybuf::new()
                        .append_string(1, contract.clone())
                        .append_string(2, snip.address)
                        .append_bytes(3, to_binary(&transfer_from)?.to_vec())
                        .into_vec()
                        .into(),
                };

                IcaPacketData::from_proto_anys(
                    vec![convert_to_proto_any(
                        predefined_proto_message,
                        contract.to_string(),
                    )?],
                    None,
                )
            }
            TxEncoding::Proto3Json => {
                let predefined_json_message = ExampleCosmosMessages::MsgExecuteWasm {
                    from_address: ica_info.ica_addr,
                    contract: snip.address.to_string(),
                    amount: cosmwasm_std::coins(100, "stake"),
                    msg: to_binary(&transfer_from)?,
                }
                .to_string();
                IcaPacketData::from_json_strings(&[predefined_json_message], None)
            }
        };

        let ica_controller_msg = IcaControllerExecuteMsg::SendCustomIcaMessages {
            messages: Binary(ica_packet.data),
            packet_memo: ica_packet.memo,
            timeout_seconds: None,
        };

        let msg = cw_ica_contract.call(ica_controller_msg)?;

        Ok(Response::default().add_message(transfer_from))
    }

    /// Handles ICA controller callback messages.
    pub fn ica_callback_handler(
        deps: DepsMut,
        info: MessageInfo,
        callback_msg: IcaControllerCallbackMsg,
    ) -> Result<Response, ContractError> {
        let ica_id = addr_to_ica_id_r(deps.storage).load(info.sender.as_bytes())?;
        let mut ica_state = ica_states_r(deps.storage).load(&ica_id.to_le_bytes())?;

        if let IcaControllerCallbackMsg::OnChannelOpenAckCallback {
            channel,
            ica_address,
            tx_encoding,
        } = callback_msg
        {
            ica_state.ica_state = Some(state::IcaState {
                ica_id,
                channel_state: ChannelState {
                    channel,
                    channel_status: ChannelStatus::Open,
                },
                ica_addr: ica_address,
                tx_encoding,
            });

            ica_states_w(deps.storage).save(&ica_id.to_le_bytes(), &ica_state)?;
        }

        Ok(Response::default())
    }
}

#[entry_point]
#[allow(clippy::pedantic)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INIT_ID => reply::save_ica_contract_init_reply(deps, env, msg.result),
        _ => Err(ContractError::UnknownReplyId(msg.id)),
    }
}
mod reply {
    use secret_cosmwasm_std::{Addr, SubMsgResult};

    use crate::state::{addr_to_ica_ir_w, ica_count_r, ica_count_w, ica_states_w};

    use super::*;

    pub fn save_ica_contract_init_reply(
        deps: DepsMut,
        _env: Env,
        result: SubMsgResult,
    ) -> Result<Response, ContractError> {
        let mut ica_addr = None;

        // todo: grab from reply
        let ica_count = ica_count_r(deps.storage).load().unwrap_or(0);

        match result {
            SubMsgResult::Ok(c) => {
                for event in c.events {
                    if event.ty == "instantiate" {
                        for attr in &event.attributes {
                            if attr.key == "contract_address" {
                                ica_addr = Some(attr.value.clone());
                            }
                        }
                    }
                }

                if let Some(addr) = ica_addr.clone() {
                    let initial_state =
                        crate::state::IcaContractState::new(Addr::unchecked(addr.clone()));
                    ica_states_w(deps.storage).save(&ica_count.to_le_bytes(), &initial_state)?;
                    addr_to_ica_ir_w(deps.storage).save(addr.as_bytes(), &ica_count)?;
                    ica_count_w(deps.storage).save(&(ica_count + 1))?;
                }
            }
            SubMsgResult::Err(_) => (),
        }
        Ok(Response::new())
    }
}

mod query {
    use crate::state::{config_r, ica_count_r, ica_states_r, IcaContractState};

    use super::*;

    /// Returns the saved contract state.
    pub fn state(deps: Deps) -> StdResult<ContractState> {
        config_r(deps.storage).load()
    }

    /// Returns the saved ICA state for the given ICA ID.
    pub fn ica_state(deps: Deps, ica_id: u64) -> StdResult<IcaContractState> {
        ica_states_r(deps.storage).load(&ica_id.to_le_bytes())
    }

    /// Returns the saved ICA count.
    pub fn ica_count(deps: Deps) -> StdResult<u64> {
        ica_count_r(deps.storage).load()
    }
}

pub mod validation {
    use secret_cosmwasm_std::{Addr, DepsMut, StdError};

    use crate::verify::verify_ethereum_text;

    use super::*;

    pub fn validate_claim(
        deps: &DepsMut,
        sender: Addr,
        eth_pubkey: String,
        eth_sig: String,
        claim_plaintxt: String,
        secondary_address: &str,
    ) -> Result<(), StdError> {
        match validate_ethereum_text(
            deps,
            sender,
            &claim_plaintxt,
            eth_sig,
            eth_pubkey,
            secondary_address,
        )? {
            true => Ok(()),
            false => Err(StdError::generic_err("cannot validate eth_sig")),
        }
    }

    pub fn compute_plaintext_msg(
        claim_plaintxt: &String,
        sender: Addr,
        secondary_address: &str,
    ) -> String {
        let mut plaintext_msg = str::replace(&claim_plaintxt, "{wallet}", sender.as_ref());
        plaintext_msg = str::replace(&plaintext_msg, "{secondary_address}", secondary_address);
        plaintext_msg
    }

    pub fn validate_plaintext_msg(plaintext_msg: String) -> Result<(), StdError> {
        if !plaintext_msg.contains("{wallet}") || !plaintext_msg.contains("{secondary_address}") {
            return Err(StdError::generic_err(
                "Plaintext message must contain `{{wallet}}` and `{{secondary_address}}` strings",
            ));
        }
        if plaintext_msg.len() > 1000 {
            return Err(StdError::generic_err("Plaintext message is too long"));
        }
        Ok(())
    }

    pub fn validate_ethereum_text(
        deps: &DepsMut,
        sender: Addr,
        claim_plaintxt: &String,
        eth_sig: String,
        eth_pubkey: String,
        secondary_address: &str,
    ) -> StdResult<bool> {
        let plaintext_msg = compute_plaintext_msg(claim_plaintxt, sender, secondary_address);
        match hex::decode(eth_sig.clone()) {
            Ok(eth_sig_hex) => {
                verify_ethereum_text(deps.as_ref(), &plaintext_msg, &eth_sig_hex, &eth_pubkey)
            }
            Err(_) => Err(StdError::InvalidHex {
                msg: format!("Could not decode {eth_sig}"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {}
