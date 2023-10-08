use anyhow::Result as AnyResult;
use cosmwasm_std::{Addr, Coin, Empty, Uint128};
use cw_multi_test::{App, AppBuilder, AppResponse, Contract, ContractWrapper, Executor};

use cw_membership::{
    contract::{execute, instantiate, query},
    ContractError,
};
use membership::{
    member::Member,
    msg::{
        EnableMembershipMsg, ExecuteMsg, InstantiateMsg, LinkSocialMediaMsg, MemberCountResponse,
        MembersResponse, MembershipSupplyResponse, MembershipsResponse, QueryMemberCountMsg,
        QueryMembersMsg, QueryMembershipSupplyMsg, QueryMembershipsMsg, QueryMsg,
    }, membership::Membership,
};

pub const FAUCET: &str = "faucet";

pub const ADMIN: &str = "terra1";
pub const REGISTER_ADMIN: &str = "terra2";
pub const FEE_COLLECTOR: &str = "terra3";

pub const USER_1: &str = "terra4";
pub const USER_2: &str = "terra5";

pub const SOCIAL_MEDIA_HANDLE_1: &str = "twitter1";
// pub const SOCIAL_MEDIA_HANDLE_2: &str = "twitter2";

pub const FEE_DENOM: &str = "uluna";

fn mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(FAUCET),
                vec![Coin {
                    denom: FEE_DENOM.to_string(),
                    // 1_000_000_000 uLuna i.e. 1k LUNA since 1 LUNA = 1_000_000 uLuna
                    amount: Uint128::new(1_000_000_000),
                }],
            )
            .unwrap();
    })
}

fn contract_cw_thread() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

pub fn proper_instantiate() -> (App, Addr, Addr, Addr, Addr, Addr, Addr) {
    let mut app = mock_app();
    let cw_thread_contract_code_id = app.store_code(contract_cw_thread());

    let msg = InstantiateMsg {
        admin_addr: Some(ADMIN.to_string()),
        registration_admin_addr: Some(REGISTER_ADMIN.to_string()),
        protocol_fee_collector_addr: Some(FEE_COLLECTOR.to_string()),
        fee_denom: Some(FEE_DENOM.to_string()),
        protocol_fee_membership_trading_fee_percentage: None,
        default_trading_fee_percentage_of_membership: None,
        default_membership_trading_fee_membership_issuer_fee_percentage: None,
        default_membership_trading_fee_membership_holder_fee_percentage: None,
    };
    let cw_thread_contract_addr = app
        .instantiate_contract(
            cw_thread_contract_code_id,
            Addr::unchecked(ADMIN),
            &msg,
            &[],
            "cw_thread",
            None,
        )
        .unwrap();

    let admin_addr = Addr::unchecked(ADMIN.to_string());
    let registration_admin_addr = Addr::unchecked(REGISTER_ADMIN.to_string());
    let fee_collector_addr = Addr::unchecked(FEE_COLLECTOR.to_string());
    let user_1_addr = Addr::unchecked(USER_1.to_string());
    let user_2_addr = Addr::unchecked(USER_2.to_string());

    (
        app,
        cw_thread_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    )
}

pub fn get_fund_from_faucet(app: &mut App, addr: Addr, amount: Uint128) {
    app.send_tokens(
        Addr::unchecked(FAUCET),
        addr,
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount,
        }],
    )
    .unwrap();
}

pub fn register_user(app: &mut App, cw_thread_contract_addr: &Addr, user_addr: &Addr) {
    app.execute_contract(
        user_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::Register(),
        &[],
    )
    .unwrap();
}

pub fn link_social_media_and_enable_membership(
    app: &mut App,
    cw_thread_contract_addr: &Addr,
    registration_admin_addr: &Addr,
    user_addr: &Addr,
    social_media_handle: &str,
) {
    app.execute_contract(
        registration_admin_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::LinkSocialMedia(LinkSocialMediaMsg {
            user_addr: user_addr.to_string(),
            social_media_handle: social_media_handle.to_string(),
        }),
        &[],
    )
    .unwrap();
    app.execute_contract(
        registration_admin_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::EnableMembership(EnableMembershipMsg {
            user_addr: user_addr.to_string(),
        }),
        &[],
    )
    .unwrap();
}

pub fn print_balance(
    app: &App,
    contract_addr: &Addr,
    admin_addr: &Addr,
    fee_collector_addr: &Addr,
    registration_admin_addr: &Addr,
    user_1_addr: &Addr,
    user_2_addr: &Addr,
) {
    println!(
        "contract_addr balance {}, admin balance {}, fee_collector balance {}, register_admin balance {}, user_1 balance {}, user_2 balance {}",
        app.wrap().query_balance(contract_addr.clone(), FEE_DENOM).unwrap(),
        app.wrap().query_balance(admin_addr.clone(), FEE_DENOM).unwrap(),
        app.wrap().query_balance(fee_collector_addr.clone(), FEE_DENOM).unwrap(),
        app.wrap().query_balance(registration_admin_addr.clone(), FEE_DENOM).unwrap(),
        app.wrap().query_balance(user_1_addr.clone(), FEE_DENOM).unwrap(),
        app.wrap().query_balance(user_2_addr.clone(), FEE_DENOM).unwrap(),
    );
}

pub fn assert_err(res: AnyResult<AppResponse>, err: ContractError) {
    match res {
        Ok(_) => panic!("Result was not an error"),
        Err(generic_err) => {
            let contract_err: ContractError = generic_err.downcast().unwrap();
            assert_eq!(contract_err, err);
        }
    }
}

pub fn assert_balance(app: &App, user_addr: &Addr, expected_balance: Uint128, denom: &str) {
    let balance = app.wrap().query_balance(user_addr, denom).unwrap();
    assert_eq!(balance.amount, expected_balance);
}

pub fn assert_membership_supply(
    app: &App,
    contract_addr: &Addr,
    membership_issuer_addr: &Addr,
    expected_supply: Uint128,
) {
    let query_membership_supply_res: MembershipSupplyResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::QueryMembershipSupply(QueryMembershipSupplyMsg {
                membership_issuer_addr: membership_issuer_addr.to_string(),
            }),
        )
        .unwrap();
    assert_eq!(
        query_membership_supply_res,
        MembershipSupplyResponse {
            supply: expected_supply
        }
    );
}

pub fn assert_member_count(
    app: &App,
    contract_addr: &Addr,
    membership_issuer_addr: &Addr,
    expected_count: Uint128,
) {
    let query_membership_supply_res: MemberCountResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::QueryMemberCount(QueryMemberCountMsg {
                membership_issuer_addr: membership_issuer_addr.to_string(),
            }),
        )
        .unwrap();
    assert_eq!(
        query_membership_supply_res,
        MemberCountResponse {
            count: expected_count
        }
    );
}

pub fn assert_members(
    app: &App,
    contract_addr: &Addr,
    membership_issuer_addr: &Addr,
    expected_members: Vec<Member>,
) {
    let query_members_res: MembersResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::QueryMembers(QueryMembersMsg {
                membership_issuer_addr: membership_issuer_addr.to_string(),
                start_after_member_addr: None,
                limit: None,
            }),
        )
        .unwrap();
    assert_eq!(
        query_members_res,
        MembersResponse {
            members: expected_members.clone(),
            total_count: expected_members.len() as usize,
            count: expected_members.len() as usize
        }
    );
}

pub fn assert_memberships(
    app: &App,
    contract_addr: &Addr,
    user_addr: &Addr,
    expected_memberships: Vec<Membership>,
) {
    let query_memberships_res: MembershipsResponse = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &QueryMsg::QueryMemberships(QueryMembershipsMsg {
                user_addr: user_addr.to_string(),
                start_after_membership_issuer_addr: None,
                limit: None,
            }),
        )
        .unwrap();
    assert_eq!(
        query_memberships_res,
        MembershipsResponse {
            memberships: expected_memberships.clone(),
            total_count: expected_memberships.len() as usize,
            count: expected_memberships.len() as usize
        }
    );
}
