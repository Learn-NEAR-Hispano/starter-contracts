use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};

//Función que nos regresa el valor de 1 NEAR en un u128
fn one_near() -> u128 {
    u128::from_str_radix("1000000000000000000000000", 10).unwrap()
}

//Definimos el struct principal.
//Si nuestro contrato necesitara más colecciones, estas se definen aquí.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NcdContract {
    participantes: UnorderedMap<String, Participante>,
}

//E inicializamos el contrato por default
impl Default for NcdContract {
    fn default() -> Self {
        Self {
            //Inicializamos la colección con un prefijo único
            participantes: UnorderedMap::new(b"p".to_vec()),
        }
    }
}

//Definimos los structs que utilizaremos dentro del contrato
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Participante {
    pub cuenta: String,
    pub nombre: String,
    pub edad: u64,
    pub certificado: bool,
}

//En este contrato no se utiliza el default, pero es buena práctica tenerlo inicializado.
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

//Creamos la implementación del método new. El equivalente en AS sería el constructor.
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

//Igual que con el struct de Participante, implementamos los métodos del contrato en un impl.
#[near_bindgen]
impl NcdContract {
    /// Método de ESCRITURA para registrar un nuevo participante
    /// El comando para utilizarlo en la terminal es:
    /// >> near call $CONTRATO set_participante '{"nombre":"NOMBRE","edad":18}' --accountId cuenta.near --amount 1
    ///  $CONTRATO es una variable que contiene el id de la cuenta del contrato
    /// @param nombre string que requiere el nombre del participante a registrar
    /// @param edad entero de 64 bits sin signo que requiere la edad del participante
    /// Es necesario enviarle 1 NEAR (o más) como pago a este método.
    #[payable]
    pub fn set_participante(&mut self, nombre: String, edad: u64) {
        let cuenta = env::signer_account_id().to_string();
        let deposito = env::attached_deposit();

        assert!(edad > 0, "Edad inválida.");
        assert!(
            nombre.len() >= 3,
            "El nombre debe contener 3 o más caractéres."
        );
        assert!(
            deposito >= one_near(),
            "Debes de pagar 1 NEAR para registrarte."
        );

        let participante = Participante::new(cuenta.clone(), String::from(&nombre), edad);

        self.participantes.insert(&cuenta, &participante);

        env::log_str("Registro creado exitosamente.");
    }

    /// Método de LECTURA que regresa un participante
    /// El comando para utilizarlo en la terminal es:
    /// >> near view $CONTRATO get_participante '{"cuenta":"CUENTA.NEAR"}'
    /// @param cuenta string que contiene la cuenta (key) del usuario a consultar
    /// @returns Option<Participante>
    pub fn get_participante(&self, cuenta: String) -> Option<Participante> {
        self.participantes.get(&cuenta)
    }

    /// Método de LECTURA que regresa toda la lista de participantes registrados
    /// El comando para utilizarlo en la terminal es:
    ///  >> near view $CONTRATO get_participantes '{}'
    /// @returns Vec<Participante> (vector de participantes)
    pub fn get_participantes(&self) -> Vec<Participante> {
        self.participantes.values_as_vector().to_vec()
    }

    /// Método de ESCRITURA para certificar a un participante
    /// Además, transfiere 5 NEAR como compensación al participante que se haya certificado.
    /// El comando para utilizarlo en la terminal es:
    ///  >> near call $CONTRATO set_certificado '{"cuenta":"cuenta.near"}' --accountId cuenta.near --amount 1
    ///
    /// @param cuenta string que contiene la cuenta del participante a certificar
    /// @returns bool: Regresa verdadero o falso dependiendo de si se ejecutó la acción.
    pub fn set_certificado(&mut self, cuenta: String) -> bool {
        let master: AccountId = "aklassen.testnet".parse().unwrap();

        assert!(
            env::signer_account_id() == master,
            "No tienes permisos para ejecutar este comando."
        );

        let cuenta_near: AccountId = cuenta.parse().unwrap();

        match self.participantes.get(&cuenta) {
            Some(mut participante) => {
                participante.certificado = true;

                Promise::new(cuenta_near).transfer(5 as u128);
                self.participantes.insert(&cuenta, &participante);

                env::log_str("Participante certificado. El participante ha recibido su recompensa de 5 NEAR.");

                true
            }
            None => {
                env::log_str("Participante no encontrado.");
                false
            }
        }
    }
}

// PRUEBAS UNITARIAS
// Para correr las pruebas unitarias ejecuta el comando: cargo test
// Puedes encontrar más información en: https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    use super::*;

    const CUENTA: &str = "participante.testnet";
    const NOMBRE: &str = "Participante";
    const EDAD: u64 = 18;

    fn set_context() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.build());

        testing_env!(context
            .attached_deposit(one_near())
            .signer_account_id(CUENTA.parse().unwrap())
            .build());
    }

    #[test]
    pub fn test_set_participante() {
        set_context();
        let mut contrato = NcdContract::default();

        contrato.set_participante(String::from(NOMBRE), EDAD);

        let p = contrato.participantes.get(&String::from(CUENTA)).unwrap();

        assert_eq!(p.cuenta, CUENTA.to_string());
        assert_eq!(p.nombre, NOMBRE);
        assert_eq!(p.edad, EDAD);
        assert_eq!(p.certificado, false);
    }

    #[test]
    #[should_panic(expected = "Edad inválida.")]
    pub fn test_set_participante_edad() {
        set_context();
        let mut contrato = NcdContract::default();

        contrato.set_participante(String::from(NOMBRE), 0);
    }

    #[test]
    #[should_panic(expected = "El nombre debe contener 3 o más caractéres.")]
    pub fn test_set_participante_nombre() {
        set_context();
        let mut contrato = NcdContract::default();

        contrato.set_participante(String::from("p"), EDAD);
    }

    #[test]
    #[should_panic(expected = "Debes de pagar 1 NEAR para registrarte.")]
    pub fn test_set_participante_monto() {
        let mut contrato = NcdContract::default();

        contrato.set_participante(String::from(NOMBRE), EDAD);
    }
}
