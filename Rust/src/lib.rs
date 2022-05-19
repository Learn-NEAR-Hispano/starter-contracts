use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc, Promise};

fn one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
}

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
    #[payable]
    pub fn set_participante(&mut self, nombre: String, edad: u64) {
        let cuenta = env::signer_account_id();
        let deposito = env::attached_deposit();

        assert!(edad > 0, "Edad inválida.");
        assert!(
            nombre.len() >= 3,
            "El nombre debe contener 3 o más caractéres."
        );
        assert!(
            deposito > one_near(),
            "Debes de pagar 1 NEAR para registrarte."
        );

        let participante = Participante::new(String::from(&cuenta), String::from(&nombre), edad);

        self.participantes.insert(&cuenta, &participante);

        env::log(format!("Registro creado exitosamente.").as_bytes());
    }

    pub fn get_participante(&self, cuenta: String) -> Option<Participante> {
        self.participantes.get(&cuenta)
    }

    pub fn get_participantes(&self) -> Vec<(String, Participante)> {
        self.participantes.to_vec()
    }

    pub fn set_certificado(&mut self, cuenta: String) -> bool {
        assert!(
            env::signer_account_id() == "aklassen.testnet",
            "No tienes permisos para ejecutar este comando."
        );

        match self.participantes.get(&cuenta) {
            Some(mut participante) => {
                participante.certificado = true;

                Promise::new(String::from(&cuenta)).transfer(5 as u128);
                self.participantes.insert(&cuenta, &participante);

                env::log(format!("Participante certificado. El participante ha recibido su recompensa de 5 NEAR.").as_bytes());

                true
            }
            None => {
                env::log(format!("Participante no encontrado.").as_bytes());
                false
            }
        }
    }
}
