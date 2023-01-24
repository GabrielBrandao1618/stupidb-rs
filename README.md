# Stupidb-rs

Uma CLI simples para armazenar dados de pessoas

## Como usar

É importante ressaltar que o projeto foi criado para fins didáticos, portanto não se trata de uma ferramenta que deve ser usada para algum propósito real.

Ao executar sem quaisquer parâmetros, o programa irá listar todos os comandos disponívels, sendo eles:

* [Push](#push)
* [List](#list)
* [Get](#get)
* [Remove](#remove)
* [Clear](#clear)

### Push

Armazena um novo registro, para isso, requer os parâmetros nome e idade, que devem ser passados nesta exata ordem. Exemplo:
```bash
./stupidb-rs push Gabriel 18
```
Todos os registros são criados em uma pasta oculta `.data`

### List

Lista todos os registros gravados. Exemplo de comando:

```bash
./stupidb-rs list
```
Por padrão, o comando irá listar apenas 50 registros, é possível mudar isso ao utilizar a flag `--limit`:
```bash
./stupidb-rs list --limit 1
```

A função de listagem também conta com filtros de idade, basta utiliza-los através das flags `--min-age` e `--max-age`:
```bash
./stupidb-rs list --max-age 20 --min-age 10
```

### Get

Tem uma finalidade parecida com o list, mas serve para retornar apenas o registro de determinado id. Exemplo:
```bash
./stupidb-rs get ec078509-4b76-4df7-9b22-dbfb53e61d74
```

### Remove

Remove um registro de acordo com algum parâmetro, até dado momento, apenas o id é aceito. O comando pode ser executado da seguinte forma:
```bash
./stupidb-rs remove id ec078509-4b76-4df7-9b22-dbfb53e61d74
```
Dessa forma, o registro com o id acima será removido

### Clear

Limpa todos os dados e apaga completamente a pasta `.data`. Não requer nenhum paâmetro, então é tão simples como:
```bash
./stupidb-rs clear
```