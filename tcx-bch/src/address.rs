use tcx_chain::curve::{Secp256k1Curve, PublicKey, Secp256k1PublicKey, CurveType};
use tcx_chain::{Coin, HdKeystore, Account};
use crate::Result;
use bitcoin::network::constants::Network;
use bitcoin::{Address as BtcAddress, PublicKey as BtcPublicKey, PrivateKey};
use tcx_chain::keystore::{KeyType, Address, Extra, CoinInfo};
use secp256k1::{SecretKey, Secp256k1};
use bitcoin_hashes::hex::ToHex;
use serde_json::Value;
use crate::transaction::{Utxo, BitcoinCashTransaction};
use std::str::FromStr;
use std::marker::PhantomData;
use bip39::{Mnemonic, Language, Seed};
use bch_addr::Converter;
use tcx_chain::bips::DerivationInfo;
use std::mem;



pub struct BchAddress {}

impl BchAddress {
    const XPUB_VERSION: [u8;4] = [0x04, 0x88, 0xb2, 0x1e];
    const XPRV_VERSION: [u8;4] = [0x04, 0x88, 0xad, 0xe4];

    pub fn is_main_net(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_mainnet_addr(addr)
    }
}

impl Address for BchAddress {

    fn is_valid(addr: &str) -> bool {
        let convert = Converter::new();
        convert.is_cash_addr(addr)
    }

    fn from_public_key(pub_key: &impl PublicKey) -> Result<String> {
//        let pub_key: &Secp256k1PublicKey = &pub_key;
        let pub_key: Secp256k1PublicKey =  Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
//        let pub_key = pub_key as &Secp256k1PublicKey;
        let legacy = BtcAddress::p2pkh(&pub_key, Network::Bitcoin);
        let convert = Converter::new();
        convert.to_cash_addr(&legacy.to_string()).map_err(|err| format_err!("{}", "generate_address_failed"))
    }
}

pub struct BchTestNetAddress {}

impl BchTestNetAddress {
    const XPUB_VERSION: [u8;4] = [0x04, 0x35, 0x87, 0xCF];
    const XPRV_VERSION: [u8;4] = [0x04, 0x35, 0x83, 0x94];
}

impl Address for BchTestNetAddress {

    fn is_valid(address: &str) -> bool {
        let convert = Converter::new();
        convert.is_cash_addr(address)
    }

    fn from_public_key(pub_key: &impl PublicKey) -> Result<String> {
        let pub_key = Secp256k1PublicKey::from_slice(&pub_key.to_bytes())?;
        let legacy = BtcAddress::p2pkh(&pub_key, Network::Testnet);
        let convert = Converter::new();
        convert.to_cash_addr(&legacy.to_string()).map_err(|err| format_err!("{}", "generate_address_failed"))
    }

    fn extended_public_key_version() -> [u8;4] {
        BchTestNetAddress::XPUB_VERSION
    }
    fn extended_private_key_version() -> [u8;4] {
        BchTestNetAddress::XPRV_VERSION
    }

}




