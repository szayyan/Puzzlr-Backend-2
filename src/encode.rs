
//use ethers::{ signers, abi };
//use ethers::utils::{ hex };

/*  Ethers library unsupported on windows */
// TODO finish implementing validator signature
pub fn get_signature(address: &str, epoch: usize) -> &'static str {

    // TODO
    // should really use env! macro for compile time error if key is missing  
    let secret_key = std::env::var( "VALIDATOR_SECRET_KEY" ).unwrap();

    let signature = "";
    return signature
}