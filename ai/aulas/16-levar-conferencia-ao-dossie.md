# Aula 16 — Levar a conferência ao dossiê

## Conceito

Uma ficha aponta para uma fonte; uma fonte de resultado pode apontar para uma
captura. São identidades distintas: TEST_BOUNDARY_2 é a fonte documental e
RUN_EQUIVALENCE_BOUNDARY_2 é a execução registrada. O link não corrige hashes
divergentes: ele fornece uma associação adicional a conferir.

Antes de implementar, definimos que a exportação levaria o resultado da
conferência e seus limites, sem transformar uma declaração em autenticação.

## Implementação

A fonte `test_run` pode conter `capture`, com três campos obrigatórios:

```json
{
  "path": "ai/experimentos/05-associacao/fronteira/execucao.json",
  "sha256": "<SHA-256 dos bytes do registro>",
  "run_id": "RUN_EQUIVALENCE_BOUNDARY_2"
}
```

Esse é um exemplo de formato; o hash real está no
[dossiê](../experimentos/07-dossie-captura/evidencias.json).
Os caminhos do dossiê continuam relativos ao diretório em que o Bibliotecário
é iniciado (nos comandos da aula, a raiz do repositório). O caminho `saida.bin`
da captura é relativo ao diretório do registro.

O Bibliotecário verifica, antes de aceitar a ligação:

1. `kind` é `test_run`, sem o campo `execution` legado preenchido.
2. O hash e as linhas da fonte correspondem ao arquivo atual.
3. O SHA-256 do registro corresponde a `capture.sha256` e seu `run_id`
   corresponde ao declarado na ligação.
4. O caminho da fonte, resolvido e canonicalizado, corresponde a `saida.bin`
   da captura; os hashes declarados para essa saída também coincidem.
5. A conferência da Aula 15 passa, incluindo saída, associações e arquivos
   atuais quando há hash posterior disponível.

O campo é opcional: os dossiês antigos mantêm seu contrato. Fontes de código
não recebem esse link de saída; não inferimos que foram executadas.
Erros impedem a validação ou a exportação, com código 1.

Na exportação, `source.capture_validation` contém status, identidade e hash do
registro, resultado do comando, itens conferidos, relatório e limites. O campo
legado `executed` e os campos de conferência do cabeçalho antigo não são
exportados nessa fonte. Não há consulta ao modelo nesta etapa.

## Testes

```bash
cargo test --offline --bin validate_evidence
```

**72 testes passaram**, incluindo os 68 anteriores e quatro novos que cobrem:
exportação individual e múltipla com IDs e limites; ID/hash do registro incorretos;
fonte errada ou linhas adulteradas; tipo de fonte incorreto ou combinação com
execução legada. Os testes novos usam o dossiê da aula e as fontes da captura
no workspace, sem alterar os artefatos históricos.

## Medição e exportação

Criamos **1 fonte e 2 fichas manuais**, sobre o total de testes aprovados e o
pânico esperado no limite. A conferência terminou com **0 referências inválidas**
e código **0**. Exportamos ambas:

```bash
cargo run --offline --bin validate_evidence -- ai/experimentos/07-dossie-captura/evidencias.json --fact F_BOUNDARY_RUN_RESULT --fact F_BOUNDARY_LIMIT_PANIC --context 1 --output ai/experimentos/07-dossie-captura/selecao.json
```

A [seleção](../experimentos/07-dossie-captura/selecao.json) preserva as duas
fichas e a conferência vinculada a cada uma. O destino já existe; use outro
para repetir. Para validar sem exportar, use apenas o caminho do dossiê.

Não houve nova execução de testes de fronteira, benchmark ou envio ao Qwen.
A contagem de testes/fichas é a medição desta etapa. A conferência pode repetir
leituras por ficha; não otimizamos nem medimos esse custo.

## Explicação e limites

A ligação foi conferida na leitura atual, sem snapshot atômico. O hash do JSON
identifica o registro referenciado, mas alguém pode alterar tanto o registro
quanto o hash no dossiê. Isso não autentica a execução nem comprova compilação.
A verdade semântica das duas fichas continua sendo responsabilidade da análise;
o programa não deduz quatro testes aprovados ao interpretar a saída.

Alterar o validador nesta aula pode fazer a captura RUN_VALIDATE_CAPTURE_1,
que observou suas fontes na Aula 15, divergir dos arquivos atuais. Isso é
esperado e não foi “corrigido” reescrevendo o registro histórico. O novo dossiê
aponta para RUN_EQUIVALENCE_BOUNDARY_2, cujas cinco fontes conferidas permanecem
correspondentes. Nenhuma comprovação foi atribuída a RUN_VECTOR_1.

## Fechamento e próxima aula

A **Aula 16 está concluída**: ligação explícita, conferência, testes e exportação
com limites. A avaliação da Aula 10 permanece **3/6** e o envio ao Ollama continua
manual. A revisão local das aulas 6–12 foi preservada.

Na **Aula 17 — Avaliar a explicação da captura**, a proposta é preparar uma
pergunta e critérios prévios para avaliar se o Qwen distingue fonte, execução,
conferência e autenticação. O envio e a obtenção da resposta continuarão manuais.
Depois seguimos integração e avaliação, performance e cenas por linguagem natural.
