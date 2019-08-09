
use bitcoin::network::constants::Network;
use secp256k1::{Secp256k1, Message};

use bitcoin::{PrivateKey, TxIn, OutPoint, Script, PublicKey, TxOut, Transaction};
use bitcoin_hashes::sha256d::Hash as Hash256;
use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::hex::FromHex;
use std::str::FromStr;
use bitcoin_hashes::Hash;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey, DerivationPath};
use bip39::{Mnemonic, Language, Seed};

pub mod bip143_with_forkid;
pub mod hard_wallet_keystore;
pub mod address;
pub mod transaction;

use bip143_with_forkid::SighashComponentsWithForkId;
use core::result;
use tcx_chain::keystore::{Extra, CoinInfo};
use tcx_chain::curve::{CurveType, Secp256k1Curve};
use serde::{Deserialize, Serialize};
use tcx_chain::keystore::Address;

#[macro_use] extern crate failure;
#[macro_use] extern crate hex_literal;
extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;

pub type Result<T> = result::Result<T, failure::Error>;


const SYMBOL: &'static str = "BCH";
const PATH: &'static str = "m/44'/145'/0'/0/0";

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedPubKeyExtra {
    xpub: String
}


impl Extra for ExtendedPubKeyExtra {
    fn from(coin_info: &CoinInfo, seed: &Seed) -> Result<Self> {
        ensure!(coin_info.curve == CurveType::SECP256k1, "BCH must be at secp256k1");
        let derivation_info = Secp256k1Curve::extended_pub_key(&coin_info.derivation_path, &seed)?;
        let xpub = address::BchAddress::extended_public_key(&derivation_info);
        Ok(ExtendedPubKeyExtra {
            xpub
        })
    }

}





#[cfg(test)]
mod tests {
    use tcx_chain::{HdKeystore, Metadata, Account};
    use crate::address::{BchAddress};
    use tcx_chain::curve::{Secp256k1Curve, CurveType};
    use tcx_chain::coin::Coin;
    use serde_json::Value;
    use tcx_chain::keystore::CoinInfo;
    use crate::ExtendedPubKeyExtra;

    const PASSWORD: &str = "Insecure Password";
    const BIP_PATH: &str = "m/44'/145'/0'";
    const MNEMONIC: &str = "inject kidney empty canal shadow pact comfort wife crush horse wife sketch";

    #[test]
    fn bch_create() {

        let mut meta = Metadata::default();
        meta.name = "CreateTest".to_string();

        let mut keystore = HdKeystore::new("Insecure Password", meta);

//        let coin = BchCoin::<Secp256k1Curve, BchAddress>::append_account(&mut keystore, PASSWORD, BIP_PATH);
        let bch_coin = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };
        let coin: &Account = keystore.derive_coin::<BchAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD).unwrap();
        let json_str = keystore.json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();
        assert!(!address.is_empty());
        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BCH", coin);
    }

    #[test]
    fn bch_recover() {
        let mut meta = Metadata::default();
        meta.name = "RecoverTest".to_string();

        let mut keystore = HdKeystore::from_mnemonic(&MNEMONIC, &PASSWORD, meta);

        let bch_coin = CoinInfo {
            symbol: "BCH".to_string(),
            derivation_path: BIP_PATH.to_string(),
            curve: CurveType::SECP256k1,
        };

        let coin: &Account = keystore.derive_coin::<BchAddress, ExtendedPubKeyExtra>(&bch_coin, PASSWORD).unwrap();
        let json_str = keystore.json();
        let v: Value = serde_json::from_str(&json_str).unwrap();

        let active_accounts = v["activeAccounts"].as_array().unwrap();
        assert_eq!(1, active_accounts.len());
        let account = active_accounts.first().unwrap();
        let address = account["address"].as_str().unwrap();

        assert_eq!("bitcoincash:qqyta3mqzeaxe8hqcdsgpy4srwd4f0fc0gj0njf885", address);

        let path = account["derivationPath"].as_str().unwrap();
        assert_eq!(BIP_PATH, path);
        let coin = account["coin"].as_str().unwrap();
        assert_eq!("BCH", coin);

        let extra = account["extra"].as_object().expect("extra");
        let xpub = extra["xpub"].as_str().expect("xpub");
        assert_eq!("xpub6Bmkv3mmRZZWoFSBdj9vDMqR2PCPSP6DEj8u3bBuv44g3Ncnro6cPVqZAw6wTEcxHQuodkuJG4EmAinqrrRXGsN3HHnRRMtAvzfYTiBATV1", xpub)
    }


}
