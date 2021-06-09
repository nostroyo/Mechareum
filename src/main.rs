use crate::mecha::{Mecha, MechaColor};
use crate::mecha_collection::CachedMechaCollection;
use crate::ethBridge::ETHNFTContract;
use std::io::{stdin, Error, ErrorKind};
use std::str::FromStr;
use crate::backend_mecha_function::BackEndMechaFunction;
use std::io;
use std::num::ParseIntError;
use web3::types::U256;

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
        println!("How many mecha create ?");
        let read_value = read_u8_from_console()?;
        match read_value {
            Ok(value) => {nb_mecha = value; break;}
            Err(_) => {}
        }
    }

    for _ in 0..nb_mecha {
        let mut buffer = String::new();
        println!("mecha name ?");
        stdin().read_line(&mut buffer)?;
        mecha_NFT_contract.generate_new_mecha(String::from(buffer.trim_end()));
        let last_mecha_id = mecha_NFT_contract.get_total_mecha_owned() - 1;
        let mecha = mecha_NFT_contract.get_owned_mecha_by_index(last_mecha_id);
    }

    let user_mecha_list = CachedMechaCollection::new(mecha_NFT_contract);
    let mut i = 1;
    let mut mecha_list= Vec::new();
    for mecha in user_mecha_list {
        println!("{} Mecha info {}", i, mecha.info());
        mecha_list.push(mecha);
        i += 1;
    }

    let mut first_mecha = fill_fighting_mecha(& mecha_list)?;
    let mut second_mecha = fill_fighting_mecha(& mecha_list)?;


    loop {
        print!("Turn {} ", nbTurn);

        let fastest;
        let slowest;
        if first_mecha.speed >= second_mecha.speed {
            fastest = &mut first_mecha;
            slowest = &mut second_mecha;
        } else {
            fastest = &mut second_mecha;
            slowest = & mut first_mecha;
        }

        println!("{} attack", fastest.name);
        fastest.AttackOpponent(slowest);
        if CheckKO(slowest){
            break;
        }

        println!("{} attack", slowest.name);
        slowest.AttackOpponent(fastest);
        if CheckKO(fastest){
            break;
        }

        println!("stats: {}", first_mecha.info());
        println!("stats: {}", second_mecha.info());

        nbTurn += 1;
    }
    Ok(())
}

fn read_u8_from_console() -> io::Result<(Result<u8, ParseIntError>)> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    match u8::from_str(&buffer.trim_end())
    {
        Ok(nb) => {
            Ok(Ok(nb))

        }
        Err(e) => {
            println!("NAN");
            Ok(Err(e))
        }
    }
}

fn CheckKO(mecha: &Mecha) -> bool {
    if mecha.health == 0 {
        println!("{} KO", mecha.name);
        true
    } else {
        false
    }
}

fn fill_fighting_mecha(mecha_list: & Vec<Mecha>) -> io::Result<(Mecha)> {

    println!("select mecha number");

    let read_value = read_u8_from_console()?;
    let result = mecha_list.get(read_value.unwrap() as usize - 1);
    match result {
        None => Err(Error::new(ErrorKind::Other, "oh no!")),
        Some(m) => {Ok(m.clone())}
    }
}

