# Renderer consumindo Librarian v0.1.0

Os dois crates passam da revisão `4e84f004061b1df84ae5d8f22af9007aa2d99962`
para `a009af8276c4bb58c67905a10fea32f1cbbf8a38`. Cargo.toml e Cargo.lock
mantêm revisão Git fixa; nenhuma outra dependência mudou.

Validação específica: `cargo test --locked --test librarian_integration` passou
com **2 aprovados, zero falhas**. A CLI preserva a query externa de referência e
rejeita fonte alterada fora da seleção sem modificar o bundle anterior. A suíte
completa não foi repetida.

A [release v0.1.0](https://github.com/ViniciusSJV/librarian/releases/tag/v0.1.0)
foi publicada no commit acima, cuja
[CI Linux/Windows](https://github.com/ViniciusSJV/librarian/actions/runs/35899202017)
já havia passado.

O [acervo xadrez](../../acervo/xadrez-v1/README.md) cadastra um serviço TypeScript
real na revisão documentada. Exportação pela CLI consumindo o novo Librarian
concluiu; foram conferidos ordem dos quatro IDs, referências, trechos, lacunas
e hashes dos arquivos. `xadrez-bundle/`, `xadrez-export.txt` e `validation.json`
preservam a evidência desta etapa.

É uso do contrato sobre fonte de outro projeto, não integração da biblioteca
na aplicação Angular. Nenhuma execução da aplicação, benchmark ou chamada LLM.
Relatórios e snapshots anteriores não foram alterados.

## Roteiro para o usuário

O TESTME da raiz inclui obtenção dos projetos, comandos completos e resultados
esperados para a fonte de xadrez. O comando documentado foi executado: quatro
fichas verificadas, zero referências inválidas e IDs na ordem esperada. A
repetição no mesmo destino foi rejeitada e todos os bytes do bundle permaneceram
iguais. O bundle dessa conferência foi movido para uma pasta temporária, deixando
o destino do roteiro livre para o usuário. Nenhuma suíte foi repetida nesta etapa.
