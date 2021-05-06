extern crate web3;

use crate::mecha;

use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::types::{TransactionRequest, Address, U256};


fn GetMecha() {
    let (_eloop, http) = web3::transports::Http::new(
        "http://localhost:8545"
    ).unwrap();

    let web3 = web3::Web3::new(http);
    let accounts = web3.eth().accounts().wait().unwrap();

    let address: Address = "265eD92136a8D7AC96f7Eb133222EADCD92cB628".parse().unwrap();

    let contract = Contract::from_json(web3.eth(), address,include_bytes!("contract/abi.json")).unwrap();

    //let result = contract.call("mint", (my_account,), my_account, Options::default());
    let result = contract.query("MechasOwnership", (1_u8,), accounts[0], Options::default(), None);
    let characteristics: (u8, u8, u8, u8, String) = result.wait().unwrap();
    println!("{}", characteristics.4);

    let mint_options = Options::with(|opt| opt.gas = Some(3_000_000.into()));
    let tx_hash = contract.call("mint", (), accounts[0], mint_options).wait().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::ethBridge::GetMecha;

    #[test]
    fn test_call() {
        GetMecha();
    }

}