use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Uint64};

use crate::{config::Config, member::Member, membership::Membership, user::User};

// TODO: P0: add a proxy contract that can charge custom fee so people can build tailored frontend

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    // Default to sender
    pub admin_addr: Option<String>,
    // Default to sender
    pub registration_admin_addr: Option<String>,
    // Default to sender
    pub protocol_fee_collector_addr: Option<String>,
    // Default to uluna
    // TODO: P1: use noble USDC
    pub fee_denom: Option<String>,

    // Protocol fee percentage for membership trading
    pub protocol_fee_membership_trading_fee_percentage: Option<Uint64>,

    // Default membership trading fee in my 1 membership price percentage
    pub default_trading_fee_percentage_of_membership: Option<Uint64>,

    // Default membership trading fee to membership issuer fee percentage
    pub default_membership_trading_fee_membership_issuer_fee_percentage: Option<Uint64>,
    // Default membership trading fee to membership holder fee percentage
    pub default_membership_trading_fee_membership_holder_fee_percentage: Option<Uint64>,
    // TODO: P0: add new default param on how much membership each holder can own
    // TODO: P0: add new default param on whether only allow verified user to buy membership
    // TODO: P1: setup fee grant to cover onboarding fee, enough to register, post and ask
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    Enable(EnableMsg),
    Disable(DisableMsg),

    EnableOpenRegistration(EnableOpenRegistrationMsg),
    DisableOpenRegistration(DisableOpenRegistrationMsg),

    UpdateConfig(UpdateConfigMsg),

    // Anyone can register an account
    // But without registering a membership they can only buy and sell other people's memberships but not issue their own memberships
    Register(),

    // Only register admin can link social media for user
    LinkSocialMedia(LinkSocialMediaMsg),

    // Only register admin can register membership for user
    // User must link social media first to be eligible for membership registration to prevent impersonation
    // This will initialize the user's membership and set the supply to 1 owned by the user
    // After that anyone can buy / sell user's membership
    EnableMembership(EnableMembershipMsg),

    // Only membership issuer can update its membership trading fee percentage
    UpdateTradingFeePercentageOfMembership(UpdateTradingFeePercentageOfMembershipMsg),

    // Only membership issuer can update its membership trading fee config
    UpdateMembershipTradingFeeShareConfig(UpdateMembershipTradingFeeShareConfigMsg),

    // Anyone can buy membership
    BuyMembership(BuyMembershipMsg),

    // Anyone can sell membership if they have it
    SellMembership(SellMembershipMsg),
}

#[cw_serde]
pub struct EnableMsg {}

#[cw_serde]
pub struct DisableMsg {}

#[cw_serde]
pub struct EnableOpenRegistrationMsg {}

#[cw_serde]
pub struct DisableOpenRegistrationMsg {}

#[cw_serde]
pub struct UpdateConfigMsg {
    pub admin_addr: Option<String>,
    pub registration_admin_addr: Option<String>,
    pub protocol_fee_collector_addr: Option<String>,
    pub fee_denom: Option<String>,

    pub protocol_fee_membership_trading_fee_percentage: Option<Uint64>,

    pub default_trading_fee_percentage_of_membership: Option<Uint64>,

    pub default_share_to_issuer_percentage: Option<Uint64>,
    pub default_share_to_all_members_percentage: Option<Uint64>,
}

#[cw_serde]
pub struct LinkSocialMediaMsg {
    pub user_addr: String,
    pub social_media_handle: String,
}

#[cw_serde]
pub struct EnableMembershipMsg {
    pub user_addr: String,
}

#[cw_serde]
pub struct UpdateTradingFeePercentageOfMembershipMsg {
    pub membership_issuer_addr: String,
    pub trading_fee_percentage_of_membership: Uint64,
}

#[cw_serde]
pub struct UpdateMembershipTradingFeeShareConfigMsg {
    pub membership_issuer_addr: String,
    // Revenue share percentage for membership issuer
    pub share_to_issuer_percentage: Uint64,
    // Revenue share percentage for all members
    pub share_to_all_members_percentage: Uint64,
}

#[cw_serde]
pub struct BuyMembershipMsg {
    pub membership_issuer_addr: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct SellMembershipMsg {
    pub membership_issuer_addr: String,
    pub amount: Uint128,
}

// ========== query ==========

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig(QueryConfigMsg),

    #[returns(UserResponse)]
    QueryUser(QueryUserMsg),

    // Get total number of memberships issued by the membership issuer
    #[returns(MembershipSupplyResponse)]
    QueryMembershipSupply(QueryMembershipSupplyMsg),

    // Get total number of members holding the membership
    // This is not necessarily the same as the supply because each member can hold multiple memberships
    #[returns(MemberCountResponse)]
    QueryMemberCount(QueryMemberCountMsg),

    // Returns all members, with pagination
    #[returns(MembersResponse)]
    QueryMembers(QueryMembersMsg),

    // Returns all memberships user currently holds, with pagination
    #[returns(MembershipsResponse)]
    QueryMemberships(QueryMembershipsMsg),

    // QueryCostToBuyMembership calculates the price and fee
    #[returns(CostToBuyMembershipResponse)]
    QueryCostToBuyMembership(QueryCostToBuyMembershipMsg),

    // QueryCostToSellMembership calculates the price and fee
    #[returns(CostToSellMembershipResponse)]
    QueryCostToSellMembership(QueryCostToSellMembershipMsg),
}

#[cw_serde]
pub struct QueryConfigMsg {}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct QueryUserMsg {
    pub user_addr: String,
}

#[cw_serde]
pub struct UserResponse {
    pub user: User,
}

#[cw_serde]
pub struct QueryMembershipSupplyMsg {
    pub membership_issuer_addr: String,
}

#[cw_serde]
pub struct MembershipSupplyResponse {
    pub supply: Uint128,
}

#[cw_serde]
pub struct QueryMemberCountMsg {
    pub membership_issuer_addr: String,
}

#[cw_serde]
pub struct MemberCountResponse {
    pub count: Uint128,
}

#[cw_serde]
pub struct QueryMembersMsg {
    pub membership_issuer_addr: String,
    pub start_after_member_addr: Option<String>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct MembersResponse {
    pub members: Vec<Member>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryMembershipsMsg {
    pub user_addr: String,
    pub start_after_membership_issuer_addr: Option<String>,
    pub limit: Option<u32>,
}

#[cw_serde]
pub struct MembershipsResponse {
    pub memberships: Vec<Membership>,
    pub count: usize,
    pub total_count: usize,
}

#[cw_serde]
pub struct QueryCostToBuyMembershipMsg {
    pub membership_issuer_addr: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct CostToBuyMembershipResponse {
    // Price is total price for buy amount of membership, not the price per membership
    pub price: Uint128,
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to membership issuer
    pub issuer_fee: Uint128,
    // Fee paid to all members
    pub all_members_fee: Uint128,
    // Price + protocol fee + membership issuer fee + membership holder fee
    pub total_needed_from_user: Uint128,
}

#[cw_serde]
pub struct QueryCostToSellMembershipMsg {
    pub membership_issuer_addr: String,
    pub amount: Uint128,
}

#[cw_serde]
pub struct CostToSellMembershipResponse {
    // Price is total price for sell amount of membership, not the price per membership
    pub price: Uint128,
    // Fee paid to protocol
    pub protocol_fee: Uint128,
    // Fee paid to membership issuer
    pub issuer_fee: Uint128,
    // Fee paid to all members
    pub all_members_fee: Uint128,
    // Protocol fee + membership issuer fee + membership holder fee
    pub total_needed_from_user: Uint128,
}
