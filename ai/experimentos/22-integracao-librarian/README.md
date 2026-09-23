# Renderer consumindo o Librarian publicado

`Cargo.toml` e `Cargo.lock` fixam `librarian-core` e `librarian-graph-engine` na
revisão Git `4e84f004061b1df84ae5d8f22af9007aa2d99962` de
https://github.com/ViniciusSJV/librarian. A pasta local `Librarian/` é opcional.
Nenhuma outra versão de dependência foi atualizada.

## Experimento

O teste público `tests/librarian_integration.rs` executa a CLI do renderer com o
dossiê externo e verifica duas situações:

1. Fontes válidas geram exatamente os bytes da consulta de referência anterior
   à extração, preservando também o dossiê, a pergunta, a seleção e o hash da query.
2. A alteração de uma fonte fora da seleção impede a exportação de um novo
   bundle e mantém intacta a consulta exportada anteriormente.

Os arquivos alterados pelo segundo teste são cópias temporárias; o dossiê e as
fontes históricas do renderer não são modificados.

## Resultados

- `cargo test --locked`: **317 aprovados**, zero falhas, dois ignorados.
- `cargo test --locked -p librarian-core -p librarian-graph-engine`:
  **21 aprovados**, zero falhas.
- O alvo `librarian_integration` tem dois testes, já incluídos nos 317 do renderer.
- Total: **338 aprovados**, zero falhas, dois ignorados.
- `cargo metadata --locked` confirmou os dois crates como dependências Git da
  revisão fixada, sem referência ao checkout local `Librarian/`.
- As quatro fontes do dossiê externo continuam correspondendo aos hashes e linhas.
- Recálculo das 35 amostras preservadas: mediana de **13.232.986 ns**.

Uma cópia limpa dos arquivos preparados para commit, sem a pasta `Librarian/`
e sem `ai/RETOMADA.md`, também passou nos dois testes de integração. O log está em
`clean-checkout-tests.txt`. A execução usou `--offline`, o cache Git do Cargo e
um diretório de compilação compartilhado; prova independência do checkout local,
mas não é um teste de instalação sem cache. Esses dois testes repetidos não são
somados novamente ao total.

`validation.json` e os logs desta pasta registram as verificações. Os testes HTTP
da suíte usam servidores locais simulados; não houve nova consulta ao Ollama.
O recálculo da baseline não é uma nova medição nem evidência de ganho adicional.

## Reproduzir

Na raiz do renderer, com Rust e Git instalados:

```sh
cargo fetch --locked
cargo test --locked --test librarian_integration
cargo test --locked
cargo test --locked -p librarian-core -p librarian-graph-engine
```

O primeiro comando obtém dependências públicas pela rede. A suíte completa requer
Unix para o capturador e permissão para abrir servidores HTTP locais nos testes.
No Windows nativo, selecione os alvos compatíveis explicitamente.

## Preservação dos registros

Os três ZIPs históricos ausentes foram recuperados do commit `68dde8f` para
conservar as avaliações que dependem deles. O ZIP `attempt-02.zip` da cena externa
continua ausente; a avaliação e os hashes registrados anteriormente foram mantidos.
Os snapshots não foram reescritos para fazê-los corresponder às fontes atuais.

`ai/RETOMADA.md` foi excluído por solicitação do usuário. Uma nova retomada será
criada posteriormente. Este relatório registra somente o experimento concluído.
