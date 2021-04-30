extern crate web3;

use crate::mecha;

use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};


fn GetMecha() {
    let (_eloop, http) = web3::transports::Http::new(
        "http://localhost:8545"
    ).unwrap();
    let my_account: Address = "0xF2b2208cecb42a55Fd328E871B2d04C85e91Bd5E".parse().unwrap();
    let web3 = web3::Web3::new(http);

    let address: Address = "0x265eD92136a8D7AC96f7Eb133222EADCD92cB628".parse().unwrap();

    let contract = Contract::from_json(web3.eth(), address,include_bytes!("./abi.json")).unwrap();

    //let result = contract.call("mint", (my_account,), my_account, Options::default());
    let result = contract.query("balanceOf", (my_account,), None, Options::default(), None);
    let mecha: U256 = result.wait().unwrap();
    assert_eq!(mecha, 0.into());
}

#[cfg(test)]
mod tests {
    use crate::ethBridge::GetMecha;

    #[test]
    fn test_call() {
        GetMecha();
    }

}