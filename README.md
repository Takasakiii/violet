# Violet

Violet é um pequeno e simples monitorador de aplicação, voltado para receber eventos de erro e estado.

## Instalação simples:

### Dependencias:
- [Docker](https://www.docker.com/)
- [Docker Compose](https://github.com/docker/compose) (Para linux apenas)

### Processo:
 - Baixe os arquivos do projeto pelo git ou pelo botão `Download Zip` do Github
 - Renomeie o arquivo `example.env` para `main.env` e preencha as variaveis de ambiente
 - Crie um arquivo `database.env` para inserir a variavel de ambiente `MYSQL_ROOT_PASSWORD` que configura a senha de root do mysql.
 - Execulte o seguinte comando:
 ```sh
 docker-compose up -d
 ```

## Instalação sem Docker:

### Dependencias:
- [Rust Compiler e Cargo](http://rustlang.org/)
- openssl e libssl-dev (Para linux apenas)
- [MariaDB 10.3](https://mariadb.org/)

### Processo:
 - Baixe os arquivos do projeto pelo git ou pelo botão `Download Zip` do Github
 - Build o projeto usando:
 ```sh
 cargo build --release
 ```
 - Renomeie o arquivo `example.env` para `.env` e preencha as variaveis de ambiente
 - Execulte o projeto (Arquivo binario está localizado em `target/release/violet`)
