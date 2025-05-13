use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, CosmosMsg, QueryRequest, SubMsg};

use token_bindings::{KiichainMsg, KiichainQuery};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ReflectMsg {
        msgs: Vec<CosmosMsg<KiichainMsg>>,
    },
    ReflectSubMsg {
        msgs: Vec<SubMsg<KiichainMsg>>,
    },
    ChangeOwner {
        owner: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OwnerResponse)]
    Owner {},
    /// Queries the blockchain and returns the result untouched
    #[returns(ChainResponse)]
    Chain {
        request: QueryRequest<KiichainQuery>,
    },
    /// If there was a previous ReflectSubMsg with this ID, returns cosmwasm_std::Reply
    #[returns(cosmwasm_std::Reply)]
    SubMsgResult { id: u64 },
}

// We define a custom struct for each query response

#[cw_serde]
pub struct OwnerResponse {
    pub owner: String,
}

#[cw_serde]
pub struct CapitalizedResponse {
    pub text: String,
}

#[cw_serde]
pub struct ChainResponse {
    pub data: Binary,
}
