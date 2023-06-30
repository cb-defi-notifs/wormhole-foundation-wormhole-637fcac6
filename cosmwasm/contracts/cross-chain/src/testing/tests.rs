use cosmwasm_std::{StdResult};

use cw_wormhole::{
    byte_utils::{
        get_string_from_32,
    },
};

use crate::{
    state::{RegisterChainChannel, TransferPayload, UpgradeContract},
};

#[test]
fn verify_get_string_from_32_handles_null_strings() -> StdResult<()> {
    let data = hex::decode("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let channel_id = get_string_from_32(&data);
    assert_eq!("", channel_id);
    Ok(())
}

#[test]
fn verify_get_string_from_32_handles_longer_strings() -> StdResult<()> {
    let long_string = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789?!";
    let data = long_string.as_bytes();
    let channel_id = get_string_from_32(&data);
    assert_eq!(long_string, channel_id);
    Ok(())
}

#[test]
fn verify_register_chain_channel_deserialize() -> StdResult<()> {
    let data = hex::decode("00120000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006368616e6e656c2d3138").unwrap();
    let RegisterChainChannel {
        chain_id,
        channel_id,
    } = RegisterChainChannel::deserialize(&data)?;
    assert_eq!(18, chain_id);
    assert_eq!("channel-18".to_string(), channel_id);
    Ok(())
}

#[test]
fn verify_upgrade_contract_deserialize() -> StdResult<()> {
    let data = hex::decode("0000000000000000000000000000000000000000000000000000000000001234").unwrap();
    let UpgradeContract {
        new_contract,
    } = UpgradeContract::deserialize(&data)?;
    assert_eq!(0x1234, new_contract);
    Ok(())
}


/*
new Uint8Array(
  Buffer.from(
    JSON.stringify({
      basic_transfer: {
        chain_id: 18,
        recipient: "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v"
      }
    })
  )
)

'{"basic_transfer":{"chain_id":18,"recipient":"terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v"}}'
 */

#[test]
fn verify_transfer_payload_deserialize() -> StdResult<()> {
    let json = "{\"basic_transfer\":{\"chain_id\":18,\"recipient\":\"terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v\"}}";
    let payload: TransferPayload = serde_json_wasm::from_slice(json.as_bytes()).unwrap();
    match payload {
        TransferPayload::BasicTransfer { chain_id, recipient } => {
            assert_eq!(18, chain_id);
            assert_eq!("terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v", recipient.to_string());
            Ok(())
        }
    }
}
