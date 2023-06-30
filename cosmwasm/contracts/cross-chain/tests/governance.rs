mod helpers;
use cosmwasm_std::{Addr};
use cw_multi_test::{Executor};

use helpers::{CHANNEL_18, CHANNEL_42, create_submit_vaa_msg, instantiate_contracts, OWNER, query_chain_channels};

#[test]
fn submit_illformed_vaa() {
    // Just some random bytes.
    let execute_msg = create_submit_vaa_msg("0000000000000000000000000000000000000000000000000000000075757364");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: querier contract error: generic error: invalidvaa",
        err.root_cause().to_string().to_lowercase()
    );
}

#[test]
fn submit_governance_invalid_emitter_chain() {
    // Emitter chain is Ethereum, not Solana.
    let execute_msg = create_submit_vaa_msg("010000000001002102d5e805aba1b18ccb4ef196b1ea8750e3486cd9b5f4c17740251acf63bce856bac67ea310ab5b672416821ab9b07c3ccb4ece93f11ab01aef317710bc1e720000000000a5567d7d00020000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: expected a governance vaa",
        err.root_cause().to_string().to_lowercase()
    );
}

#[test]
fn submit_governance_invalid_emitter_address() {
    // Emitter address is five, not four.
    let execute_msg = create_submit_vaa_msg("0100000000010041d427e5a930c218de4d454981e80192f382d602e0e7a85651b6d62fcef22ee27ba91839d2383a3b6d7d6a0b5b0e4fc29d824a02f19519871b286468464e27380000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000005ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: expected a governance vaa",
        err.root_cause().to_string().to_lowercase()
    );
}

#[test]
fn submit_governance_invalid_signature() {
    // Signed with Testnet key, not Devnet.
    let execute_msg = create_submit_vaa_msg("010000000001000cf1f94bedfe50b6b69a42149a6767fd090cf638fe9667360d981ee119adc20b31a90b1ce757ab97fa396f8a77c4a8818bea5b3cdf4ece189834f7c6754cacd20100000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: querier contract error: generic error: guardiansignatureerror",
        err.root_cause().to_string().to_lowercase()
    );
}

#[test]
#[should_panic]
fn submit_governance_short_payload_should_panic() {
    // Payload ends after the action.
    let execute_msg = create_submit_vaa_msg("01000000000100b19e738c7c719159672c52e29007390bc9ab1fbe6312937ec0d8d3077d65a5a9272647064a1b1d465a43d4f50e0743abe44297e113f446c138a7deab8daf15e70000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f7201");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have panicked");
}

#[test]
fn submit_governance_invalid_module() {
    // Module is IbcReceiver not IbcTranslator.
    let execute_msg = create_submit_vaa_msg("01000000000100251cbabeead4aa70a801e97032d929e8fa403681dc6608127353985970b248f279622101178a7075488abeb50b4b01f7278b4a603b8b3accb7a370e09243ff180000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f200000000000000000000000000000000000000000004962635265636569766572010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: governance vaa is for an invalid module",
        err.root_cause().to_string().to_lowercase()
    );
}

#[test]
fn submit_governance_not_for_wormchain() {
    // Target chain is Ethereum, not Wormchain.
    let execute_msg = create_submit_vaa_msg("01000000000100e5c8d8041c0c2369c620d6517d4aafda787840202fdced6dd0b1eeb9644728a61ef221c1cd6e80bb1320dcd4344c0771a0f0ed87208d52f4ab5970b572905bd90000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f7201000200120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: governance vaa is for another chain",
        err.root_cause().to_string().to_lowercase()
    );
}

#[test]
fn submit_governance_invalid_action() {
    // Governance action is 42 rather than 1.
    let execute_msg = create_submit_vaa_msg("01000000000100e016ae3a5e38b9c6b78db982646643b14aa37f98caf9e06fd253746780dada9c2b93907ff852580ff84f3c153b66d324f96b8935f752f6b36c313ea6e31eb8bd0000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f722a0c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: invalidvaaaction",
        err.root_cause().to_string().to_lowercase()
    );
}


#[test]
fn submit_governance_add_chain() {
    let execute_msg = create_submit_vaa_msg("010000000001003954625825b74af01b602e401026731b5eda40b0eec103c6c80a7d33102947ca111e67baaa4dca6e2313acc03292e19c60f9130656a6bc4e9ddffb84c17cc2a30000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

    let actual_channels = query_chain_channels(&router, ibc_gateway_contract_addr.clone());
    assert_eq!(actual_channels.len(), 1);
    assert_eq!(actual_channels[0].0, 18);
    assert_eq!(actual_channels[0].1.to_string(), CHANNEL_18.to_string());
}

#[test]
fn submit_governance_update_chain() {
    let execute_msg = create_submit_vaa_msg("010000000001003954625825b74af01b602e401026731b5eda40b0eec103c6c80a7d33102947ca111e67baaa4dca6e2313acc03292e19c60f9130656a6bc4e9ddffb84c17cc2a30000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();
    
    let actual_channels = query_chain_channels(&router, ibc_gateway_contract_addr.clone());
    assert_eq!(actual_channels.len(), 1);
    assert_eq!(actual_channels[0].0, 18);
    assert_eq!(actual_channels[0].1.to_string(), CHANNEL_18.to_string());

    let execute_msg2 = create_submit_vaa_msg("010000000001008dec02b6fe961837a8a35b7919a72a7b75cc3902fdffaddf1572ef2a55ea57a9266a5e204db75758c38d6223652967588be975dcf719bdf9bd072ac1611ee2f10100000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3432");

    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg2,
            &[],
        )
        .unwrap();
    
    let actual_channels2 = query_chain_channels(&router, ibc_gateway_contract_addr.clone());
    assert_eq!(actual_channels2.len(), 1);
    assert_eq!(actual_channels2[0].0, 18);
    assert_eq!(actual_channels2[0].1.to_string(), CHANNEL_42.to_string());
}

#[test]
fn submit_governance_already_executed() {
    let execute_msg = create_submit_vaa_msg("010000000001003954625825b74af01b602e401026731b5eda40b0eec103c6c80a7d33102947ca111e67baaa4dca6e2313acc03292e19c60f9130656a6bc4e9ddffb84c17cc2a30000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

    let err = router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .expect_err("submit_vaa should have failed");

    assert_eq!(
        "generic error: vaa already executed",
        err.root_cause().to_string().to_lowercase()
    );
}

#[test]
fn submit_governance_set_channel_id_to_null() {
    // First add the channel as normal.
    let execute_msg = create_submit_vaa_msg("010000000001003954625825b74af01b602e401026731b5eda40b0eec103c6c80a7d33102947ca111e67baaa4dca6e2313acc03292e19c60f9130656a6bc4e9ddffb84c17cc2a30000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

    let actual_channels = query_chain_channels(&router, ibc_gateway_contract_addr.clone());
    assert_eq!(actual_channels.len(), 1);
    assert_eq!(actual_channels[0].0, 18);
    assert_eq!(actual_channels[0].1.to_string(), CHANNEL_18.to_string());

    // Set channel_id to all zeros.
    let execute_msg2 = create_submit_vaa_msg("010000000001008cc95280459d52fae6a20770cae61fa02269fa7a6d513c0b7390e7e03c5a24060d77f1cca1af29da800ce4eb4f6125f1d5d5afd3bbea8c0bcdff1d9cde38d9d70000000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c20001200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg2,
            &[],
        )
        .unwrap();

    let actual_channels = query_chain_channels(&router, ibc_gateway_contract_addr.clone());
    assert_eq!(actual_channels.len(), 1);
    assert_eq!(actual_channels[0].0, 18);
    assert_eq!(actual_channels[0].1.to_string(), "".to_string());
}

#[test]
fn submit_governance_channel_id_is_max_len() {
    // Make the channel_id 64 chars long (ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789?!).
    let execute_msg = create_submit_vaa_msg("01000000000100fb0d5ed0b2bc3fb59027f0650682c5d5f6a88a3a91f7f4f8ecfb9ff87ce97819774b1f85c6162dee79f93aac7aa13e1d43b1fa2dab71212bbdca35b60eab4d0b0100000000a5567d7d00010000000000000000000000000000000000000000000000000000000000000004ee8c114665f9261f20000000000000000000000000000000000000004962635472616e736c61746f72010c2000124142434445464748494a4b4c4d4e4f505152535455565758595a6162636465666768696a6b6c6d6e6f707172737475767778797a303132333435363738393f21");

    let (mut router, ibc_gateway_contract_addr, _, _) = instantiate_contracts();
    router
        .execute_contract(
            Addr::unchecked(OWNER),
            ibc_gateway_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

    let actual_channels = query_chain_channels(&router, ibc_gateway_contract_addr.clone());
    assert_eq!(actual_channels.len(), 1);
    assert_eq!(actual_channels[0].0, 18);
    assert_eq!(actual_channels[0].1.to_string(), "QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVphYmNkZWZnaGlqa2xtbm9wcXJzdHV2d3h5ejAxMjM0NTY3ODk/IQ==".to_string());
}
