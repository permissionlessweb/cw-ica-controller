use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ica_controller::helpers::ica_callback_execute;
use cw_ica_controller::types::msg::options::ChannelOpenInitOptions;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
    pub ica_controller_code_id: u64,
    pub ica_controller_code_hash: String,
    pub headstash_addr: String,
    pub bloom_plaintxt_msg: String,
}

#[ica_callback_execute]
#[cw_serde]
pub enum ExecuteMsg {
    CreateIcaContract {
        channel_open_init_options: ChannelOpenInitOptions,
    },
    /// IbcBloom sends a predefined action from the ICA controller to the ICA host.
    ///
    ///
    /// In this example, the predefined action is a ICS20 transfer, but we verifiy an offline signature with a few specific requirements:
    /// - eth pubkey generated the signature
    /// - the signature contains the following structure: { destination wallet of token transfer} {message signer}
    /// if both are confirmed, any funds included will be sent to destination.
    /// todo:
    /// -
    IbcBloom {
        /// The ICA ID.
        ica_id: u64,
        /// The recipient's address, on the counterparty chain, to send the tokens to from ICA host.
        to_address: String,
        /// eth pubkey that generates the signature
        eth_pubkey: String, // 0x1
        /// eth_sig is ethereum signature hash generated from the to_address string.
        eth_sig: String,
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
