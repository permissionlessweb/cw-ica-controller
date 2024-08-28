use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ica_controller::{
    helpers::ica_callback_execute,
    types::{
        msg::options::ChannelOpenInitOptions,
        state::headstash::{HeadstashParams, HeadstashTokenParams},
    },
};
use secret_headstash::state::Headstash;

#[cw_serde]
pub struct Snip25InitParams {
    pub ibc_hash: String,
    pub native: String,
}
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
    /// 1. Upload the headstash contract code
    UploadHeadstash {
        /// The ICA ID.
        ica_id: u64,
    },
    /// 2. Instantiates the secret headstash contract on Secret Network.
    InstantiateHeadstash {
        /// The ICA ID.
        ica_id: u64,
        /// Timestamp seconds of when headstash can begin
        start_date: u64,
        /// Total token supply for each involved asset. Will be depreciated for more granular control with
        total: headstash_cosmwasm_std::Uint128,
    },
    /// 3. Instantiate a snip25 contract for every token defined in tokens.
    InstantiateTerpNetworkSnip25 {
        /// The ICA ID.
        ica_id: u64,
        /// Tokens to have their snip25 contract created
        tokens: Vec<HeadstashTokenParams>,
    },
    /// 4. Authorized the headstash contract as a minter for both snip25 contracts.
    AuthorizeMinter { ica_id: u64 },
    /// 5. Transfer each token included in msg over via ics20.
    IBCTransferTokens { ica_id: u64, channel_id: String },
    /// 6. Add Eligible Addresses To Headstash
    AddHeadstashClaimers { ica_id: u64, to_add: Vec<Headstash> },
    /// 7. Authorize
    AuthorizeFeegrant { ica_id: u64, to_grant: Vec<String> },
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
