# Testar renderer + Librarian

Requisitos: Git, Rust/Cargo com linker e Python 3. Execute na raiz do renderer.
O primeiro uso pode baixar dependências. Não precisa de banco nem Ollama.

## 1. Consultar o acervo com a dependência publicada

```sh
python3 ai/acervo/renderer-v1/validate.py ./renderer-demo-bundle
```

Esperado: `status: passed`, sete fontes, oito fatos selecionados e recálculo de
35 amostras com mediana **13.232.986 ns**. Abra `renderer-demo-bundle/query.json`
para ver as evidências da pergunta sobre a cena externa e seus limites.

O script confere arquivos, edição Git, linhas, estatísticas, seleção e hashes.
É recálculo de registros preservados, não um novo benchmark. Exige os commits do
acervo disponíveis no Git; um clone raso pode não conter essa história.

## 2. Usar os dois projetos locais

Com o checkout do Librarian que contém o exemplo `export` em `Librarian/`:

```sh
python3 ai/acervo/renderer-v1/validate.py ./librarian-demo-bundle --librarian Librarian
```

Esperado: os mesmos valores. Compare `query.json` dos dois bundles no editor:
devem ser idênticos em bytes. `origin.json` pode diferir em timestamp e caminhos.
Este comando usa explicitamente a biblioteca local; não modifica Cargo.toml/lock
nem a revisão fixa consumida pelo renderer.

Os diretórios de destino devem ser novos. Se já existirem, use outros nomes.
As fontes históricas não devem ser editadas para fazer a conferência passar.

## 3. Experimentar outro domínio

No checkout independente, siga [Librarian/TESTME.md](Librarian/TESTME.md): uma fonte
de manutenção fictícia mostra cadastro e consulta de uma única afirmação.
O [fluxograma](Librarian/WORKFLOW.md) explica validações, rejeições e limites.
Esses links exigem o checkout local; ele é opcional para o passo 1.

## Verificação final opcional

Depois de alterar a integração:

```sh
cargo test --locked --test librarian_integration
```

Consulte a [validação registrada](ai/experimentos/23-librarian-01/README.md) antes
de repetir suítes. A CI fica para a etapa final de publicação.
