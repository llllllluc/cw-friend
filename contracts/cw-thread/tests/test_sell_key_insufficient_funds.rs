use cosmwasm_std::{Coin, Uint128};
use cw_multi_test::Executor;

use cw_thread::ContractError;

use thread::msg::{
    BuyKeyMsg, CostToBuyKeyResponse, ExecuteMsg, QueryCostToBuyKeyMsg, QueryCostToSellKeyMsg,
    QueryMsg, SellKeyMsg,
};

pub mod helpers;
use crate::helpers::{
    assert_err, get_fund_from_faucet, link_social_media_and_register_key, print_balance,
    proper_instantiate, register_user, FEE_DENOM, SOCIAL_MEDIA_HANDLE_1,
};

#[test]
fn test_sell_key_insufficient_funds() {
    let (
        mut app,
        cw_thread_contract_addr,
        admin_addr,
        registration_admin_addr,
        fee_collector_addr,
        user_1_addr,
        user_2_addr,
    ) = proper_instantiate();

    let uint_128_amount_30 = Uint128::from(30_u8);
    let uint_128_amount_10 = Uint128::from(10_u8);

    register_user(&mut app, &cw_thread_contract_addr, &user_1_addr);
    link_social_media_and_register_key(
        &mut app,
        &cw_thread_contract_addr,
        &registration_admin_addr,
        &user_1_addr,
        SOCIAL_MEDIA_HANDLE_1,
    );

    print_balance(
        &app,
        &cw_thread_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // User 1 buy 30 amount of its own keys
    let query_user_1_simulate_buy_key_res: CostToBuyKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToBuyKey(QueryCostToBuyKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_30,
            }),
        )
        .unwrap();
    println!(
        "query_user_1_simulate_buy_key_res {:?}",
        query_user_1_simulate_buy_key_res
    );

    get_fund_from_faucet(
        &mut app,
        user_1_addr.clone(),
        query_user_1_simulate_buy_key_res.total_needed_from_user,
    );

    app.execute_contract(
        user_1_addr.clone(),
        cw_thread_contract_addr.clone(),
        &ExecuteMsg::BuyKey(BuyKeyMsg {
            key_issuer_addr: user_1_addr.to_string(),
            amount: uint_128_amount_30,
        }),
        &[Coin {
            denom: FEE_DENOM.to_string(),
            amount: query_user_1_simulate_buy_key_res.total_needed_from_user,
        }],
    )
    .unwrap();

    print_balance(
        &app,
        &cw_thread_contract_addr,
        &admin_addr,
        &fee_collector_addr,
        &registration_admin_addr,
        &user_1_addr,
        &user_2_addr,
    );

    // User 1 tries to sell 10 amount of its own keys but fails because it didn't pay enough protocol fee
    let query_user_1_simulate_sell_key_res: CostToBuyKeyResponse = app
        .wrap()
        .query_wasm_smart(
            cw_thread_contract_addr.clone(),
            &QueryMsg::QueryCostToSellKey(QueryCostToSellKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_10,
            }),
        )
        .unwrap();

    get_fund_from_faucet(&mut app, user_1_addr.clone(), Uint128::one());

    assert_err(
        app.execute_contract(
            user_1_addr.clone(),
            cw_thread_contract_addr.clone(),
            &ExecuteMsg::SellKey(SellKeyMsg {
                key_issuer_addr: user_1_addr.to_string(),
                amount: uint_128_amount_10,
            }),
            &[Coin {
                denom: FEE_DENOM.to_string(),
                amount: Uint128::one(),
            }],
        ),
        ContractError::InsufficientFundsToPayDuringSell {
            needed: query_user_1_simulate_sell_key_res.total_needed_from_user,
            available: Uint128::one(),
        },
    );
}
