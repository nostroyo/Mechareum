extern crate web3;

use crate::mecha::{Mecha, BackendMechaCharacteristics};
use crate::backend_mecha_function::BackEndMechaFunction;

use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::types::{TransactionRequest, Address, U256};
use web3::transports::Http;


pub struct ETHNFTContract {
    mecha_contract_nft: Contract<Http>,
    player_address: Address,
}

impl ETHNFTContract {
    fn new(hexa_contract_address: String, hexa_player_address: String) -> Self {
        let (_, http) = web3::transports::Http::new(
            "http://localhost:8545"
        ).unwrap();

        let web3 = web3::Web3::new(http);
        let accounts = web3.eth().accounts().wait().unwrap();

        let contract_address: Address = hexa_contract_address.parse().unwrap();

        let contract = Contract::from_json(web3.eth(), contract_address, include_bytes!("contract/abi.json")).unwrap();

        ETHNFTContract {
            mecha_contract_nft: contract,
            player_address: hexa_contract_address.parse().unwrap()
        }
    }
}
impl BackEndMechaFunction for ETHNFTContract {

    fn get_mecha_characteristics_by_id(&self, id: U256) -> BackendMechaCharacteristics {

        let result = self.mecha_contract_nft.query(
            "MechasOwnership",
            (id,),
            self.player_address,
            Options::default(),
            None);

        result.wait().unwrap()
    }

    fn generate_new_mecha(&mut self, mecha_name: String) {

        let mint_options = Options::with(|opt| opt.gas = Some(3_000_000.into()));
        let tx_hash = self.mecha_contract_nft.call("mint", (), self.player_address, mint_options).wait().unwrap();
    }

    fn get_total_mecha_owned(&mut self) -> U256 {

        let result = self.mecha_contract_nft.query(
            "balanceOf",
            self.player_address,
            self.player_address,
            Options::default(),
            None);

        result.wait().unwrap()
    }

    fn get_owned_mecha_by_index(&mut self, id: U256) -> BackendMechaCharacteristics {

        let result = self.mecha_contract_nft.query(
            "tokenOfOwnerByIndex",
            (self.player_address, id),
            self.player_address,
            Options::default(),
            None);

        let global_index = result.wait().unwrap();

        self.get_mecha_characteristics_by_id(global_index)

    }
}

fn GetMecha() {



}

#[cfg(test)]
mod tests {
    use crate::ethBridge::GetMecha;

    #[test]
    fn test_call() {
        GetMecha();
    }

}