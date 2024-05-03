
use ethers::prelude::*; 
use ethers::providers::{Http, Provider};
 
 
use k256::Secp256k1;
use k256::ecdsa::SigningKey;

use std::convert::TryFrom;

use std::sync::Arc;


use dotenv::dotenv;
use std::env;


use rustc_hex::FromHexError;


use std::fmt;
use std::error::Error as StdError;
 
use ethers::prelude::WalletError;
use url::ParseError;

#[derive(Debug)]
pub enum WalletClientError {
    EnvVarError(std::env::VarError),
    ProviderError(ethers::providers::ProviderError),
    WalletParsingError(WalletError),
    ParseUrlError( ParseError),
    FromHexError( FromHexError),
    AddressParseError,
    ContractCallError
}


impl From<std::env::VarError> for WalletClientError {
    fn from(err: std::env::VarError) -> WalletClientError {
        WalletClientError::EnvVarError(err)
    }
}

impl From<ethers::providers::ProviderError> for WalletClientError {
    fn from(err: ethers::providers::ProviderError) -> WalletClientError {
        WalletClientError::ProviderError(err)
    }
}

impl From<ParseError> for WalletClientError {
    fn from(err: ParseError) -> WalletClientError {
        WalletClientError::ParseUrlError(err)
    }
}

impl From<WalletError> for WalletClientError {
    fn from(err: WalletError) -> WalletClientError {
        WalletClientError::WalletParsingError(err)
    }
}
 
impl From<FromHexError> for WalletClientError {
    fn from(err: FromHexError) -> WalletClientError {
        WalletClientError::FromHexError(err)
    }
}
 
 
impl std::error::Error for WalletClientError {}


impl fmt::Display for WalletClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WalletClientError::FromHexError(err) => write!(f, "Hex parsing error: {}", err),
            
            WalletClientError::EnvVarError(err) => write!(f, "Environment variable error: {}", err),
            WalletClientError::ProviderError(err) => write!(f, "Provider error: {}", err),
            WalletClientError::ParseUrlError(err) => write!(f, "Url parsing error: {}", err),
            WalletClientError::WalletParsingError(err) => write!(f, "Wallet parsing error: {}", err),
            WalletClientError::AddressParseError => write!(f,"Address parse error "),
            WalletClientError::ContractCallError => write!(f,"contract call error")
        }
    }
}


pub struct WalletClient {
    
    pub provider: Provider<Http>  ,
    pub wallet: Wallet<SigningKey>,
    pub signer_middleware:  Arc< SignerMiddleware<Provider<Http> , Wallet<SigningKey>>  > 
    
    
}

impl WalletClient   {
    
    pub fn from_env( ) -> Result<Self,WalletClientError> {
            
        dotenv().ok();

        
         // 5. Use a private key to create a wallet
        // Do not include the private key in plain text in any production code
        // This is just for demonstration purposes
        // Do not include '0x' at the start of the private key
            
        
        // Fetch the RPC API endpoint and the private key from the environment variables
        let rpc_url = env::var("RPC_URL").expect("Missing RPC_API_ENDPOINT");
        let private_key = env::var("PRIVATE_KEY").expect("Missing PRIVATE_KEY");

        let chain_id:u64 = env::var("CHAIN_ID").expect("Missing CHAIN_ID").parse().unwrap();
    
        let provider = Provider::<Http>::try_from(rpc_url)?;
    
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id( chain_id  );
            
        let signer_middleware = Arc::new( SignerMiddleware::new(provider.clone(), wallet.clone()) ) ;
            
        Ok( Self {
            provider,
            wallet,
            signer_middleware 
        } )
        
        
    }
    
}