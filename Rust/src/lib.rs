use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NcdContract {
    participantes: UnorderedMap<String, Participante>,
}

impl Default for NcdContract {
    fn default() -> Self {
        Self {
            participantes: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Participante {
    pub cuenta: String,
    pub nombre: String,
    pub edad: u64,
    pub certificado: bool,
}

impl Default for Participante {
    fn default() -> Self {
        Participante {
            cuenta: String::from(""),
            nombre: String::from(""),
            edad: 0,
            certificado: false,
        }
    }
}

impl Participante {
    pub fn new(cuenta: String, nombre: String, edad: u64) -> Self {
        Self {
            cuenta,
            nombre,
            edad,
            certificado: false,
        }
    }
}

#[near_bindgen]
impl NcdContract {
    pub fn set_participante(&mut self, cuenta: String, nombre: String, edad: u64) {
        assert!(edad > 0, "No puedes tener 0 años.");

        let participante = Participante::new(String::from(&cuenta), String::from(&nombre), edad);

        self.participantes.insert(&cuenta, &participante);

        env::log(format!("Se creó el registro exitosamente (:").as_bytes());
    }

    pub fn get_participante(&self, cuenta: String) -> Option<Participante> {
        self.participantes.get(&cuenta)
    }
}
