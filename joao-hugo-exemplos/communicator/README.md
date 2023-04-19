Para correr esta simulação temos de seguir os seguintes passos:

Compilar o projeto correndo ```cargo build``` na pasta communicator.

Adicionar /path/to/shadow_main/joao-hugo-exemplos/communicator/target/debug ao path:

Alterar /path/to/shadow/  pelo path apropriado:

```bash
echo 'export PATH="${PATH}:/home/${USER}/path/to/shadow/joao-hugo-exemplos/communicator/target/debug"' >> ~/.bashrc && source ~/.bashrc
```

E temos também de alterar o path no ficheiro run para o path adequado:
será o mesmo path sem o /target/debug, e aponta para o ficheiro Cargo.toml
exemplo:
```bash
/home/shadow-starter/shadow-main/joao-hugo-exemplos/communicator/Cargo.toml
```


Começar o listener com:
```bash
jh_receiver
```

E correr a simulação executando:
```bash
source run
```
na pasta do exemplo.


