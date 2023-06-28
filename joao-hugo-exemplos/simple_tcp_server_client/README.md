Para a simulação funcionar é necessário que a pasta project-tonic esteja no PATH


Adicionar esta linha ao ficheiro .bashrc (com o path correto):

export PATH="${PATH}:/home/shadow-starter/simple_tcp_server_client/project-tonic/target/debug"

E fazer source .bashrc



Temos também de dar update ao path para a zermia_lib no ficheiro /simple_tcp_server_client/project_tonic/Cargo.toml

Ainda temos de instalar o protobuf compiler
Ubuntu:
```
sudo apt-get install protobuf-compiler
```

Fedora:
```
sudo dnf install protobuf-compiler
```

Depois fazemos cargo build na pasta simple_tcp_server_client/project-tonic