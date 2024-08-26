use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ica_controller::helpers::ica_callback_execute;
use cw_ica_controller::types::msg::options::ChannelOpenInitOptions;
use cw_ica_controller::types::state::headstash::HeadstashParams;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub ica_controller_code_id: u64,
    pub snip25_code_id: u64,
}

#[ica_callback_execute]
#[cw_serde]
pub enum ExecuteMsg {
    CreateIcaContract {
        salt: Option<String>,
        channel_open_init_options: ChannelOpenInitOptions,
        headstash_params: HeadstashParams,
    },
    /// SendPredefinedAction sends a predefined action from the ICA controller to the ICA host.
    /// This demonstration is useful for contracts that have predefined actions such as DAOs.
    ///
    /// In this example, the predefined action is a `MsgSend` message which sends 100 "stake" tokens.
    UploadHeadstash {
        /// The ICA ID.
        ica_id: u64,
        /// The recipient's address, on the counterparty chain, to send the tokens to from ICA host.
        to_address: String,
    },
    InstantiateHeadstash {
        /// The ICA ID.
        ica_id: u64,
        /// Timestamp seconds of when headstash can begin
        start_date: u64,
        /// Total token supply for each involved asset. Will be depreciated for more granular control with
        total: headstash_cosmwasm_std::Uint128,
    },
    InstantiateTerpNetworkSNIP25 {
        /// The ICA ID.
        ica_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// GetContractState returns the contact's state.
    #[returns(crate::state::ContractState)]
    GetContractState {},
    /// GetIcaState returns the ICA state for the given ICA ID.
    #[returns(crate::state::IcaContractState)]
    GetIcaContractState { ica_id: u64 },
    /// GetIcaCount returns the number of ICAs.
    #[returns(u64)]
    GetIcaCount {},
}
