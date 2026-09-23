# Testar renderer + Librarian

Requisitos: Git, Rust/Cargo com linker e Python 3. Execute na raiz do renderer.
O primeiro uso pode baixar dependências. Não precisa de banco nem Ollama.

Para começar em outra máquina, clone sem `--depth` e entre na pasta:

```sh
git clone https://github.com/ViniciusSJV/renderer.git renderer-local
cd renderer-local
```

Se você já tem o checkout, use sua pasta atual. Os exemplos usam `python3`;
no Windows, use `py -3` caso esse seja o comando instalado.

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

Se essa pasta ainda não existir, obtenha a versão publicada:

```sh
git clone --branch v0.1.0 https://github.com/ViniciusSJV/librarian.git Librarian
```

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

Este passo funciona sem o checkout local do Librarian. A fonte real do
xadrez-angular já está preservada no acervo: não precisa clonar o projeto,
instalar Angular ou iniciar servidor.

Execute na raiz do renderer, com `xadrez-demo-bundle` ainda inexistente:

```sh
cargo run --locked --bin validate_evidence -- ai/acervo/xadrez-v1/dossier.json --fact F_INIT --fact F_POSITION --fact F_PLAY --fact F_MOVE --context 1 --question ai/acervo/xadrez-v1/question.txt --bundle ./xadrez-demo-bundle
```

Esperado: término sem erro, quatro fichas verificadas e zero referências
inválidas. Abra `xadrez-demo-bundle/query.json` e confira a ordem:

| ID | Chamada declarada |
| --- | --- |
| `F_INIT` | GET `/api/init` |
| `F_POSITION` | GET `/api/row/` + posição |
| `F_PLAY` | PUT `/api/rows/jogada` |
| `F_MOVE` | PUT `/api/rows/mover/` + posição |

Cada item em `evidence.selections` contém a referência e as lacunas;
`evidence.contexts` contém os trechos. O bundle também deve ter `dossier.json`,
`question.txt` e `origin.json`. A pergunta está acompanhada de evidências:
não há resposta automática de LLM nem comprovação de chamadas HTTP executadas.

Para conferir uma rejeição segura, repita exatamente o comando: deve falhar
porque o destino existe, mantendo o bundle anterior. Para repetir com sucesso,
troque o destino por `./xadrez-demo-bundle-2`.

Mais detalhes: [origem e limites da fonte](ai/acervo/xadrez-v1/README.md).

No checkout independente, siga [Librarian/TESTME.md](Librarian/TESTME.md): uma fonte
de manutenção fictícia mostra cadastro e consulta de uma única afirmação.
O [fluxograma](Librarian/WORKFLOW.md) explica validações, rejeições e limites.
Esses links exigem o checkout local; ele é opcional para o passo 1.

## Verificação final opcional

Depois de alterar a integração:

```sh
cargo test --locked --test librarian_integration
```

Consulte a [validação da revisão atual](ai/experimentos/24-librarian-01-consumidor/README.md)
antes de repetir suítes. A CI da biblioteca v0.1.0 já passou em Linux e Windows.
