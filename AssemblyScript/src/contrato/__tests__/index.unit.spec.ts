import { ONE_NEAR } from "../../utils";
import * as contrato from "../assembly";
import { participantes } from "../assembly/index"
import { VMContext } from "near-sdk-as";

const NOMBRE = "Participante";
const EDAD = 18;
const CUENTA = "participante.testnet"

const setContext = (): void => {
  //Variables del contexto
  VMContext.setAttached_deposit(ONE_NEAR);
  VMContext.setSigner_account_id(CUENTA);
};

describe("Pruebas para método setParticipante", () => {
  it("Registra un participante con sus respectivos datos.", () => {

    setContext();

    contrato.setParticipante(NOMBRE, EDAD);

    const p = participantes.get(CUENTA);

    if (p) {
      expect(p.cuenta).toBe(CUENTA)
      expect(p.nombre).toBe(NOMBRE)
      expect(p.edad).toBe(EDAD)
      expect(p.certificado).toBe(false)
    }

  });

  it("Requiere que la edad sea mayor a 0.", () => {
    setContext();
    expect(() => {
      contrato.setParticipante(NOMBRE, 0);
    }).toThrow("Edad inválida.");
  })

  it("Requiere que el nombre tenga 3 o más caractéres.", () => {
    setContext();
    expect(() => {
      contrato.setParticipante("p", EDAD);
    }).toThrow("El nombre debe contener 3 o más caractéres.");
  })

  it("Requiere que se haga un depósito de al menos 1 NEAR.", () => {
    expect(() => {
      contrato.setParticipante(NOMBRE, EDAD);
    }).toThrow("Debes de pagar 1 NEAR para registrarte.");
  })
})


describe("SetCertificado", () => {
  it("Requiere que aklassen.testnet sea quien ejecute el método.", () => {
    expect(() => {
      contrato.setCertificado(CUENTA)
    }).toThrow("No tienes permisos para ejecutar este comando.")
  })
})