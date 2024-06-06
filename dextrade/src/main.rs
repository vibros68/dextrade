mod config;
mod dex_error;

use std::error::Error;

use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_client::SerializableTransaction;
use solana_sdk::{transaction::VersionedTransaction};
use solana_sdk::signature::{ Signer, Keypair};
use tokio;
use jupiter_swap_api_client::quote::TokenResponse;
use crate::dex_error::DexError;

//const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
//const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

//pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm"); // Coinbase 2 wallet

#[tokio::main]
async fn main() {
    _main().await;
    //Keypair::from_bytes(&secret_key).unwrap();
    //return;
    /*let secret_key: [u8; 64] = [
        175,207,115,148,22,31,66,140,182,167,243,203,253,82,179,125,78,161,49,160,234,47,212,131,
        249,189,21,86,50,129,228,63,13,179,4,180,13,235,209,87,163,248,85,80,178,163,36,82,80,8,
        20,103,80,95,135,54,217,170,96,231,156,122,8,170
    ];
    let keypair = Keypair::from_bytes(&secret_key).unwrap();


    let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
    println!("Using base url: {}", api_base_url);

    let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

    let quote_request = QuoteRequest {
        amount: 5_000_000,
        input_mint: NATIVE_MINT,
        output_mint: USDC_MINT,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    // GET /quote
    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();
    println!("{quote_response:#?}");

    // POST /swap
    let swap_response = jupiter_swap_api_client
        .swap(&SwapRequest {
            user_public_key: keypair.pubkey(),
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();

    println!("Raw tx len: {}", swap_response.swap_transaction.len());
    //////////////////////////////////////////////////////////////////

    let mut  versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction).unwrap();
    //versioned_transaction.get_recent_blockhash()

    // send with rpc client...
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());
    let res = rpc_client.get_latest_blockhash().await.unwrap();
    versioned_transaction.message.set_recent_blockhash(res);

    println!("node blockhash: {}", res.to_string());

    // versioned_transaction.

    // Replace with a keypair or other struct implementing signer
    // let null_signer = NullSigner::new(&keypair);
    let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[&keypair]).unwrap();

    println!("{:?}",signed_versioned_transaction);

    let bh = signed_versioned_transaction.get_recent_blockhash();
    println!("tx blockhash: {}", bh.to_string());



    // This will fail with "Transaction signature verification failure" as we did not really sign
    let err = rpc_client
        .send_and_confirm_transaction(&signed_versioned_transaction)
        .await
        .unwrap_err();
    println!("{err}");
    // let signature = match res {
    //     Ok(signature) =>  signature,
    //     Err(error) => panic!("error: {:?}", error)
    // };
    // println!("{:?}", signature);

    //rpc_client.send

    // POST /swap-instructions
    let swap_instructions = jupiter_swap_api_client
        .swap_instructions(&SwapRequest {
            user_public_key: keypair.pubkey(),
            quote_response,
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();
    println!("swap_instructions: {:?}",swap_instructions.token_ledger_instruction);*/
}

async fn _main() -> DexError {
    println!("Trading on jupiter...");
    println!("Loading configuration");
    let conf = config::get_config().unwrap();
    let keypair = match Keypair::from_bytes(&conf.secret_key) {
        Ok(k) => k,
        Err(e) => return dex_error::new("".into(),e.to_string()),
    };
    println!("from address: [{}]",keypair.pubkey().to_string());

    let jupiter_client = JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".into());

    let tokens = jupiter_client.tokens().await.unwrap();
    let from_token = match find_token_address(conf.swap.from_symbol.clone(),&tokens) {
        Some(from_token) => from_token,
        None => return dex_error::new("lookup currency".into(),format!("{} is not supported",conf.swap.from_symbol)),
    };
    let to_token = match find_token_address(conf.swap.to_symbol.clone(),&tokens) {
        Some(from_token) => from_token,
        None => return dex_error::new("lookup currency".into(),format!("{} is not supported",conf.swap.to_symbol)),
    };

    let dec = u64::pow(10,from_token.decimals as u32);
    let from_amount = conf.swap.from_amount *(dec as f64);
    let quote_request = QuoteRequest {
        amount: from_amount as u64,
        input_mint: from_token.address,
        output_mint: to_token.address,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };
    let to_dec = u64::pow(10, to_token.decimals as u32);
    let quote_response = jupiter_client.quote(&quote_request).await.unwrap();
    let from_dec = u64::pow(10, from_token.decimals as u32);
    let from_amount = from_amount as f64 / from_dec as f64;
    let to_amount = quote_response.out_amount as f64 / to_dec as f64;
    println!("trade {from_amount} ${} to {to_amount} ${}, rate {:.6}",
             from_token.symbol, to_token.symbol, to_amount/conf.swap.from_amount);

    let swap_response = jupiter_client
        .swap(&SwapRequest {
            user_public_key: keypair.pubkey(),
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await
        .unwrap();

    //println!("Raw tx: {:?}", swap_response.swap_transaction);

    let mut  versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction).unwrap();
    //versioned_transaction.get_recent_blockhash()

    //println!("{}", versioned_transaction.)

    // send with rpc client...
    let solana_node = RpcClient::new("https://api.mainnet-beta.solana.com".into());
    let recent_blockhash = solana_node.get_latest_blockhash().await.unwrap();
    versioned_transaction.message.set_recent_blockhash(recent_blockhash);

    println!("node blockhash: {}", recent_blockhash.to_string());

    // Replace with a keypair or other struct implementing signer
    // let null_signer = NullSigner::new(&keypair);
    let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[&keypair]).unwrap();

    println!("signed versioned transaction: {:?}",signed_versioned_transaction.verify_and_hash_message().unwrap());

    let client_result = solana_node
        .send_and_confirm_transaction(&signed_versioned_transaction)
        .await
        .unwrap();
    println!("transaction id: [{}]", client_result.to_string());
    // let signature = match res {
    //     Ok(signature) =>  signature,
    //     Err(error) => panic!("error: {:?}", error)
    // };
    // println!("{:?}", signature);

    //rpc_client.send

    // POST /swap-instructions
    // let swap_instructions = jupiter_client
    //     .swap_instructions(&SwapRequest {
    //         user_public_key: keypair.pubkey(),
    //         quote_response,
    //         config: TransactionConfig::default(),
    //     })
    //     .await
    //     .unwrap();
    // println!("swap_instructions: {:?}",swap_instructions.token_ledger_instruction);


    dex_error::new("".into(),"".into())
}

fn find_token_address(symbol: String, tokens: &Vec<TokenResponse>) -> Option<TokenResponse> {
    for token in tokens {
        if symbol == token.symbol {
            return Some(token.clone())
        }
    }
    None
}


