use std::path::PathBuf;
use std::str::FromStr;

use amplify::hex::FromHex;
use bp::dbc::Method;
use bp::{Outpoint, Txid};
use ifaces::Rgb20;
use rgbstd::containers::{FileContent, Kit};
use rgbstd::interface::{FilterIncludeAll, FungibleAllocation};
use rgbstd::invoice::Precision;
use rgbstd::persistence::{MemIndex, MemStash, MemState, Stock};
use schemata::dumb::DumbResolver;
use schemata::NonInflatableAsset;

#[rustfmt::skip]
fn main() {
    let beneficiary_txid = 
        Txid::from_hex("1240a21693434f5e872551ded3d349692490db7839aff89a1667e78d36f6d51e").unwrap();
    let beneficiary = Outpoint::new(beneficiary_txid, 0);

    let contract = Rgb20::testnet::<NonInflatableAsset>("ssi:anonymous","TEST", "Test asset", None, Precision::CentiMicro)
        .expect("invalid contract data")
        .allocate(Method::TapretFirst, beneficiary, 100_000_000_000_u64)
        .expect("invalid allocations")
        .issue_contract()
        .expect("invalid contract data");

    let contract_id = contract.contract_id();

    eprintln!("{contract}");
    contract.save_file("examples/rgb20-simplest.rgb").expect("unable to save contract");
    contract.save_armored("examples/rgb20-simplest.rgba").expect("unable to save armored contract");

    let kit = Kit::load_file("schemata/NonInflatableAssets.rgb").unwrap().validate().unwrap();

    let stock_path = PathBuf::from_str("examples").expect("invalid path");
    let mut stock = Stock::<MemStash, MemState, MemIndex>::new(stock_path);

    stock.import_kit(kit).expect("invalid issuer kit");
    stock.import_contract(contract, &mut DumbResolver).unwrap();

    // Reading contract state through the interface from the stock:
    let contract = stock.contract_iface_class::<Rgb20>(contract_id).unwrap();
    // let contract = Rgb20::from(contract);
    let allocations = contract.fungible("assetOwner", &FilterIncludeAll).unwrap();
    eprintln!("\nThe issued contract data:");
    eprintln!("{}", serde_json::to_string(&contract.spec()).unwrap());

    for FungibleAllocation  { seal, state, witness, .. } in allocations {
        eprintln!("amount={state}, owner={seal}, witness={witness}");
    }
    eprintln!("totalSupply={}", contract.total_supply());
}
