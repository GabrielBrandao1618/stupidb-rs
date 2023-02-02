# Stupidb-rs

Uma CLI simples para armazenar dados de pessoas

## Como usar

É importante ressaltar que o projeto foi criado para fins didáticos, portanto não se trata de uma ferramenta que deve ser usada para algum propósito real.

Para utilizar um comando, basta chamar o executável passando uma string com algum comando SQL(stupid query language)

```bash
stupidb-rs "select where age > 17"
```

Os comandos principais são os listados à seguir:

* [Select](#select)
* [Insert](#insert)
* [Delete](#delete)

### Select

Lista registros gravados
```bash
stupidb-rs "select"
```
Por padrão lista apenas 50, mas isso pode ser mudado ao passar o parâmetro `limit`:
```bash
stupidb-rs "select limit 200"
```

Também é possível utilizar filtros com o parâmetro `where`:
```bash
stupidb-rs "select where age > 18"
```

### Insert

Cria um novo registro recebendo nome e idade:

```bash
stupidb-rs "insert name=Gabriel, age=18"
```

### Delete

Deleta registros
```bash
stupidb-rs "delete where age = 14"
```
Sim, caso seja executado sem o parâmetro where, todos os registros são apagados
