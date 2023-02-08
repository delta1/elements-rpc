mod image;

use elements::{Address, BlockHash};
use serde::{de::DeserializeOwned, Deserialize};

#[jsonrpc_client::api]
pub trait Rpc {
    // blockchain
    async fn getbestblockhash(&self) -> BlockHash;
    async fn getblockcount(&self) -> u64;
    // control
    // generating
    async fn generatetoaddress(
        &self,
        nblocks: usize,
        address: &str,
        maxtries: Option<u64>,
    ) -> Vec<BlockHash>;
    async fn sendtoaddress(
        &self,
        address: &str,
        amount: f64,
        comment: Option<&str>,
        comment_to: Option<&str>,
        subtractfeefromamount: Option<bool>,
        replaceable: Option<bool>,
        conf_target: Option<u64>,
        estimate_mode: Option<&str>,
        avoid_reuse: Option<bool>,
        assetlabel: Option<&str>,
        ignoreblindfail: Option<bool>,
        fee_rate: Option<f64>,
        verbose: Option<bool>,
    ) -> SendToAddress;
    // mining
    // network
    // raw transactions
    // signer
    // util
    async fn estimatesmartfee(&self, conf_target: u64, estimate_mode: &str) -> EstimateSmartFee;
    // wallet
    async fn createwallet(
        &self,
        wallet_name: &str,
        disable_private_keys: Option<bool>,
        blank: Option<bool>,
        passphrase: Option<&str>,
        avoid_reuse: Option<bool>,
        descriptors: Option<bool>,
        load_on_startup: Option<bool>,
        external_signer: Option<bool>,
    ) -> CreateWallet;
    async fn getaddressinfo(&self, address: &str) -> AddressInfo;
    async fn getnewaddress(&self, label: Option<&str>, address_type: Option<&str>) -> String;
    async fn getwalletinfo(&self) -> WalletInfo;
    // zmq
}

#[derive(Debug, Deserialize)]
pub struct CreateWallet {
    name: String,
    warning: String,
}

#[derive(Debug, Deserialize)]
pub struct EstimateSmartFee {
    feerate: Option<f64>,
    errors: Option<Vec<String>>,
    blocks: u64,
}

#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    address: String,
    #[serde(rename = "scriptPubKey")]
    script_pubkey: String,
    ismine: bool,
    iswatchonly: bool,
    solvable: bool,
    desc: Option<String>,
    parent_desc: Option<String>,
    isscript: bool,
    ischange: bool,
    iswitness: bool,
    witness_version: Option<u64>,
    witness_program: Option<String>,
    script: Option<String>,
    hex: Option<String>,
    pubkeys: Option<Vec<String>>,
    sigsrequired: Option<u64>,
    pubkey: Option<String>,
    // embedded:
    iscompressed: Option<bool>,
    confidential_key: String,
    unconfidential: String,
    confidential: String,
    timestamp: u64,
    hdkeypath: Option<String>,
    hdseedid: Option<String>,
    hdmasterfingerprint: Option<String>,
    labels: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendToAddress {
    txid: String,
    fee_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct WalletInfo {
    walletname: String,
    walletversion: u64,
    format: String,
    balance: f64,
    unconfirmed_balance: f64,
    immature_balance: f64,
    txcount: u64,
    keypoololdest: u64,
    keypoolsize: u64,
    keypoolsize_hd_internal: u64,
    unlocked_until: Option<u64>,
    paytxfee: f64,
    hdseedid: Option<String>,
    private_keys_enabled: bool,
    avoid_reuse: bool,
    // scanning: Scanning,
    descriptors: bool,
}

#[jsonrpc_client::implement(Rpc)]
pub struct Client {
    inner: reqwest::Client,
    base_url: reqwest::Url,
}

impl Client {
    pub fn new(base_url: String) -> Result<Self, url::ParseError> {
        Ok(Self {
            inner: reqwest::Client::new(),
            base_url: base_url.parse()?,
        })
    }
}

#[cfg(test)]
mod test {
    use testcontainers::clients;

    use super::*;

    #[tokio::test]
    async fn simple() {
        let docker = clients::Cli::default();
        let container = docker.run(image::Elementsd::default());
        let port = container.get_host_port_ipv4(18444);

        let client = Client::new(format!("http://user:pass@127.0.0.1:{}", port)).unwrap();

        let wallet_name = "asdf".to_string();
        let wallet = client
            .createwallet(&wallet_name, None, None, None, None, None, None, None)
            .await
            .unwrap();
        dbg!(&wallet);
        assert_eq!(wallet.name, wallet_name);
        assert!(wallet.warning.is_empty());

        let address = client.getnewaddress(None, None).await.unwrap();
        dbg!(address);
        let address = client.getnewaddress(None, Some("legacy")).await.unwrap();
        dbg!(address);
        let address = client
            .getnewaddress(None, Some("p2sh-segwit"))
            .await
            .unwrap();
        dbg!(address);
        let address = client.getnewaddress(None, Some("bech32")).await.unwrap();
        dbg!(&address);

        let hash = client.getbestblockhash().await.unwrap();
        dbg!(hash);

        let count = client.getblockcount().await.unwrap();
        dbg!(count);
        assert_eq!(count, 0);

        let nblocks = 501;
        let hashes = client
            .generatetoaddress(nblocks, &address, None)
            .await
            .unwrap();
        dbg!(&hashes);
        assert_eq!(hashes.len(), nblocks);

        let info = client.getwalletinfo().await.unwrap();
        dbg!(&info);

        let sent = client
            .sendtoaddress(
                &address,
                0.1,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(true),
            )
            .await
            .unwrap();
        dbg!(&sent);

        let info = client.getaddressinfo(&address).await.unwrap();
        dbg!(&info);

        let temp = client.estimatesmartfee(1, "conservative").await.unwrap();
        dbg!(&temp);
    }
}
