use ethers::prelude::*; 
use ethers::providers::{Http, Provider};
 
 
use std::sync::Arc; 
 

use eyre::Result;


use evm_token_tools_rs::util::wallet_client::{WalletClient,WalletClientError};

//https://www.gakonst.com/ethers-rs/

#[tokio::main]
async fn main()   -> Result<(),WalletClientError> {
        
        
    abigen!(
        ERC20,
        "./src/abi/erc20.abi.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );
     
    let wallet_client = match WalletClient::from_env() {
        Ok(wc) => wc,
        Err( e ) =>  return Err( e )
        
    };
    
    //let provider = wallet_client.provider;
    //let wallet = wallet_client.wallet;
    let signer_middleware = Arc::clone(&wallet_client.signer_middleware);
   
       
    
    // specify the contract address
    //0xbtc on goerli 
    let contract_address = match "0xab89a7742cb10e7bce98540fd05c7d731839cf9f".parse::<Address>() {
        Ok(addr) => addr, 
        Err( .. ) => return Err(  WalletClientError::AddressParseError )
    };

    // Initialize contract
    //let contract = Contract::new(contract_address, contract_abi_str.parse()?, wallet);

    let token_contract = ERC20::new(contract_address, signer_middleware.clone());
    
    
    let signed_call = match token_contract.decimals(  ).call().await {
        Ok(call) => call,
        Err( .. ) => return Err( WalletClientError::ContractCallError )        
    }; 
    
    //let receipt = signed_call.await  ;
    
      
    println!("decimal result {} " , signed_call);
      
    
    
    Ok(())
}


 