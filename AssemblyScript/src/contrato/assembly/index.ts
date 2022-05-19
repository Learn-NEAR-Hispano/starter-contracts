import { PersistentUnorderedMap, logging, context, u128, ContractPromiseBatch } from 'near-sdk-as'

const ONE_NEAR = u128.from('1000000000000000000000000');

//Creamos una clase llamada participante
@nearBindgen
class Participante {
  cuenta: string;
  nombre: string;
  edad: u32;
  certificado: bool;

  //Inicializamos el objeto
  constructor(cuenta: string, nombre: string, edad: u32) {
    this.cuenta = cuenta;
    this.nombre = nombre;
    this.edad = edad;
    this.certificado = false;
  }
}

//Creamos una colección para almacenar información en nuestro contrato.
const participantes = new PersistentUnorderedMap<string, Participante>("p");

//MÉTODOS DEL CONTRATO:

//Esta función te registra como participante.
export function setParticipante(nombre: string, edad: u32): void {

  //Usamos el context de la transacción para obtener datos de la misma.
  const cuenta = context.sender;
  const deposito = context.attachedDeposit;

  //Hacemos validaciones. Queremos que:
  //* No pongan 0 como edad, osea que la edad sea mayor a 0.
  //* El nombre tenga más de 3 caractéres.
  //* Paguen 1 NEAR cada que se registren
  assert(edad > 0, "Edad inválida.");
  assert(nombre.length >= 3, "El nombre debe contener 3 o más caractéres.");
  assert(deposito > ONE_NEAR, "Debes de pagar 1 NEAR para registrarte.");

  //Instanciamos la clase (creamos un objeto) y le mandamos los datos al constructor.
  let participante = new Participante(cuenta, nombre, edad);

  //Guardamos la información en la blockchain.
  //PersistentUnorderedMap requiere una clave y el dato a guardar.
  //Para más información consulta: https://docs.near.org/docs/concepts/data-storage#persistentunorderedmap
  participantes.set(cuenta, participante);

  //Le enviamos un mensaje de confirmación a la consola.
  logging.log("Registro creado exitosamente.");
}

//Función de LECTURA. Consultamos un integrante en base a su cuenta.
export function getParticipante(cuenta: string): Participante | null {
  return participantes.get(cuenta);
}

//Función de LECTURA. Consultamos toda la lista de participantes registrados.
export function getParticipantes(): Participante[] {
  return participantes.values();
}

//Función de ESCRITURA.
//En esta función modificamos el estado de la certificación.
//Esta función regresa un valor booleano.
export function setCertificado(cuenta: string): bool {

  //Si la cuenta ejecutando el comando no es aklassen.testnet, no podrá hacerse ningún cambio.
  assert(context.sender == "aklassen.testnet", "No tienes permisos para ejecutar este comando.");

  let participante = participantes.get(cuenta);

  //Necesitamos evaluar si la línea de arriba encontró al participante.
  if (participante) {
    participante.certificado = true;

    //Le transferimos al participante 5 NEAR como premio por haber logrado su certificación.
    ContractPromiseBatch.create(cuenta).transfer(u128.from(5));

    participantes.set(cuenta, participante);
    logging.log("Participante certificado. El participante ha recibido su recompensa de 5 NEAR.");

    return true;
  }
  else {
    logging.log("Participante no encontrado.");
    return false;
  }
}

