use reqwest::{Client};
use anyhow::{Result};
use serde::{Deserialize, Serialize};
use serde_this_or_that::{as_f64};

pub async fn get_ticker(market: &str) -> Result<MarketTicker, anyhow::Error> {
    let response = Client::new().
        get(format!("https://api.binance.com/api/v3/ticker/24hr?symbol={market}")).
        send().await?;
    utils::check_status_code_and_deserialize(response).await
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketTicker {
    pub symbol: String,
    #[serde(deserialize_with = "as_f64")]
    pub price_change: f64,
    #[serde(deserialize_with = "as_f64")]
    pub price_change_percent: f64,
    #[serde(deserialize_with = "as_f64")]
    pub weighted_avg_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub prev_close_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub last_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub last_qty: f64,
    #[serde(deserialize_with = "as_f64")]
    pub bid_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub bid_qty: f64,
    #[serde(deserialize_with = "as_f64")]
    pub ask_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub ask_qty: f64,
    #[serde(deserialize_with = "as_f64")]
    pub open_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub high_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub low_price: f64,
    #[serde(deserialize_with = "as_f64")]
    pub volume: f64,
    #[serde(deserialize_with = "as_f64")]
    pub quote_volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,

}
//"{\"symbol\":\"BNBBTC\",\"priceChange\":\"-0.00003600\",\"priceChangePercent\":\"-0.521\",
// \"weightedAvgPrice\":\"0.00681968\",\"prevClosePrice\":\"0.00691700\",\"lastPrice\":\"0.00687900\",
// \"lastQty\":\"0.02600000\",\"bidPrice\":\"0.00687800\",\"bidQty\":\"0.73400000\",\"askPrice\":\"0.00687900\",
// \"askQty\":\"5.77400000\",\"openPrice\":\"0.00691500\",\"highPrice\":\"0.00694100\",\"lowPrice\":\"0.00672100\",
// \"volume\":\"17354.08500000\",\"quoteVolume\":\"118.34924314\",\"openTime\":1734774858410,
// \"closeTime\":1734861258410,\"firstId\":266019768,\"lastId\":266056593,\"count\":36826}"