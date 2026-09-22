# Experimento 18 — origem de uma consulta recém-exportada

Etapa pequena de integração, sem envio ao Ollama. O dossiê manual aponta para
a verificação de modelo vazio no código atual de ollama_common/mod.rs. Não é
registro de execução desse código; a leitura é conferida por hash/linhas.

## Reproduzir no Codespaces

```bash
cargo test --locked --offline --bin validate_evidence
cargo run --locked --offline --bin validate_evidence -- ai/experimentos/18-origem-consulta/evidencias.json --fact F_MODEL_EMPTY_CHECK --context 1 --question ai/experimentos/18-origem-consulta/pergunta.txt --bundle ai/experimentos/18-origem-consulta/pacote-02
```

Usar destino novo. O pacote-01 é o resultado preservado nesta sessão. O modo
offline exige dependências locais. Não transferimos este código ao Windows ainda.

## Resultados

78 testes aprovados. Conferências da CLI em verificacao.json: hashes das cópias,
igualdade byte a byte com --output, destino existente recusado sem sobrescrita,
referência inválida sem pacote e opções conflitantes recusadas. A tentativa de
reexportar o dossiê histórico da Aula 17 foi recusada por divergência do hash
atual de Cargo.toml; seu diagnóstico está preservado. Não corrigimos hashes
históricos para fazer a conferência passar.

Uma amostra debug do processo durou 228,051 ms, incluindo I/O, sem compilação.
Consulta: 2201 bytes; origin.json: 1189 bytes. Não é benchmark nem ganho medido.

## Mudança nos testes antigos

Seis testes de ligação de captura dependiam da captura histórica, cujo hash de
Cargo.toml deixou de corresponder após adicionar reqwest. A fixture agora cria
registro sintético e arquivos temporários, removidos ao final. O relatório
histórico é usado apenas como texto de teste: não é reautenticado nem alterado.
Um teste adicional altera a fonte temporária e exige rejeição. Testes existentes
de hash, ID, ligação e cache continuam exigindo suas respectivas falhas.

## Contrato do pacote

--bundle exige --question e é incompatível com --output. Primeiro o Bibliotecário
valida as fontes/referências e constrói a consulta. Só então reserva um diretório
novo e grava dossier.json, question.txt e query.json, com seus bytes originais.
origin.json registra caminhos de origem, ID do dossiê (ou null), hashes, seleção,
contexto e horário de término da exportação. É gravado por último; sua ausência
indica pacote parcial. Código 0 indica término do comando; erros retornam 1.
Falha de gravação pode deixar diretório parcial; não sobrescrevê-lo ao repetir.

O pacote não é snapshot atômico das fontes, não autentica execução nem valida
semântica. Não contém rubrica ainda. Caminhos de fontes continuam relativos à
raiz do projeto conforme a CLI atual. O cliente send_ollama ainda não consome
origin.json e não deve promover o pacote a conferência no momento de um envio
posterior. Próximo passo: unir conferência/exportação e envio, preservar origem
no envelope da tentativa e vincular uma rubrica antes da resposta.
