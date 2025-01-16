use solana_client::nonblocking::rpc_client::RpcClient;
use jupiter_swap_api_client::quote::TokenResponse;
use crate::config::Config;
use crate::dex_error::DexError;
use crate::binance;

pub struct Runner {
    cfg: Config,
    from_token: TokenResponse,
    to_token: TokenResponse,
    sol_node: RpcClient,
}

impl Runner {
    pub fn new(cfg: Config, from_token: TokenResponse, to_token: TokenResponse) -> Runner {
        let sol_node = RpcClient::new("https://api.mainnet-beta.solana.com".into());
        Runner { cfg, from_token, to_token, sol_node }
    }
    pub async fn run(&self) -> Result<(), DexError> {
        let ticker = binance::get_ticker("BNBBTC").await;
        println!("{:#?}", ticker);
        self.submit_order(0.5)?;
        println!("Order success!");
        Ok(())
    }
    pub fn submit_order(&self, amount: f64) -> Result<(), DexError> {
        Ok(())
    }
}