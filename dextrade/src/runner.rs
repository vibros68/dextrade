use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use std::str::FromStr;
//use spl_token::state::Account;
use crate::config::Config;
use crate::dex_error::DexError;
use crate::{binance, dex_error};
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    quote::TokenResponse,
    JupiterSwapApiClient,
};

pub struct Runner {
    cfg: Config,
    from_token: TokenResponse,
    to_token: TokenResponse,
    sol_node: RpcClient,
    jup_client: JupiterSwapApiClient,
}

impl Runner {
    pub fn new(cfg: Config, jup_client: JupiterSwapApiClient, from_token: TokenResponse, to_token: TokenResponse) -> Runner {
        let sol_node = RpcClient::new("https://api.mainnet-beta.solana.com".into());
        Runner { cfg, from_token, to_token, sol_node, jup_client }
    }
    pub async fn run(&self) -> Result<(), DexError> {
        let keypair = match Keypair::from_bytes(&self.cfg.secret_key) {
            Ok(keypair) => keypair,
            Err(err) => {
                return Err(dex_error::new("keypair".into(), format!("{:?}", err)))
            }
        };
        println!("{}");
        println!("{}",self.from_token.symbol);
        println!("{}",self.from_token.decimals);
        println!("{}",self.to_token.decimals);
        let binance_rate = self.get_rate().await?;
        println!("{:#?}", binance_rate);

        let balance = self.sol_node.get_balance(&keypair.pubkey()).await;
        println!("balance: {:?}",balance);
        let token_address = self.from_token.address;
        let from_data = self.sol_node.get_account_data(&token_address).await;
        let from_account: Account = Account::unpack_from_slice(&from_data).unwrap();
        println!("from_balance: {:?}",from_account.balance);
        let quote_request = QuoteRequest {
            amount: 100000000000,
            input_mint: self.from_token.address,
            output_mint: self.to_token.address,
            slippage_bps: 50,
            ..QuoteRequest::default()
        };
        let quote_response = match self.jup_client.quote(&quote_request).await {
            Ok(quote_response) => quote_response,
            Err(err) => {
                return Err(dex_error::new("quote failed".into(), err.to_string()))
            },
        };
        println!("{:?}",quote_response);

        println!("from address: [{}]",keypair.pubkey().to_string());

        let swap_response = match self.jup_client
            .swap(&SwapRequest {
                user_public_key: keypair.pubkey(),
                quote_response: quote_response.clone(),
                config: TransactionConfig::default(),
            })
            .await {
            Ok(swap_response) => swap_response,
            Err(err) => {
                return Err(dex_error::new("swap failed".into(), err.to_string()))
            }
        };
        self.submit_order(0.5)?;
        println!("Order success!");
        Ok(())
    }
    pub async fn get_rate(&self) -> Result<f64, DexError> {
        let from_ticker = match binance::get_ticker(format!("{}USDT", self.from_token.symbol).as_str()).await {
            Ok(ticker) => ticker,
            Err(err) => {
                return Err(dex_error::new("fetch from ticker".into(),err.to_string()));
            }
        };
        let to_ticker = match binance::get_ticker(format!("{}USDT", self.to_token.symbol).as_str()).await {
            Ok(ticker) => ticker,
            Err(err) => {
                return Err(dex_error::new("fetch from ticker".into(),err.to_string()));
            }
        };
        Ok(from_ticker.last_price / to_ticker.last_price)
    }
    pub fn submit_order(&self, amount: f64) -> Result<(), DexError> {
        Ok(())
    }
}