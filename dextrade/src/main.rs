mod config;
mod dex_error;

use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
// use solana_client::rpc_client::SerializableTransaction;
use solana_sdk::{transaction::VersionedTransaction};
use solana_sdk::signature::{ Signer, Keypair};
use tokio;
use jupiter_swap_api_client::quote::TokenResponse;

#[tokio::main]
async fn main() {
    let _err = match  _main().await {
        Ok(err) => err,
        Err(e) => {
            eprintln!("running error: {e}");
            return;
        }
    };
}

async fn _main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Trading on jupiter...");
    println!("Loading configuration");
    let conf = config::get_config()?;
    let keypair = Keypair::from_bytes(&conf.secret_key)?;
    println!("from address: [{}]",keypair.pubkey().to_string());

    let jupiter_client = JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".into());

    let tokens = jupiter_client.tokens().await?;
    let from_token = match find_token_address(conf.swap.from_symbol.clone(),&tokens) {
        Some(from_token) => from_token,
        //None => return dex_error::new("lookup currency".into(),format!("{} is not supported",conf.swap.from_symbol)),
        None => return Err(Box::new(dex_error::new("lookup currency".into(),format!("{} is not supported",conf.swap.from_symbol)))),
    };
    let to_token = match find_token_address(conf.swap.to_symbol.clone(),&tokens) {
        Some(from_token) => from_token,
        //None => return dex_error::new("lookup currency".into(),format!("{} is not supported",conf.swap.to_symbol)),
        None => return Err(Box::new(dex_error::new("lookup currency".into(),format!("{} is not supported",conf.swap.from_symbol))))
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
    let quote_response = jupiter_client.quote(&quote_request).await?;
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
        .await?;

    //println!("Raw tx: {:?}", swap_response.swap_transaction);

    let mut  versioned_transaction: VersionedTransaction =
        bincode::deserialize(&swap_response.swap_transaction)?;
    //versioned_transaction.get_recent_blockhash()

    //println!("{}", versioned_transaction.)

    // send with rpc client...
    let solana_node = RpcClient::new("https://api.mainnet-beta.solana.com".into());
    let recent_blockhash = solana_node.get_latest_blockhash().await?;
    versioned_transaction.message.set_recent_blockhash(recent_blockhash);

    println!("node blockhash: {}", recent_blockhash.to_string());

    // Replace with a keypair or other struct implementing signer
    // let null_signer = NullSigner::new(&keypair);
    let signed_versioned_transaction =
        VersionedTransaction::try_new(versioned_transaction.message, &[&keypair]).unwrap();

    println!("signed versioned transaction: {:?}",signed_versioned_transaction.verify_and_hash_message().unwrap());

    let client_result = solana_node
        .send_and_confirm_transaction(&signed_versioned_transaction)
        .await?;
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


    Ok(())
}

fn find_token_address(symbol: String, tokens: &Vec<TokenResponse>) -> Option<TokenResponse> {
    for token in tokens {
        if symbol == token.symbol {
            return Some(token.clone())
        }
    }
    None
}


