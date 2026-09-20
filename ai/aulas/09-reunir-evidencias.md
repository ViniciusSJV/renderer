# Aula 9 — Reunir evidências sem misturar seus papéis

## Conceito

Uma pergunta pode precisar de mais de uma ficha. O código descreve o
procedimento; o relatório registra um resultado. Reuni-los numa consulta
não comprova automaticamente que aquela versão do código produziu aquele
resultado. Cada ficha precisa conservar sua origem e os limites da evidência.

Também podemos selecionar duas fichas da mesma fonte. Se elas apontam para
linhas próximas, suas janelas de contexto podem repetir o mesmo texto.
Queremos compartilhar esse contexto sem perder o endereço de cada ficha.

## Primeiro passo: selecionar várias fichas

Em [validate_evidence.rs](../../src/bin/validate_evidence.rs), passamos a aceitar
`--fact ID` repetido. `select_facts` resolve os IDs na ordem solicitada e rejeita
IDs inexistentes ou repetidos. A CLI continua validando o dossiê inteiro antes
da seleção. Erros impedem a criação do arquivo de exportação.

A seleção de uma única ficha conserva o formato anterior. Várias fichas
produzem uma lista `selections`; cada item preserva afirmação, referência,
metadados e limites. A pergunta continua separada, usando `--question`.

A [exportação inicial](../experimentos/03-tuplas/consulta-codigo-e-resultado-inicial.json)
reuniu F_VECTOR_TEST_X e F_VECTOR_TEST_PASSED. Nessa etapa, os contextos ainda
ficavam dentro de cada seleção. Os 57 testes passaram; a CLI também rejeitou
IDs repetidos, inexistentes e sem valor, preservou arquivos existentes e
produziu bytes iguais ao artefato da Aula 8 para a seleção individual.

## Segundo passo: compartilhar contexto

Nas novas exportações múltiplas, `contexts` guarda os blocos de texto.
Cada `source.context` na lista `selections` contém:

- `requested_radius`: raio pedido.
- `start_line` e `end_line`: limites da janela individual, inclusivos.
- `context_id`: referência ao bloco compartilhado.

A linha exata e o trecho da ficha continuam em `source.line` e `source.excerpt`.
Esse trecho curto permanece repetido intencionalmente para leitura da ficha;
a deduplicação trata as janelas de contexto, não todo texto do documento.
Metadados e questões abertas também continuam em cada seleção.

Os blocos possuem ID local à exportação, `source_id`, limites e `lines`.
Não unimos textos de fontes diferentes, mesmo que sejam idênticos. As fontes
aparecem na ordem em que foram selecionadas pela primeira vez; seus blocos
ficam em ordem de linha. A ordem das fichas permanece a solicitada.

### Como unir janelas

Internamente, usamos intervalos com início incluso e fim exclusivo, como nas
fatias Rust. Ordenamos os intervalos de cada fonte e percorremos a lista:

1. Se o próximo intervalo sobrepõe ou encosta no anterior, ampliamos o fim.
2. Se existe uma lacuna, começamos outro bloco.

A função `merge_ranges` faz essa união. Para K janelas de uma fonte, a ordenação
custa O(K log K) comparações e a passagem posterior é linear. Isso descreve
apenas a união: a implementação completa ainda usa buscas lineares e percorre
as fichas ao associar blocos. Não fizemos benchmark nem afirmamos ganho de tempo.

Uma janela individual pode ser menor que o bloco compartilhado. Seus limites
originais são preservados para que o leitor consiga recuperá-la. Não incluímos
linhas das lacunas entre janelas distantes. Continuamos trabalhando com texto;
a união não entende funções Rust e não garante um trecho semanticamente completo.

## Experimento reproduzível

Selecionamos as fichas de x, y e do resultado do teste:

```bash
cargo run --offline --bin validate_evidence -- ai/experimentos/03-tuplas/evidencias-execucao-identificada.json --fact F_VECTOR_TEST_X --fact F_VECTOR_TEST_Y --fact F_VECTOR_TEST_PASSED --context 4 --output ai/experimentos/03-tuplas/consulta-contexto-compartilhado.json
```

O [artefato final](../experimentos/03-tuplas/consulta-contexto-compartilhado.json)
já existe. Para repetir o comando, escolha outro destino.

| Ficha | Janela individual | Bloco compartilhado |
| --- | --- | --- |
| F_VECTOR_TEST_X | SRC_TUPLE, 155–163 | CTX_1: SRC_TUPLE, 155–164 |
| F_VECTOR_TEST_Y | SRC_TUPLE, 156–164 | CTX_1: SRC_TUPLE, 155–164 |
| F_VECTOR_TEST_PASSED | TEST_VECTOR_1, 23–29 | CTX_2: TEST_VECTOR_1, 23–29 |

As janelas individuais somavam 25 linhas de contexto; os blocos somam 17.
Todas as linhas solicitadas foram preservadas. Essa contagem não inclui os
excertos individuais nem mede tamanho total do JSON ou quantidade de tokens.

Uma segunda exportação temporária com os mesmos parâmetros produziu bytes
iguais. Também verificamos que `--question` preserva o conjunto de seleções e
blocos em `evidence`. A pergunta temporária testou o transporte; não foi enviada
ao Qwen e não constitui avaliação do modelo.

## Testes e limites

Executamos `cargo test --offline --bin validate_evidence`: **60 testes passaram**.
Os testes cobrem sobreposição, adjacência, intervalos contidos, lacunas,
fontes distintas com texto igual, raio zero, raio máximo, ordem das fichas,
recuperação das janelas individuais e preservação dos papéis de código e execução.
A seleção individual mantém o formato anterior.

Os IDs CTX identificam blocos apenas dentro de uma exportação. Não representam
identidade histórica nem devem ser usados isoladamente para comparar documentos.
Os artefatos das etapas anteriores permanecem preservados com seus formatos.

Não houve nova consulta ao Qwen, benchmark ou execução da suíte completa do
renderer. O resultado desta aula é um pacote de evidências com menos repetição
de contexto, não uma melhora demonstrada na qualidade das respostas.

## Fechamento e Aula 10

A Aula 9 está concluída: selecionamos várias fichas, preservamos suas origens
e compartilhamos contexto sem preencher lacunas ou misturar fontes.

Na **Aula 10 — Avaliar uma consulta com código e resultado**, vamos preparar
uma pergunta para esse conjunto e definir critérios antes de receber a resposta
do Qwen. Avaliaremos referências, distinção entre procedimento e resultado,
limites da macro não fornecida e ausência de comprovação da associação histórica
entre código e execução. O envio continuará manual; a resposta recebida e a
avaliação serão registradas separadamente. Uma resposta não demonstrará
confiabilidade geral nem uma melhora causal em relação às aulas anteriores.
