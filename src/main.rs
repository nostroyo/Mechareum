use crate::mecha::{Mecha, MechaColor};
use crate::mecha_collection::CachedMechaCollection;
use crate::ethBridge::ETHNFTContract;
use std::io::{stdin, Error};
use std::str::FromStr;
use crate::backend_mecha_function::BackEndMechaFunction;
use std::io;

mod mecha;
mod mecha_state;
mod ethBridge;
mod mecha_collection;
mod backend_mecha_function;
mod mock_backend_function;

const PLAYER_ADDRESS: &str = "F2b2208cecb42a55Fd328E871B2d04C85e91Bd5E";
const MECHA_CONTRACT_ADDRESS: &str = "DA55220355db9762fF10281C6E6946f15E29AAa0";

fn main() -> io::Result<()> {
    let mut nbTurn = 1;

    let mut mecha_NFT_contract = ETHNFTContract::new(
        MECHA_CONTRACT_ADDRESS.to_string(),
        PLAYER_ADDRESS.to_string()
    );

    let mut nb_mecha;
    loop {
        let mut buffer = String::new();
        println!("How many mecha create ?");
        stdin().read_line(&mut buffer)?;
        match u8::from_str(&buffer.trim_end()) {
            Ok(nb) => {
                nb_mecha = nb;
                break;
            }
            Err(_) => { println!("NAN") }
        }
    }

    for i in 0..nb_mecha {
        let mut buffer = String::new();
        println!("mecha name ?");
        stdin().read_line(&mut buffer)?;
        mecha_NFT_contract.generate_new_mecha(String::from(buffer.trim_end()));
        let last_mecha_id = mecha_NFT_contract.get_total_mecha_owned() - 1;
        let mecha = mecha_NFT_contract.get_owned_mecha_by_index(last_mecha_id);

        println!("Mecha char {:?}", mecha);
    }

    let mut user_mecha_list= CachedMechaCollection::new(mecha_NFT_contract);
    for mecha in user_mecha_list {
        println!("Mecha info {}", mecha.info());
    }

    Ok(())
    //
    //
    //
    //
    //
    //
    //
    // let mut battling_mecha = [
    //     Mecha::new(String::from("YTH"), 25, 1, 1, MechaColor::Red),
    //     Mecha::new(String::from("CPU"), 2, 2, 2, MechaColor::Blue),
    // ];
    //
    // //let mut user_mecha_list: Option<CachedMechaCollection> = CachedMechaCollection::new ;
    //
    //
    //
    // loop {
    //     print!("Turn {} ", nbTurn);
    //
    //     if let Some((First, Elements)) = battling_mecha.split_first_mut() {
    //
    //         if let Some(Second) = Elements.get_mut(0) {
    //
    //             let fastest;
    //             let slowest;
    //             if First.speed >= Second.speed {
    //                 fastest = First;
    //                 slowest = Second;
    //             } else {
    //                 fastest = Second;
    //                 slowest = First;
    //             }
    //
    //             println!("{} attack", fastest.name);
    //             fastest.AttackOpponent(slowest);
    //             if CheckKO(slowest){
    //                 break;
    //             }
    //
    //             println!("{} attack", slowest.name);
    //             slowest.AttackOpponent(fastest);
    //             if CheckKO(fastest){
    //                 break;
    //             }
    //         }
    //     }
    //     print_info(&battling_mecha);
    //
    //     nbTurn += 1;
    // }
}

fn CheckKO(mecha: &Mecha) -> bool {
    if mecha.health == 0 {
        println!("{} KO", mecha.name);
        true
    } else {
        false
    }
}

fn print_info(battling_mecha: &[Mecha]) {
    for mecha in battling_mecha {
        println!("stats: {}", mecha.info());
        println!("stats: {}", mecha.info());
    }

}

