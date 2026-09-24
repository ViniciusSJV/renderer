# Primeiro acervo automático do renderer

Esta página registra a entrega inicial de ingestão. A etapa seguinte já está
implementada e documentada em [busca lexical e grafo](../renderer-search-v1/README.md),
que utiliza esta edição preservada sem reescrever seus snapshots.

Esta etapa gera fontes e declarações estruturais automaticamente com
`librarian-ingest`, desenvolvido no Librarian. Não é um novo dossiê de afirmações
nem uma consulta LLM. Os acervos manuais `renderer-v1` e `xadrez-v*` são preservados.

## Preparação e comandos

Os checkouts locais precisam estar lado a lado:

```text
Projetos/
  librarian/crates/librarian-ingest/
  renderer/
```

O renderer consome o crate novo por `path = "../librarian/crates/librarian-ingest"`.
A versão publicada 0.1.0 não contém esse crate. Core e Graph Engine continuam
usando a revisão Git existente. Após publicar a nova capacidade, a dependência
de ingestão poderá ser fixada em uma revisão publicada.

Na raiz do renderer, com destino novo:

```sh
cargo run --locked --bin catalog_sources -- generate ./renderer-sources-demo
cargo run --locked --bin catalog_sources -- verify ./renderer-sources-demo
cargo run --locked --bin catalog_sources -- verify ./renderer-sources-demo .
cargo run --locked --bin catalog_sources -- show ./renderer-sources-demo ray_from_pixel
```

`generate` usa [renderer-sources.json](../renderer-sources.json): todos os `.rs`
em `src/` e `tests/`, mais Cargo.toml/lock como fontes auxiliares. Não consulta LLM,
executa testes ou compila os arquivos para descobrir os símbolos. Diretórios são
enumerados recursivamente, sem exclusões internas implícitas.

`verify` sozinho confere a edição preservada. Com `.` também compara os arquivos
atuais, indicando alterações, indisponibilidades e novos arquivos no escopo.
`show` exibe declarações com nome exato; ainda não interpreta perguntas.

Saída 0 indica sucesso sem diagnósticos, 2 indica diagnósticos de extração ou
divergência atual, e 1 indica erro de entrada/I/O/integridade. Uma edição com
diagnósticos não deve ser apresentada como extração completa.

## Artefatos desta edição

`edition/` preserva `manifest.json`, `sources.jsonl`, `symbols.jsonl`,
`chunks.jsonl`, `diagnostics.json` e `snapshots/<sha256>`. Bytes são preservados sem
normalização de finais de linha. `.gitattributes` protege os artefatos contra
conversão de texto no Git. O manifesto é publicado por último; destino existente
é rejeitado. Verifique a edição preservada com:

```sh
cargo run --locked --bin catalog_sources -- verify ai/acervo/renderer-auto-v1/edition
cargo run --locked --bin catalog_sources -- show ai/acervo/renderer-auto-v1/edition ray_from_pixel
```

Linhas são inclusivas, começando em 1. Bytes começam em 0, com fim exclusivo.
Cada fonte tem identidade vinculada ao caminho e conteúdo; alterações geram novos
IDs. Trechos de módulo, impl e método podem se sobrepor. Snapshots incluem o
arquivo completo, inclusive comentários fora das declarações.

## Limites e próxima etapa

Validação local em 23/09/2026, Windows: **63 fontes, 906 símbolos/trechos e zero
diagnósticos**. A comparação com a árvore atual não encontrou divergências.
Uma segunda geração produziu os mesmos 68 arquivos, conferidos por SHA-256,
incluindo manifesto, registros e snapshots. `Camera::ray_from_pixel` foi localizado
em `src/camera.rs:75–89`, com o corpo completo; `World::color_at`, em `src/world.rs:95–104`.

O workspace do Librarian passou em **30 testes**, incluindo sete do novo crate.
No renderer, os alvos `--lib --bin catalog_sources --bin validate_evidence`
passaram em **278 testes** (209 + 1 + 68). A tentativa inicial de teste de integração
compilou também `capture_execution`, exclusivo de Unix; o consumidor foi testado
como alvo específico no Windows e exercitado pela CLI real. A fixture sintética
do validador foi ajustada para usar o hash dos bytes copiados neste checkout;
os hashes dos registros históricos em disco não foram modificados.

Comandos de teste usados:

```sh
# Na raiz do Librarian
cargo test --workspace --locked --offline
# Na raiz do renderer
cargo test --locked --offline --lib --bin catalog_sources --bin validate_evidence
```

O teste do Librarian usou `--target-dir` dentro do workspace gravável do renderer;
isso muda o destino dos artefatos de compilação, não os testes executados.

O extrator registra sintaxe, incluindo itens sob `cfg`, sem decidir quais foram
compilados. Não expande macros, resolve referências entre arquivos, interpreta
comportamento ou comprova execução de testes. Os índices em memória permitem
consultas por fonte, ID e nome. Hashes conferem consistência, não autenticidade.
Metadados Git são observações opcionais; a edição pode refletir alterações locais.

A próxima etapa é recuperar trechos pela pergunta e montar um dossiê rastreável
para a conferência/exportação existente. A extração de TypeScript/Angular será
outro adaptador, reutilizando o contrato do acervo.
