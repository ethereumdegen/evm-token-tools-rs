use ethers::prelude::*; 
use ethers::providers::{Http, Provider};
use ethers::utils::parse_units;
 
 
use std::sync::Arc; 
 

use eyre::Result;


use evm_token_tools_rs::util::wallet_client::{WalletClient,WalletClientError};

use clap::Parser;
use std::env;

use ethers::types::transaction::eip2930::AccessList;
 

use dotenv::dotenv;


/*


cargo run --bin erc20_approve -- --token-address 0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14 --approved-address 0x8DAE8766bC47376cc7c14cbAA0AeE511829F5dDe --amount 10000000000


*/



#[derive(Parser)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(long)]
    token_address: String,
    #[clap(long)]
    approved_address: String,
    #[clap(long)]
    amount: U256,
}


//https://www.gakonst.com/ethers-rs/

#[tokio::main]
async fn main()   -> Result<(),WalletClientError> {
        
     dotenv().ok();

    abigen!(
        ERC20,
        "./src/abi/erc20.abi.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );
     
    
    
    //let provider = wallet_client.provider;
    //let wallet = wallet_client.wallet;


    let wallet_client = match WalletClient::from_env() {
        Ok(wc) => wc,
        Err( e ) =>  return Err( e )
        
    };
    
    //let provider = wallet_client.provider;
    let wallet = wallet_client.wallet;


    let signer_middleware = Arc::clone(&wallet_client.signer_middleware);
   
       


    let opts: Opts = Opts::parse();
 
    let token_address = match opts.token_address.parse::<Address>() {
        Ok(addr) => addr,
        Err(..) => return Err(WalletClientError::AddressParseError),
    };

    let approved_address = match opts.approved_address.parse::<Address>() {
        Ok(addr) => addr,
        Err(..) => return Err(WalletClientError::AddressParseError),
    };

     let amount =   opts.amount ;  



   
    // Initialize contract
    //let contract = Contract::new(contract_address, contract_abi_str.parse()?, wallet);

    let token_contract = ERC20::new(token_address, signer_middleware.clone());
    
    
  


     let tx_data_action = token_contract.approve(
         approved_address , amount 
        ); 


    let tx_data_bytes = tx_data_action.calldata().unwrap();
    let data = Some( tx_data_bytes );


     let chain_id:u64 = env::var("CHAIN_ID").expect("Missing CHAIN_ID").parse().unwrap();
     
     
    let transaction:Eip1559TransactionRequest = Eip1559TransactionRequest {
        to: Some(NameOrAddress::Address(token_address)),
        data, 
        from: Some(wallet.address()),
        gas:  None,
        value: None,
        nonce: None,
        access_list: AccessList::default(),    
        max_priority_fee_per_gas: Some( U256::from( parse_units("10.0", "gwei").unwrap() )  ),
        max_fee_per_gas:Some( U256::from( parse_units("200.0", "gwei").unwrap() )  ),
        chain_id: Some(  chain_id.into() ),        
    };
    
    let pending_tx = signer_middleware.send_transaction(transaction, None).await;
       match pending_tx {
        Ok( tx ) =>  {
            println!("tx hash  {:?} " , tx.tx_hash())
             } ,
        Err(e) => {
            println!("{}", e.to_string());
            }
    }

    
    //let receipt = signed_call.await  ;
    
     
    
    
    Ok(())
}


 