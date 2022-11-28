# NCD-samples
Repositorio de ejemplos de apoyo para la certificación de desarrolladores en NEAR.

## Contenido

Este repositorio contiene ejemplos de Smart Contracts desarrollados en AssemblyScript y Rust, así como una implementación para front end utilizando HTML y JavaScript.

Para probar los contratos se requiere [NodeJs](https://nodejs.org/en/download/) y [NPM](https://docs.npmjs.com/cli/v7/configuring-npm/install/).

Además, es necesario instalar la `near-cli`

```sh
npm i -g near-cli
```

Puedes probar si tu instalación fue correcta utilizando el siguiente comando:

```sh
near state ncdsamples.testnet
```

Por último, para poder hacer llamadas a los contratos desde tu terminal es necesario hacer login con el siguiente comando:

```sh
near login
```

Se abrirá una ventana del navegador donde podrás seleccionar tu wallet.

### AssemblyScript

* Código del contrato: `AssemblyScript/src/contrato/assembly/index.ts`
* Cuenta pre-desplegada: `as.ncdsamples.testnet`

### Rust

* Código del contrato: `Rust/src/lib.rs`
* Cuenta pre-desplegada: `rust.ncdsamples.testnet`

### JavaScript

* Código del contrato: `JavaScript/src/contract.ts`
* Cuenta pre-desplegada: `js.ncdsamples.testnet`

Puedes consultar más información sobre el uso de cualquiera de las plantillas en sus respectivas carpetas.