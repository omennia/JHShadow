Para poder usar este programa na simulação, temos de adicionar a pasta
ao PATH.

Podemos fazê-lo correndo os seguintes comandos neste diretório:

-> Fazemos "cargo build" para compilar o projeto
-> Corremos "pwd" para sabermos o diretório
-> Copiamos os conteúdos do diretório e substituímos no próximo comando

-> Corremos este comando, com o SUBSTITUIR_AQUI substituido para o full path para este diretorio
echo 'export PATH="${PATH}:SUBSTITUIR_AQUI/target/debug"' >> ~/.bashrc && source ~/.bashrc


Por exemplo: 
echo 'export PATH="${PATH}:/home/hugocardante/JHShadow/joao-hugo-exemplos/client_server_do_relatorio/target/debug"' >> ~/.bashrc && source ~/.bashrc



De seguida corrigimos o PATH no runner (run), para que possamos correr a simulação apenas com ./run