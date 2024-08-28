#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
// use cw2::set_contract_version;
use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{self, ContractState, CONTRACT_ADDR_TO_ICA_ID, ICA_COUNT, ICA_STATES, STATE},
};
use cw_ica_controller::{
    headstash::commands::*,
    helpers::{CwIcaControllerCode, CwIcaControllerContract},
    types::{
        callbacks::IcaControllerCallbackMsg,
        msg::{options::ChannelOpenInitOptions, ExecuteMsg as IcaControllerExecuteMsg},
        state::headstash::HeadstashTokenParams,
        state::{headstash::HeadstashParams, ChannelState, ChannelStatus},
    },
};
use headstash_cosmwasm_std::Uint128;

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
    let admin = if let Some(admin) = msg.admin {
        deps.api.addr_validate(&admin)?
    } else {
        info.sender
    };

    STATE.save(
        deps.storage,
        &ContractState::new(admin, msg.ica_controller_code_id),
    )?;
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
            salt,
            channel_open_init_options,
            headstash_params,
        } => execute::create_ica_contract(
            deps,
            env,
            info,
            salt,
            channel_open_init_options,
            headstash_params,
        ),
        ExecuteMsg::UploadHeadstash { ica_id } => {
            execute::ica_upload_headstash_on_secret(deps, info, ica_id)
        }
        ExecuteMsg::ReceiveIcaCallback(callback_msg) => {
            ica::ica_callback_handler(deps, info, callback_msg)
        }
        ExecuteMsg::InstantiateHeadstash {
            ica_id,
            start_date,
            total,
        } => {
            execute::ica_instantiate_headstash_contract(deps, env, info, ica_id, start_date, total)
        }
        ExecuteMsg::InstantiateTerpNetworkSnip25 { ica_id, tokens } => {
            execute::ica_instantiate_terp_network_snip25s(deps, info, ica_id, tokens)
        }
        ExecuteMsg::AuthorizeMinter { ica_id } => {
            execute::ica_authorize_snip25_minter(deps, info, ica_id)
        }
        ExecuteMsg::IBCTransferTokens { ica_id, channel_id } => {
            execute::ibc_transfer_to_snip_contracts(deps, env, info, ica_id, channel_id)
        }
        ExecuteMsg::AddHeadstashClaimers { ica_id, to_add } => {
            execute::ica_add_headstash_claimers(deps, info, ica_id, to_add)
        }
        ExecuteMsg::AuthorizeFeegrant { ica_id, to_grant } => {
            execute::ica_authorize_feegrant(deps, info, ica_id, to_grant)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractState {} => to_json_binary(&query::state(deps)?),
        QueryMsg::GetIcaContractState { ica_id } => {
            to_json_binary(&query::ica_state(deps, ica_id)?)
        }
        QueryMsg::GetIcaCount {} => to_json_binary(&query::ica_count(deps)?),
    }
}

mod execute {

    use secret_headstash::state::Headstash;
    use state::SNIP25_CONTRACTS;

    use super::*;

    pub fn create_ica_contract(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        salt: Option<String>,
        channel_open_init_options: ChannelOpenInitOptions,
        headstash_params: HeadstashParams,
    ) -> Result<Response, ContractError> {
        let state = STATE.load(deps.storage)?;
        if state.admin != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        let ica_code = CwIcaControllerCode::new(state.ica_controller_code_id);

        let instantiate_msg = cw_ica_controller::types::msg::InstantiateMsg {
            owner: Some(env.contract.address.to_string()),
            channel_open_init_options,
            send_callbacks_to: Some(env.contract.address.to_string()),
            headstash_params,
        };

        let ica_count = ICA_COUNT.load(deps.storage).unwrap_or(0);

        let salt = salt.unwrap_or(env.block.time.seconds().to_string());
        let label = format!("icacontroller-{}-{}", env.contract.address, ica_count);

        let (cosmos_msg, contract_addr) = ica_code.instantiate2(
            deps.api,
            &deps.querier,
            &env,
            instantiate_msg,
            label,
            Some(env.contract.address.to_string()),
            salt,
        )?;

        let initial_state = state::IcaContractState::new(contract_addr.clone());

        ICA_STATES.save(deps.storage, ica_count, &initial_state)?;

        CONTRACT_ADDR_TO_ICA_ID.save(deps.storage, contract_addr, &ica_count)?;

        ICA_COUNT.save(deps.storage, &(ica_count + 1))?;

        Ok(Response::new().add_message(cosmos_msg))
    }

    pub fn ica_upload_headstash_on_secret(
        deps: DepsMut,
        info: MessageInfo,
        ica_id: u64,
    ) -> Result<Response, ContractError> {
        let cw_ica_contract =
            helpers::retrieve_ica_owner_account(deps.as_ref(), info.sender.clone(), ica_id)?;

        let upload_msg = upload_headstash_contract_msg(info.sender);
        let msg = helpers::send_msg_as_ica(vec![upload_msg], cw_ica_contract);

        // TODO: save code_id of headstash on Secret Network to state on ica-callback.
        Ok(Response::default().add_message(msg))
    }

    pub fn ica_instantiate_headstash_contract(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        ica_id: u64,
        start_date: u64,
        total_amount: Uint128,
    ) -> Result<Response, ContractError> {
        let mut msgs = vec![];
        let ica_state = ICA_STATES.load(deps.storage, ica_id)?;
        let cw_ica_contract = helpers::retrieve_ica_owner_account(
            deps.as_ref(),
            info.sender.clone(),
            ica_id.clone(),
        )?;

        if let Some(hs) = ica_state.headstash_params {
            if let Some(ica) = ica_state.ica_state {
                let init_headstash_msg = instantiate_headstash_contract_msg(
                    hs.headstash_code_id,
                    secret_headstash::msg::InstantiateMsg {
                        claim_msg_plaintext: "{wallet}".into(),
                        end_date: Some(env.block.time.plus_days(365u64).nanos()),
                        snip20_1: headstash_cosmwasm_std::ContractInfo {
                            address: headstash_cosmwasm_std::Addr::unchecked(
                                hs.token_params[0].native.clone(),
                            ),
                            code_hash: hs.snip25_code_hash.clone(),
                        },
                        snip20_2: Some(headstash_cosmwasm_std::ContractInfo {
                            address: headstash_cosmwasm_std::Addr::unchecked(
                                hs.token_params[1].native.clone(),
                            ),
                            code_hash: hs.snip25_code_hash.clone(),
                        }),
                        start_date: Some(start_date),
                        total_amount,
                        viewing_key: "eretskeretjablret".into(),
                        admin: headstash_cosmwasm_std::Addr::unchecked(ica.ica_addr),
                    },
                )?;
                let msg = helpers::send_msg_as_ica(vec![init_headstash_msg], cw_ica_contract);
                msgs.push(msg)
            } else {
                return Err(ContractError::NoIcaInfo {});
            }
        } else {
            return Err(ContractError::NoHeadstashInfo {});
        }
        // TODO: save headstash address created to state on message callback
        Ok(Response::new().add_messages(msgs))
    }

    /// Creates a snip25 msg for ica-controller to send.
    pub fn ica_instantiate_terp_network_snip25s(
        deps: DepsMut,
        info: MessageInfo,
        ica_id: u64,
        tokens: Vec<HeadstashTokenParams>,
    ) -> Result<Response, ContractError> {
        let mut msgs = vec![];
        let cw_ica_contract =
            helpers::retrieve_ica_owner_account(deps.as_ref(), info.sender.clone(), ica_id)?;

        let state = ICA_STATES.load(deps.storage, ica_id)?;
        let hp = state.headstash_params.unwrap();

        // we expect first coin in map to be scrtTERP,
        // second coin to be scrtTHIOL.
        for token in tokens.clone() {
            if let Some(t) = hp.token_params.iter().find(|t| t.native == token.native) {
                let msg = self::ica::form_instantiate_snip25(
                    cw_ica_contract.addr().to_string(),
                    token,
                    hp.snip25_code_hash.clone(),
                    hp.snip25_code_id,
                    hp.headstash.clone(),
                    t.symbol.clone(),
                )?;
                msgs.push(msg);
            }
        }
        // TODO: save SNIP25 addrs created to state on callback.
        Ok(Response::new().add_messages(msgs))
    }

    pub fn ica_authorize_snip25_minter(
        deps: DepsMut,
        info: MessageInfo,
        ica_id: u64,
    ) -> Result<Response, ContractError> {
        let mut msgs = vec![];
        let cw_ica_contract =
            helpers::retrieve_ica_owner_account(deps.as_ref(), info.sender.clone(), ica_id)?;

        let state = ICA_STATES.load(deps.storage, ica_id)?;
        let hp = state.headstash_params.unwrap();

        if let Some(ica) = state.ica_state.clone() {
            let hs_addr = hp.headstash.unwrap();
            // load snip25's from state
            let snip25 = SNIP25_CONTRACTS.load(deps.storage)?;
            for snip in snip25 {
                // add minter msg
                let msg = ica::form_authorize_minter(
                    ica.ica_addr.clone(),
                    hs_addr.clone(),
                    snip.to_string(),
                )?;
                msgs.push(msg);
            }
        }
        // push msgs for ica to run
        let ica_msg = helpers::send_msg_as_ica(msgs, cw_ica_contract);
        Ok(Response::new().add_message(ica_msg))
    }

    pub fn ibc_transfer_to_snip_contracts(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        ica_id: u64,
        channel_id: String,
    ) -> Result<Response, ContractError> {
        let mut msgs = vec![];
        let state = ICA_STATES.load(deps.storage, ica_id)?;

        if let Some(hp) = state.headstash_params {
            for coin in info.funds {
                if let Some(token) = hp.token_params.iter().find(|t| t.native == coin.denom) {
                    if let Some(snip) = token.snip_addr.clone() {
                        let msg = ica::form_ibc_transfer_msg(
                            env.block.time,
                            600u64,
                            snip,
                            channel_id.clone(),
                            coin,
                        )?;
                        // add minter msg
                        msgs.push(msg);
                    } else {
                        return Err(ContractError::SnipTokenNotSet {});
                    }
                }
            }
        } else {
            return Err(ContractError::NoHeadstashInfo {});
        }
        Ok(Response::new().add_messages(msgs))
    }

    pub fn ica_add_headstash_claimers(
        deps: DepsMut,
        info: MessageInfo,
        ica_id: u64,
        to_add: Vec<Headstash>,
    ) -> Result<Response, ContractError> {
        let mut msgs = vec![];
        let cw_ica_contract =
            helpers::retrieve_ica_owner_account(deps.as_ref(), info.sender.clone(), ica_id)?;

        let state = ICA_STATES.load(deps.storage, ica_id)?;
        let hp = state.headstash_params.unwrap();

        if let Some(ica) = state.ica_state.clone() {
            let hs_addr = hp.headstash.unwrap();
            // add headstash claimers msg
            let msg = ica::form_add_headstash(ica.ica_addr.clone(), hs_addr.clone(), to_add)?;
            msgs.push(msg);
        } else {
            return Err(ContractError::IcaInfoNotSet {});
        }
        // push msgs for ica to run
        let ica_msg = helpers::send_msg_as_ica(msgs, cw_ica_contract);
        Ok(Response::new().add_message(ica_msg))
    }

    pub fn ica_authorize_feegrant(
        deps: DepsMut,
        info: MessageInfo,
        ica_id: u64,
        to_grant: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut msgs = vec![];
        let cw_ica_contract =
            helpers::retrieve_ica_owner_account(deps.as_ref(), info.sender.clone(), ica_id)?;

        let state = ICA_STATES.load(deps.storage, ica_id)?;
        let hp = state.headstash_params.unwrap();

        if let Some(ica) = state.ica_state.clone() {
            // add headstash claimers msg
            for addr in to_grant {
                let msg = ica::form_authorize_feegrant(ica.ica_addr.clone(), addr)?;
                msgs.push(msg);
            }
        } else {
            return Err(ContractError::IcaInfoNotSet {});
        }
        // push msgs for ica to run
        let ica_msg = helpers::send_msg_as_ica(msgs, cw_ica_contract);
        Ok(Response::new().add_message(ica_msg))
    }
}

mod ica {
    use anybuf::Anybuf;
    use cosmwasm_std::{Coin, IbcTimeout, Timestamp};
    use cw_ica_controller::types::state::headstash::HeadstashTokenParams;
    use secret_headstash::state::Headstash;

    use super::*;
    /// Handles ICA controller callback messages.
    pub fn ica_callback_handler(
        deps: DepsMut,
        info: MessageInfo,
        callback_msg: IcaControllerCallbackMsg,
    ) -> Result<Response, ContractError> {
        let ica_id = CONTRACT_ADDR_TO_ICA_ID.load(deps.storage, info.sender)?;
        let mut ica_state = ICA_STATES.load(deps.storage, ica_id)?;

        match callback_msg {
            IcaControllerCallbackMsg::OnAcknowledgementPacketCallback {
                ica_acknowledgement,
                original_packet,
                relayer,
            } => todo!(),
            IcaControllerCallbackMsg::OnTimeoutPacketCallback {
                original_packet,
                relayer,
            } => todo!(),
            IcaControllerCallbackMsg::OnChannelOpenAckCallback {
                channel,
                ica_address,
                tx_encoding,
            } => {
                ica_state.ica_state = Some(crate::state::IcaState {
                    ica_id,
                    channel_state: ChannelState {
                        channel,
                        channel_status: ChannelStatus::Open,
                    },
                    ica_addr: ica_address,
                    tx_encoding,
                });
                ICA_STATES.save(deps.storage, ica_id, &ica_state)?;
            }
        };

        Ok(Response::default())
    }

    /// Instantiates a snip25 token on Secret Network via Stargate
    pub fn form_instantiate_snip25(
        sender: String,
        coin: HeadstashTokenParams,
        code_hash: String,
        code_id: u64,
        headstash: Option<String>,
        symbol: String,
    ) -> Result<CosmosMsg, ContractError> {
        let init_msg = snip20_reference_impl::msg::InstantiateMsg {
            name: "Terp Network SNIP25 - ".to_owned() + coin.name.as_str(),
            admin: headstash,
            symbol,
            decimals: 6u8,
            initial_balances: None,
            prng_seed: secret_cosmwasm_std::Binary(
                "eretjeretskeretjablereteretjeretskeretjableret"
                    .to_string()
                    .into_bytes(),
            ),
            config: None,
            supported_denoms: Some(vec![coin.ibc.clone()]),
        };
        Ok(
            #[allow(deprecated)]
            CosmosMsg::Stargate {
                type_url: "/secret.compute.v1beta1.MsgInstantiateContract".into(),
                value: anybuf::Anybuf::new()
                    .append_string(1, sender.to_string()) // sender (DAO)
                    .append_string(2, &code_hash.to_string()) // callback_code_hash
                    .append_uint64(3, code_id) // code-id of snip-25
                    .append_string(
                        4,
                        "SNIP25 For Secret Network - ".to_owned() + coin.name.as_str(),
                    ) // label of snip20
                    .append_bytes(5, to_json_binary(&init_msg)?.as_slice())
                    .append_string(8, &code_hash.to_string()) // callback_code_hash
                    .into_vec()
                    .into(),
            },
        )
    }
    pub fn form_authorize_minter(
        sender: String,
        headstash: String,
        snip25: String,
    ) -> Result<CosmosMsg, ContractError> {
        let set_minter_msg = snip20_reference_impl::msg::ExecuteMsg::AddMinters {
            minters: vec![headstash.clone()],
            padding: None,
        };
        Ok(
            #[allow(deprecated)]
            CosmosMsg::Stargate {
                type_url: "/secret.compute.v1beta1.MsgExecuteContract".into(),
                value: anybuf::Anybuf::new()
                    .append_string(1, sender.to_string()) // sender (DAO)
                    .append_string(2, &snip25.to_string()) // contract
                    .append_bytes(3, to_json_binary(&set_minter_msg)?.as_slice())
                    .into_vec()
                    .into(),
            },
        )
    }
    pub fn form_ibc_transfer_msg(
        time: Timestamp,
        seconds: u64,
        snip25: String,
        channel_id: String,
        coin: Coin,
    ) -> Result<CosmosMsg, ContractError> {
        Ok(CosmosMsg::Ibc(cosmwasm_std::IbcMsg::Transfer {
            channel_id,
            to_address: snip25,
            amount: coin,
            timeout: IbcTimeout::with_timestamp(time.plus_seconds(seconds)),
            memo: None,
        }))
    }

    pub fn form_add_headstash(
        sender: String,
        headstash: String,
        to_add: Vec<Headstash>,
    ) -> Result<CosmosMsg, ContractError> {
        let msg = secret_headstash::msg::ExecuteMsg::Add { headstash: to_add };
        Ok(
            #[allow(deprecated)]
            CosmosMsg::Stargate {
                type_url: "/secret.compute.v1beta1.MsgExecuteContract".into(),
                value: anybuf::Anybuf::new()
                    .append_string(1, sender.to_string()) // sender (DAO)
                    .append_string(2, &headstash.to_string()) // contract
                    .append_bytes(3, to_json_binary(&msg)?.as_slice())
                    .into_vec()
                    .into(),
            },
        )
    }

    pub fn form_authorize_feegrant(
        sender: String,
        grantee: String,
    ) -> Result<CosmosMsg, ContractError> {
        Ok(
            #[allow(deprecated)]
            CosmosMsg::Stargate {
                type_url: "/cosmos.feegrant.v1beta1.MsgGrantAllowance".into(),
                value: Anybuf::new()
                    .append_string(1, sender.to_string()) // granter (DAO)
                    .append_string(2, &grantee.to_string()) // grantee
                    .append_message(3, &Anybuf::new()) //todo: add allowance
                    .into_vec()
                    .into(),
            },
        )
    }
}
mod query {
    use crate::state::{IcaContractState, ICA_COUNT, ICA_STATES};

    use super::*;

    /// Returns the saved contract state.
    pub fn state(deps: Deps) -> StdResult<ContractState> {
        STATE.load(deps.storage)
    }

    /// Returns the saved ICA state for the given ICA ID.
    pub fn ica_state(deps: Deps, ica_id: u64) -> StdResult<IcaContractState> {
        ICA_STATES.load(deps.storage, ica_id)
    }

    /// Returns the saved ICA count.
    pub fn ica_count(deps: Deps) -> StdResult<u64> {
        ICA_COUNT.load(deps.storage)
    }
}

mod helpers {

    use crate::state::ICA_STATES;

    use super::*;
    pub fn retrieve_ica_owner_account(
        deps: Deps,
        sender: Addr,
        ica_id: u64,
    ) -> Result<CwIcaControllerContract, ContractError> {
        let contract_state = STATE.load(deps.storage)?;
        contract_state.verify_admin(sender)?;

        let ica_state = ICA_STATES.load(deps.storage, ica_id)?;

        Ok(CwIcaControllerContract::new(Addr::unchecked(
            ica_state.contract_addr,
        )))
    }

    pub fn send_msg_as_ica(
        msgs: Vec<CosmosMsg>,
        cw_ica_contract: CwIcaControllerContract,
    ) -> CosmosMsg {
        let ica_controller_msg = IcaControllerExecuteMsg::SendCosmosMsgs {
            messages: msgs,
            packet_memo: None,
            timeout_seconds: None,
        };

        cw_ica_contract.execute(ica_controller_msg).unwrap()
    }
}

#[cfg(test)]
mod tests {}
