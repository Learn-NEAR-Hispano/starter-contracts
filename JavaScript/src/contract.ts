import { NearBindgen, NearPromise, UnorderedMap, near, call, assert, view } from 'near-sdk-js';
import { ONE_NEAR } from 'near-sdk-js/lib/types';

//Creamos una clase llamada participante
@NearBindgen({})
class Participante {
  nombre: string;
  cuenta: string;
  edad: number;
  certificado: boolean;

  //Inicializamos el objeto
  constructor(nombre: string, cuenta: string, edad: number) {
    this.nombre = nombre;
    this.cuenta = cuenta;
    this.edad = edad;
    this.certificado = false;
  }
}

//Creamos la clase principal del contrato
@NearBindgen({})
class StarterContract {
  //Creamos una colección para almacenar información en nuestro contrato.
  participantes: UnorderedMap<Participante> = new UnorderedMap<Participante>('p');

  /**
   * Método de ESCRITURA para registrar un nuevo participante
   * El comando para utilizarlo en la terminal es:
   *  >> near call $CONTRATO set_participante '{"nombre":"NOMBRE","edad":18}' --accountId cuenta.near --amount 1
   *    * $CONTRATO es una variable de entorno que contiene el id de la cuenta del contrato
   * 
   * @param nombre string que requiere el nombre del participante a registrar
   * @param edad entero de 32 bits sin signo que requiere la edad del participante
   * 
   * Es necesario enviarle 1 NEAR (o más) como pago a este método.
   * Como vamos a aceptar pagos con este método, lo marcamos como payableFunction.
   * El método registra la cuenta que firma la tx cómo la cuenta del participante registrándose.
   */
  @call({ payableFunction: true })
  set_participante({ nombre, edad }: { nombre: string, edad: number }): void {
    //Usamos el objeto near para obtener datos de la transacción.
    const cuenta = near.signerAccountId();
    const deposito = near.attachedDeposit();

    //Hacemos validaciones. Queremos que:
    //* No pongan 0 como edad, osea que la edad sea mayor a 0.
    //* El nombre tenga más de 3 caractéres.
    //* Paguen 1 NEAR cada que se registren
    assert(edad > 0, "Edad inválida.");
    assert(nombre.length >= 3, "El nombre debe contener 3 o más caractéres.");
    assert(deposito >= ONE_NEAR, "Debes de pagar 1 NEAR para registrarte.");

    //Instanciamos la clase (creamos un objeto) y le mandamos los datos al constructor.
    const participante = new Participante(nombre, cuenta, edad);

    //Guardamos la información en la blockchain.
    //UnorderedMap requiere una clave y el dato a guardar.
    //Dado a que se requiere una clave única, vamos a usar la cuenta como clave.
    //Para más información consulta: https://docs.near.org/develop/contracts/basics#sdk-collections
    this.participantes.set(cuenta, participante);

    //Le enviamos un mensaje de confirmación a la consola.
    near.log("Registro creado exitosamente.");
  }

  /**
   * Método de LECTURA que regresa un participante
   * El comando para utilizarlo en la terminal es:
   *  >> near view $CONTRATO get_participante '{"cuenta":"CUENTA.NEAR"}'
   * @param cuenta string que contiene la cuenta (key) del usuario a consultar
   */
  @view({})
  get_participante({ cuenta }: { cuenta: string }) {
    return this.participantes.get(cuenta);
  }

  /**
   * Método de LECTURA que regresa toda la lista de participantes registrados
   * El comando para utilizarlo en la terminal es:
   *  >> near view $CONTRATO get_participantes '{}'
   */
  @view({})
  get_participantes() {
    return this.participantes.toArray();
  }

  /**
   * Método de ESCRITURA para certificar a un participante
   * Además, transfiere 5 NEAR como compensación al participante que se haya certificado.
   * El comando para utilizarlo en la terminal es:
   *  >> near call $CONTRATO set_certificado '{"cuenta":"cuenta.near"}' --accountId cuenta.near --amount 1
   * 
   * @param cuenta string que contiene la cuenta del participante a certificar
   */
  @call({})
  set_certificado({ cuenta }: { cuenta: string }) {

    //Si la cuenta ejecutando el comando no es aklassen.testnet, no podrá hacerse ningún cambio.
    assert(near.signerAccountId() == "aklassen.testnet", "No tienes permisos para ejecutar este comando.");

    //Buscamos al participante. En este caso se declara como let porque vamos a modificarlo.
    let participante = this.participantes.get(cuenta);

    //Necesitamos evaluar si la línea de arriba encontró al participante.
    if (participante && participante.certificado == false) {
      participante.certificado = true;

      //Le transferimos al participante 5 NEAR como premio por haber logrado su certificación.
      const promise = near.promiseBatchCreate(cuenta);
      near.promiseBatchActionTransfer(promise, ONE_NEAR * BigInt(5));

      //Y guardamos los cambios hechos al participante
      this.participantes.set(cuenta, participante);
      near.log("Participante certificado. El participante ha recibido su recompensa de 5 NEAR.");

      //Por último, regresamos true indicando que la acción se completó exitosamente.
      return true;
    }
    else if (participante && participante.certificado == true) {
      //Si no encuentra al participante, o si este ya está certificado
      //regresamos false.
      near.log("El participante ya se encontraba certificado.");

      return false;
    }
    else {
      near.log("Participante no encontrado.");

      return false;
    }
  }
}