# Arvore van Emde Boas com espaco linear

## Aluno
```shell
José Renan Ferreira Damasceno - 603232
joserenandamasceno@alu.ufc.br
```


## Linguagem

O trabalho foi implementado em Rust, usando `rustc 1.94.0`.

## Como compilar

```sh
make build
```

Esse comando compila o arquivo `src/main.rs` e gera o executavel `programa.exe`.

## Como executar

```sh
make run INPUT=entrada.txt
```

Tambem e possivel executar diretamente:

```sh
./programa.exe entrada.txt
```

O programa recebe um arquivo de texto como argumento. Cada linha deve conter uma das operacoes:

```txt
INC x
REM x
SUC x
PRE x
IMP
```

Os valores aceitos sao inteiros de 32 bits sem sinal, no intervalo de `0` ate `4294967295`.

## Saida

As operacoes `INC` e `REM` nao imprimem nada.

As operacoes `SUC`, `PRE` e `IMP` imprimem primeiro a propria operacao e depois o resultado.

Quando nao existe sucessor, o programa imprime `+INF`. Quando nao existe predecessor, imprime `-INF`.

Na operacao `IMP`, o programa imprime o menor elemento da estrutura e, em seguida, os clusters nao vazios do primeiro nivel. Os clusters e os valores internos sao impressos em ordem crescente.

Se a estrutura estiver vazia durante uma operacao `IMP`, o programa imprime `Min: +INF`.

## Testes

O arquivo `entrada.txt` contem o exemplo do enunciado:

```sh
make run INPUT=entrada.txt
```

O arquivo `teste_completo.txt` cobre estrutura vazia, duplicatas, remocoes, predecessor, sucessor e limites de 32 bits. Para comparar a saida:

```sh
make build
./programa.exe teste_completo.txt > saida.txt
diff saida.txt saida_esperada_teste_completo.txt
```

## Estruturas usadas

Todas as estruturas estao no arquivo `src/main.rs`.

### `Entry<V>`

Representa uma entrada da tabela de dispersao, contendo uma chave `u32` e um valor generico `V`.

### `DynamicHashTable<V>`

Tabela de dispersao propria, implementada com encadeamento separado.

Ela armazena os clusters da arvore van Emde Boas e usa redimensionamento dinamico:

- table doubling: dobra a capacidade quando a taxa de ocupacao chega a aproximadamente 75%;
- table halving: reduz a capacidade pela metade quando a taxa de ocupacao cai para aproximadamente 25%;
- a capacidade minima e 4.

Funcoes principais:

- `new`: cria uma tabela vazia;
- `get`: busca uma chave;
- `get_mut`: busca uma chave para alteracao;
- `contains_key`: verifica se uma chave existe;
- `insert`: insere ou substitui uma chave;
- `remove`: remove uma chave;
- `keys_sorted`: retorna as chaves em ordem crescente;
- `resize`: redimensiona e redistribui as entradas.

### `VanEmdeBoas`

Representa uma arvore van Emde Boas recursiva.

Campos principais:

- `bits`: tamanho do universo da arvore em bits;
- `min`: menor valor armazenado;
- `max`: maior valor armazenado;
- `summary`: arvore auxiliar com os indices dos clusters nao vazios;
- `clusters`: tabela de dispersao com os clusters criados sob demanda.

A instancia principal usa `bits = 32`. Em cada nivel, o valor e dividido em parte alta e parte baixa:

```txt
high(x) = x >> lower_bits
low(x) = x & ((1 << lower_bits) - 1)
index(high, low) = (high << lower_bits) | low
```

No primeiro nivel, isso equivale a dividir o inteiro de 32 bits em duas partes de 16 bits.

Funcoes principais:

- `insert`: insere um valor, ignorando duplicatas;
- `delete`: remove um valor, se ele existir;
- `successor`: retorna o menor valor estritamente maior que o valor consultado;
- `predecessor`: retorna o maior valor estritamente menor que o valor consultado;
- `contains`: verifica se um valor esta presente;
- `elements_sorted`: lista os valores armazenados em ordem crescente;
- `first_level_string`: monta a linha da operacao `IMP`.

## Arquivos

- `src/main.rs`: codigo-fonte completo do programa;
- `Makefile`: comandos de compilacao, execucao e limpeza;
- `README.md`: descricao do trabalho, das estruturas e das funcoes.
- `entrada.txt`: entrada pequena baseada no exemplo do enunciado e usada como valor padrao do `Makefile`;
- `teste_completo.txt`: entrada adicional para testes;
- `saida_esperada_teste_completo.txt`: saida esperada para `teste_completo.txt`.
