# CIFRARER
Cifrarer es una herramienta de automatización para crear contenedores cifrados usando el formato de cifrado luks(Solo sistemas operativos Linux).
<p align="center">
  <img width="50%" height="145%" src="https://github.com/Eduardx-2/Cifrarer/blob/main/Ij0DO.jpg">
</p>

# FUNCIÓN
Cifrarer se plantea como una herramienta de automatización a nivel de comandos. ¿Por qué? En muchas ocasiones surge la necesidad de cifrar un disco ya montado, un proceso que puede resultar tedioso y complicado. Por ello, una buena alternativa es crear un contenedor cifrado.

Cifrarer busca simplificar este procedimiento, ofreciendo una solución más cómoda y accesible para una tarea que suele ser aburrida y repetitiva. En lugar de realizar todo mediante comandos, la aplicación permite hacerlo de forma más sencilla a través de una interfaz gráfica.

<p align="center">
  <img width="80%" height="145%" src="https://github.com/Eduardx-2/Cifrarer/blob/main/cifrarer.png">
</p>
¿Por qué alguíen necesitaria un contenedor cifrado? al crear un .img cifrado puede dentro de el colocar una ruta de su sistema, imagine el caso que usted quiera cifrar la ruta de sus imagenes personales, con esto puede facilitar su creación. 


# FUNCIONALIDADES

 - Uso de **Zeroize** para borrar datos de la memoria, en este caso al crear **struct DataInfo<'a>** contiene el campo de la contraseña que se usara para cifrar el contenedor, usando **Zeroize** se encarga de limpiar ese espacio de memoria          especifico, sobreescribe los datos en memoria usando [0]. Imagine el siguente caso -> password:holamundo123 al usar Zeroize toma esa password y cuando password sale de su scope **Zeroize** reescribe ese espacio usando quedando -> [0][0][0][0][0] ⛓️

- Se usa un IPC SOCKET (Inter Process Comunication): comunicación entre procesos via socket usando el puerto 8080.

- Cifrarer prioriza Velocidad y Seguridad, pero también versatilidad por eso se usa una api rest desarrollada en python la cuál se encarga de obtener el espacio libre en el disco del sistema operativo, así evitamos que java en un entorno de flatpak no se vuelva inutil(Sandbox).

- **Tipo de designador**: Fallocate y dd, ambos sirven para crear una imagen de disco (.img), tome en cuenta que la elección depende de cada persona.
- **Tipo de filesystem**: Btrfs y ext4.

