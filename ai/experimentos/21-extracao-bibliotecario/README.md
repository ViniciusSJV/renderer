# Extração local de Bibliotecário e Graph Engine

## Estado consolidado

Repositório publicado: https://github.com/ViniciusSJV/librarian

A revisão `118f1613be3b0585b894e841dd78e9518d56800d` foi publicada e teve CI
aprovada em Linux e Windows:
https://github.com/ViniciusSJV/librarian/actions/runs/35796391179.
A revisão seguinte, `4e84f004061b1df84ae5d8f22af9007aa2d99962`, atualiza apenas o
README do Librarian e é a dependência Git agora fixada pelo renderer.

Os registros desta pasta descrevem a extração e sua renomeação. Os caminhos
`bibliotecario/` e `Librarian/` nos logs eram checkouts locais durante os ensaios;
não são necessários para compilar o renderer. As dificuldades de autenticação
foram resolvidas antes da publicação, sem gravar credenciais no Git.
A validação do consumidor com a revisão publicada está no
[experimento 22](../22-integracao-librarian/README.md).

## Alterações

- A CLI usa `prepare_query` e `PreparedQuery::write_bundle`.
- Verificação de fontes/capturas e de pareceres foi movida para a biblioteca.
- Fato, fonte parametrizada e dossiê são contratos do core. Capturas e registros
  de execução são especializações do Graph Engine.
- Todas as fontes e referências são verificadas, inclusive as não selecionadas.
- O bundle de compatibilidade rejeita referências fora da fonte, linhas fora dos
  limites, fatos duplicados e seleção desconhecida/duplicada. Uma fixture antiga
  do renderer apontava para linha 3 de uma fonte de duas linhas; foi corrigida.
- O consumidor de manutenção fictícia funciona sem renderer, executável validador,
  banco ou servidor LLM.

## Evidências e resultados

`before/` foi gerado pela CLI antes desta refatoração; `after/`, pela CLI nova.
`validation.json` registra comparação exata de bytes e hashes. A consulta externa
é idêntica, assim como dossiê e pergunta. O registro de origem tem timestamp novo
e atribui a conferência à biblioteca/callback, em vez do nome da CLI.

`renderer-tests.txt`: 315 aprovados, zero falhas, dois ignorados.
`bibliotecario-tests.txt`: 21 aprovados, zero falhas.
Total: **336 aprovados**, zero falhas, dois ignorados.
`isolated-tests.txt`: os mesmos 21 testes passaram em cópia fora da árvore do
renderer, usando apenas os arquivos do novo projeto. O exemplo também executou
nessa cópia. Essa repetição não aumenta a contagem de testes únicos. Um teste saiu da CLI e
passou a pertencer à biblioteca. Os testes HTTP usam servidor local simulado.

`maintenance-bundle/` registra a execução do exemplo em outro domínio. As fontes
são fictícias e as afirmações são fornecidas pelo adapter. Isso prova reutilização
até a exportação, sem alegar uma consulta real ou avaliação semântica por LLM.

Nenhum benchmark novo foi coletado. Nenhuma resposta LLM foi promovida a fato.
Os snapshots preservam caminhos da execução local: isso não autentica a máquina
nem torna os caminhos portáveis. Os ZIPs e avaliações históricos não foram alterados.

## Consumir a versão publicada

Na raiz do renderer:

```sh
cargo fetch --locked
cargo test --locked --test librarian_integration
cargo test --locked
```

Cargo obtém os dois crates do GitHub pela revisão fixada. O checkout independente
`Librarian/` é opcional e fica ignorado pelo Git do renderer. O banco de grafos e
a extração do LLM Engine continuam sendo etapas futuras.

## Estado preservado

Os arquivos `bibliotecario-1654bea.tar.gz` e `Librarian-118f161.tar.gz` conservam
snapshots de código das etapas anteriores; seus hashes constam dos registros.
Os logs e bundles não foram reescritos para representar a integração atual.
Os três ZIPs históricos que estavam ausentes localmente foram recuperados do
commit anterior, preservando os arquivos usados pelas avaliações existentes.
