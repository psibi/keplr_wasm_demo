use std::fmt::format;

use anyhow::{anyhow, Context};
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, Response};

#[wasm_bindgen(start)]
async fn run() {
    console_error_panic_hook::set_once();
    let keplr = Keplr::new();
    if keplr.is_err() {
        console::log_1(&"keplr not installed".into());
    } else {
        console::log_1(&"keplr installed".into());
        let keplr = keplr.unwrap();
        keplr.enable_keplr("osmo-test-5").await;
        let accounts = keplr.get_accounts("osmo-test-5").await.unwrap();
        let network = Network::OsmosisTestnet;
        let balance = network.get_balance(accounts[0].clone()).await.unwrap();
        console::log_1(&format!("Balance: {balance:#?}").into());
    }
}

pub struct Keplr(Object);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WalletAddress {
    pub address: String,
    pub algo: String,
    pub pubkey: Vec<u8>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Balance {
    pub denom: String,
    pub amount: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct BalanceResponse {
    pub balances: Vec<Balance>,
}

enum Network {
    OsmosisTestnet,
}

impl Network {
    fn get_lcd(&self) -> &str {
        match self {
            Network::OsmosisTestnet => "https://lcd.osmotest5.osmosis.zone",
        }
    }

    pub async fn get_balance(&self, address: WalletAddress) -> anyhow::Result<Vec<Balance>> {
        let url = format!(
            "{}/cosmos/bank/v1beta1/balances/{}?pagination.limit=1000",
            self.get_lcd(),
            address.address
        );
        let response = reqwest::get(&url).await?.text().await?;
        let json = js_sys::JSON::parse(&response).map_err(|err| anyhow!("{err:?}"))?;
        let json: BalanceResponse =
            serde_wasm_bindgen::from_value(json).map_err(|err| anyhow!("{err:?}"))?;
        Ok(json.balances)
    }
}

impl Keplr {
    /// Gets keplr object
    fn new() -> anyhow::Result<Keplr> {
        let window = web_sys::window().context("No window object found")?;
        let keplr = window
            .get("keplr")
            .map(Keplr)
            .context("Kelpr object not found")?;
        Ok(keplr)
    }

    pub async fn enable_keplr(&self, chain_id: &str) {
        enable_keplr(chain_id).await;
    }

    pub async fn get_accounts(&self, chain_id: &str) -> anyhow::Result<Vec<WalletAddress>> {
        let value = get_accounts(chain_id).await;
        let json = serde_wasm_bindgen::from_value(value).map_err(|err| anyhow!("{err}"))?;
        Ok(json)
    }
}

#[wasm_bindgen(module = "/keplr.js")]
extern "C" {
    // https://github.com/chainapsis/keplr-wallet/blob/3823e983845b318cebddc148675418047e7bd2d3/packages/types/src/wallet/keplr.ts#L75
    async fn enable_keplr(chain_id: &str);

    async fn get_accounts(chain_id: &str) -> JsValue;
}
