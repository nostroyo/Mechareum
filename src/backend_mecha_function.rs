use web3::types::U256;
use crate::mecha::BackendMechaCharacteristics;

pub trait BackEndMechaFunction {

    fn get_mecha_characteristics_by_id(&self, id: U256) -> BackendMechaCharacteristics;

    fn generate_new_mecha(&mut self, mecha_name: String);

    fn get_total_mecha_owned(&mut self) -> U256;

    fn get_owned_mecha_by_index(&mut self, id: U256) -> BackendMechaCharacteristics;
}

