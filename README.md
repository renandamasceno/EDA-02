# Árvore van Emde Boas com espaço linear

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

Esse comando compila o arquivo `src/main.rs` e gera o executável `programa.exe`.

## Como executar

```sh
make run INPUT=entrada.txt
```

Também é possível executar diretamente:

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

Os valores aceitos são inteiros de 32 bits sem sinal, no intervalo de `0` ate `4294967295`.

## Saida

As operações `INC` e `REM` não imprimem nada.

As operações `SUC`, `PRE` e `IMP` imprimem primeiro a propria operação e depois o resultado.

Quando não existe sucessor, o programa imprime `+INF`. Quando não existe predecessor, imprime `-INF`.

Na operação `IMP`, o programa imprime o menor elemento da estrutura e, em seguida, os clusters não vazios do primeiro nível. Os clusters e os valores internos são impressos em ordem crescente.

Se a estrutura estiver vazia durante uma operação `IMP`, o programa imprime apenas uma linha vazia após a linha `IMP`.

## Testes

O arquivo `entrada.txt` contem o exemplo do enunciado:

```sh
make run INPUT=entrada.txt
```

O arquivo `teste_completo.txt` cobre estrutura vazia, duplicatas, remoções, predecessor, sucessor e limites de 32 bits. Para comparar a saída:

```sh
make build
./programa.exe teste_completo.txt > saida.txt
diff saida.txt saida_esperada_teste_completo.txt
```

## Estruturas usadas

Todas as estruturas estão no arquivo `src/main.rs`.

### `Entry<V>`

Representa uma entrada da tabela de dispersão, contendo uma chave `u32` e um valor genérico `V`.

### `DynamicHashTable<V>`

Tabela de dispersão propria, implementada com encadeamento separado.

Ela armazena os clusters da árvore van Emde Boas e usa redimensionamento dinâmico:

- busca, inserção e remoção têm tempo esperado `O(1)`, assumindo distribuição uniforme da função de hash;
- table doubling: dobra a capacidade quando a taxa de ocupação chega a aproximadamente 75%;
- table halving: reduz a capacidade pela metade quando a taxa de ocupação cai para aproximadamente 25%;
- a capacidade minima e 4.

Funções principais:

- `new`: cria uma tabela vazia;
- `get`: busca uma chave;
- `get_mut`: busca uma chave para alteração;
- `contains_key`: verifica se uma chave existe;
- `insert`: insere ou substitui uma chave;
- `remove`: remove uma chave;
- `keys_sorted`: retorna as chaves em ordem crescente;
- `resize`: redimensiona e redistribui as entradas.

### `VanEmdeBoas`

Representa uma árvore van Emde Boas recursiva.

Campos principais:

- `bits`: tamanho do universo da árvore em bits;
- `min`: menor valor armazenado;
- `max`: maior valor armazenado;
- `summary`: árvore auxiliar com os indices dos clusters não vazios;
- `clusters`: tabela de dispersão com os clusters criados sob demanda.

A instância principal usa `bits = 32`. Em cada nível, o valor e dividido em parte alta e parte baixa:

```txt
high(x) = x >> lower_bits
low(x) = x & ((1 << lower_bits) - 1)
index(high, low) = (high << lower_bits) | low
```

No primeiro nível, isso equivale a dividir o inteiro de 32 bits em duas partes de 16 bits.

Funções principais:

- `insert`: insere um valor, ignorando duplicatas;
- `delete`: remove um valor, se ele existir;
- `successor`: retorna o menor valor estritamente maior que o valor consultado;
- `predecessor`: retorna o maior valor estritamente menor que o valor consultado;
- `contains`: verifica se um valor esta presente;
- `elements_sorted`: lista os valores armazenados em ordem crescente;
- `first_level_string`: monta a linha da operação `IMP`.

## Arquivos

- `src/main.rs`: código-fonte completo do programa;
- `Makefile`: comandos de compilação, execução e limpeza;
- `README.md`: descrição do trabalho, das estruturas e das funções.
- `entrada.txt`: entrada pequena baseada no exemplo do enunciado e usada como valor padrão do `Makefile`;
- `teste_completo.txt`: entrada adicional para testes;
- `saida_esperada_teste_completo.txt`: saída esperada para `teste_completo.txt`.
