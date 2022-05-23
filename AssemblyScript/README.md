# AssemblyScript Starter

Esta es una plantilla que puedes usar como punto de inicio para tu proyecto.

## Contenido

Este repositorio contiene un contrato inteligente con los siguientes métodos:
### Escritura:
* `setParticipante`
* `setCertificado`
### Lectura:
* `getParticipante`
* `getParticipantes`

El contrato se encuentra previamente desplegado en la cuenta `as.ncdsamples.testnet`. Puedes hacer llamadas al mismo de la siguiente manera:

```sh
near view as.ncdsamples.testnet getParticipantes
```

## Uso

### Compilando y desplegando

Lo primero que debemos hacer es instalar las dependencias necesarias para que el proyecto funcione.

```sh
npm install
```

ó

```sh
yarn install
```

Una vez hecho esto, podemos compilar el código.

```sh
npm run build
```

ó

```sh
yarn build
```

El contrato compilado en WebAssembly se guarda en la carpeta `AssemblyScript/build/release/`. Ahora solo es necesario desplegarlo en una cuenta de desarrollo.

```sh
near dev-deploy build/release/contrato.wasm
```

### Usando variables de entorno

Una vez compilado y desplegado tu proyecto, vamos a requerir identificar la cuenta neardev. Esta la puedes encontrar en el archivo `AssemblyScript/neardev/neardev`. Podemos almacenar este contrato en una variable de entorno ejecutando lo siguiente en la consola, y sustituyendo por tu cuenta de desarrollo:

```sh
export CONTRATO=dev-0000000000000-000000000
```

Haciendo esto, podemos comprobar que la variable `CONTRATO` tiene almacenada nuestra cuenta dev.

```sh
echo $CONTRATO
```

### Métodos

Lo primero que debemos hacer es registrar al menos un usuario en el contrato. Para esto utilizamos el método `setParticipante`. Este método requiere que se pague 1 NEAR para poder ser ejecutado. El método registra a la persona que lo está ejecutando como participante.

```sh
near call $CONTRATO setParticipante '{"nombre":"Nombre Participante","edad":18}' --accountId tucuenta.testnet --amount 1
```

Ahora que tenemos al menos 1 participante, podemos utilizar los métodos de lectura. `getParticipante` nos traerá la información específica de un participante dependiendo la cuenta que le enviemos como parámetro. Por otro lado, `getParticipantes` nos trae la lista de todos los participantes registrados.

```sh
near view $CONTRATO getParticipante '{"cuenta":"cuenta.testnet"}'
```

```sh
near view $CONTRATO getParticipantes
```

Por último, si queremos marcar como certificado a uno de los participantes registrados, podemos hacer uso del método `setCertificado`. Este método tiene una restricción en la que, si tu cuenta no es `aklassen.testnet` especificamente no te permitirá ejecutarlo. Esta es una forma de agregar una restricción a cuentas específicas. Puedes modificar esta cuenta en el código del contrato. Además, el método transfiere una compensación de 5 NEAR al participante por haber logrado su certificación.

```sh
near call $CONTRATO setCertificado '{"cuenta":"cuenta.testnet"}' --accountId cuenta.testnet
```

