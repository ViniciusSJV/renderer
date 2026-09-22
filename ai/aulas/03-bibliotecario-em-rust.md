# Aula 3 — Construindo o Bibliotecário em Rust

## Objetivo

Compilar e executar o validador de referências e demonstrar, com três dossiês,
por que um endereço válido não comprova a afirmação que o acompanha.

## Pré-requisitos e preparação do ambiente

Conclua as aulas 1–2 e mantenha o terminal na raiz do clone. Rust compila o
programa; Cargo obtém dependências, compila e executa testes. Verifique ambos,
sem modificar arquivos, em PowerShell ou terminal Linux:

```bash
rustc --version
```

A saída deve identificar o compilador. Em seguida, confira Cargo:

```bash
cargo --version
```

Se faltarem, instale a distribuição Rust com rustup para o sistema utilizado e
reabra o terminal. No Windows, a ferramenta de ligação MSVC também é necessária
para o alvo MSVC: a ausência de `link.exe` foi um problema registrado no projeto.
Instale as ferramentas de compilação C++/MSVC e SDK do Windows quando o diagnóstico
do compilador indicar essa falta. No Linux, é necessário um linker de sistema.

**Limitação:** não há instalador, script de bootstrap nem versão mínima de Rust
validada no repositório. Os registros da Aula 25 citam Rust/Cargo 1.98.1; edição
Rust 2021 no manifesto não significa versão mínima 1.21. A instalação inicial
do sistema não foi reproduzida nesta revisão. Valide-a com as versões acima e
a compilação abaixo; não há ambiente pronto garantido por um Dev Container.

Na raiz, obtenha as dependências exatas do lockfile. Isso usa rede e grava o
cache do Cargo, sem editar o código nem atualizar o lockfile:

```bash
cargo fetch --locked
```

O comando deve terminar sem erro. `--locked` impede resolver outra versão quando
o lockfile precisar mudar. Não use `--offline` na primeira obtenção: essa opção
só permite dependências já presentes no cache.

Compile apenas o Bibliotecário. Isso grava artefatos em `target`, sem executar
uma consulta nem exigir Ollama:

```bash
cargo build --locked --bin validate_evidence
```

Sucesso na compilação confirma ferramentas e dependências suficientes para esse
binário. A mensagem de término e o tempo variam. `src/bin` contém vários
executáveis; selecionar `--bin` evita compilar alvos Unix no Windows.

## Conceitos: a biblioteca de evidências

| Biblioteca | Estrutura do programa |
| --- | --- |
| Dossiê | `Evidence`, com fontes e fichas. |
| Obra | `Source`, identificada por `id`. |
| Trechos numerados (`Lines`) | Campo `lines: Vec<String>`; não há um tipo Rust chamado `Lines`. |
| Ficha | `Fact`, com `id`, `statement`, `source_id` e `line`. |
| Nota de revisão | `Review`, com texto avaliado, referência e parecer. |

Uma ficha é uma afirmação com endereço. F4 aponta para S1:5. O endereço usa
numeração a partir de 1; o vetor Rust começa em zero. O tipo `usize` aceita zero,
mas a regra da aplicação o rejeita. Ler JSON, desserializar e validar são etapas
diferentes.

## Implementação

Leia [src/bin/validate_evidence.rs](../../src/bin/validate_evidence.rs), começando
por `Fact`, `Evidence`, `Source`, `validate_reference` e `find_source`.
O código já contém a evolução até a Aula 25; nesta aula, concentre-se nas
referências. As estruturas mínimas abaixo são recortes explicativos, não arquivos
a substituir no clone:

```rust
struct Fact {
    id: String,
    statement: String,
    source_id: String,
    line: usize,
}
struct Source {
    id: String,
    lines: Vec<String>,
}
```

`String` possui seu texto e `Vec` guarda uma sequência. Referências `&Fact` e
`&Source` emprestam valores sem copiá-los. `validate_reference` confere fonte e
intervalo antes de acessar `lines[line - 1]`. `Result<(), String>` distingue
sucesso sem valor adicional de erro com mensagem; `?` propaga erros.

`find_source` faz busca linear e devolve `Some` ou `None`. Seu lifetime liga o
empréstimo devolvido ao acervo recebido. Para N fontes e F fichas, repetir essa
busca pode exigir até F × N comparações; isso não mede tempo real.
`HashSet` rejeita IDs repetidos: `insert` retorna falso quando o ID já existe.
Textos iguais com IDs diferentes continuam permitidos.

`serde` com `derive` e `serde_json`, declarados em `Cargo.toml`, fazem a conversão
entre JSON e estruturas. Cargo instala essas bibliotecas; não há instalação
manual de cada uma. Campos opcionais presentes no código atual serão estudados
nas próximas aulas; a versão inicial não os preservava todos.

## Passo a passo

Todos os comandos abaixo partem da raiz. Cargo pode atualizar `target`; os
comandos de consulta somente leem os dossiês e imprimem diagnósticos.
Primeiro valide o exemplo original:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/02-mutex/evidencias.json
```

Observe cinco referências válidas e término com código 0. O `--` separa opções
do Cargo dos argumentos do programa. Sem caminho, esse é o dossiê padrão.

Agora execute o caso cuja ficha F4 aponta para S9, inexistente:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/02-mutex/evidencias-fonte-ausente.json
```

O resultado deve informar a referência inválida e terminar com código 1.
Essa falha é o resultado correto deste teste negativo.

Execute o caso cujo texto diz pixel 999, embora o trecho mostre pixel 20:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/02-mutex/evidencias-afirmacao-incorreta.json
```

As referências passam. Esse sucesso estrutural demonstra o limite do validador,
sem aprovar semanticamente a afirmação.

## Conferir um parecer

Abra [parecer-f4.json](../experimentos/02-mutex/parecer-f4.json). Ele preserva a
frase original, a reformulação restrita à operação mostrada, prompt, trecho e
parecer manual do Qwen. A resposta `CONTRADIZ` aponta a diferença entre 20 e 999.
A reformulação é preservada porque o julgamento não vale automaticamente para
qualquer frase semelhante. A coleta não capturou todos os parâmetros do modelo.

Na raiz, confira a correspondência entre parecer e dossiê, sem consultar Ollama:

```bash
cargo run --locked --bin validate_evidence -- ai/experimentos/02-mutex/evidencias-afirmacao-incorreta.json ai/experimentos/02-mutex/parecer-f4.json
```

O parecer P1 deve passar: ID, afirmação original, fonte, linha e cópia do trecho
correspondem. Com o dossiê original, a afirmação difere e o parecer seria recusado.
`evidence_file` é descritivo; o programa compara o dossiê recebido, não autentica
seu caminho nem julga a equivalência semântica da reformulação.

## Validação e problemas comuns

Execute os testes do binário na raiz. Eles compilam testes e usam arquivos
temporários quando necessário; não precisam de Ollama:

```bash
cargo test --locked --bin validate_evidence
```

O registro original desta etapa contém **21 testes aprovados**. Na árvore atual,
o total é maior: verifique ausência de falhas, sem exigir a contagem histórica.
IDs repetidos, arquivo ilegível e JSON incompatível devem ser diagnosticados
antes de tratar a entrada como válida. Não altere uma ficha só para obter código 0.

## Resultado da aula e próxima aula

O Bibliotecário verifica endereços e correspondências textuais; não interpreta
a verdade dos fatos. Não houve benchmark nem mudança no renderer nessa etapa.
A [Aula 4](04-tuplas-codigo-e-evidencias.md) investiga `src/tuple.rs`, registra
uma versão da fonte e separa código de resultado de teste.
