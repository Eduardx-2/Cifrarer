# CIFRARER
Cifrarer es una herramienta de automatización para crear contenedores cifrados usando el formato de cifrado luks.
<p align="center">
  <img width="80%" height="145%" src="https://github.com/Eduardx-2/Cifrarer/blob/main/cifrarer.png">
</p>

# FUNCIÓN
Cifrarer se plantea como una herramienta de automatización a nivel de comandos. ¿Por qué? En muchas ocasiones surge la necesidad de cifrar un disco ya montado, un proceso que puede resultar tedioso y complicado. Por ello, una buena alternativa es crear un contenedor cifrado.

Cifrarer busca simplificar este procedimiento, ofreciendo una solución más cómoda y accesible para una tarea que suele ser aburrida y repetitiva. En lugar de realizar todo mediante comandos, la aplicación permite hacerlo de forma más sencilla a través de una interfaz gráfica.

# FUNCIONALIDADES

Uso de **Zeroize** para borrar datos de la memoria, en este caso al crear **struct DataInfo<'a>** contiene el campo de la contraseña que se usara para cifrar el contenedor, usando **Zeroize** se encarga de limpiar ese espacio de memoria especifico, sobreescribe los datos en memoria usando [0]. Imagine el siguente caso -> password:holamundo123 al usar Zeroize toma esa password y cuando password sale de su scope **Zeroize** reescribe ese espacio usando quedando -> [0][0][0][0][0] ⛓️
