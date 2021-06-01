extern crate web3;

use crate::mecha::{Mecha, BackendMechaCharacteristics};
use crate::backend_mecha_function::BackEndMechaFunction;

use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::types::{TransactionRequest, Address, U256};
use web3::transports::{Http, EventLoopHandle};



pub struct ETHNFTContract {
    mecha_contract_nft: Contract<Http>,
    player_address: Address,
    http_event_loop: EventLoopHandle
}

impl ETHNFTContract {
    pub(crate) fn new(hexa_contract_address: String, hexa_player_address: String) -> Self {
        let (eloop, http) = web3::transports::Http::new(
            "http://localhost:8545"
        ).unwrap();

        let web3 = web3::Web3::new(http);

        let contract_address: Address = hexa_contract_address.parse().unwrap();

        let contract = Contract::from_json(web3.eth(), contract_address, include_bytes!("contract/abi.json")).unwrap();

        ETHNFTContract {
            mecha_contract_nft: contract,
            player_address: hexa_player_address.parse().unwrap(),
            http_event_loop: eloop
        }
    }
}
impl BackEndMechaFunction for ETHNFTContract {

    fn get_mecha_characteristics_by_id(&self, id: U256) -> BackendMechaCharacteristics {//[(u8, u8, u8, u8, String); 1] {

        let result = self.mecha_contract_nft.query(
            "GetMechaById",
            (id,),
            self.player_address,
            Options::default(),
            None);

        result.wait().unwrap()
    }

    fn generate_new_mecha(&mut self, mecha_name: String) {

        let mint_options = Options::with(|opt| opt.gas = Some(3_000_000.into()));

        let tx_hash = self.mecha_contract_nft.call("mint", (mecha_name), self.player_address, mint_options).wait().unwrap();
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

#[cfg(test)]
mod tests {
    use crate::ethBridge::{ETHNFTContract};
    use crate::backend_mecha_function::BackEndMechaFunction;
    use web3::types::{U256};
    use super::web3::futures::Future;
    use super::web3::contract::Options;

    const PLAYER_ADDRESS: &str = "F2b2208cecb42a55Fd328E871B2d04C85e91Bd5E";
    const MECHA_CONTRACT_ADDRESS: &str = "DA55220355db9762fF10281C6E6946f15E29AAa0";

    fn setup() -> ETHNFTContract {
        ETHNFTContract::new(
            MECHA_CONTRACT_ADDRESS.to_string(),
            PLAYER_ADDRESS.to_string()
        )
    }

    #[test]
    fn test_connexion() {
        let (_eloop, http) = web3::transports::Http::new(
            "http://localhost:8545"
        ).unwrap();

        let web3 = web3::Web3::new(http);
        let accounts = web3.eth().accounts().wait().unwrap();
        println!("Accounts: {:?}", accounts);
    }

    #[test]
    fn test_generate() {
        let mut mecha_NFT_contract = setup();

        mecha_NFT_contract.generate_new_mecha("toto".to_string());
    }

    #[test]
    fn test_get_balance_of() {

        let mut mecha_NFT_contract = setup();
        let nb_mecha = mecha_NFT_contract.get_total_mecha_owned();
        println!("nb mecha {}", nb_mecha);
        mecha_NFT_contract.generate_new_mecha("toto".to_string());

        assert_eq!(mecha_NFT_contract.get_total_mecha_owned(), U256::from(nb_mecha + 1));

    }
    #[test]
    fn char() {
        let mut mecha_NFT_contract = setup();
        let result = mecha_NFT_contract.mecha_contract_nft.query(
            "GetMechaById",
            (U256::from(1),),
            mecha_NFT_contract.player_address,
            Options::default(),
            None);

        let result: (u8, u8, u8, u8, String) = result.wait().unwrap();
        println!("{:?}", result);
    }

}