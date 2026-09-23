# Primeira fonte real do xadrez-angular

Fonte: `src/app/_servico/xadrez.service.ts` do repositório
[xadrez-angular](https://github.com/ViniciusSJV/xadrez-angular), revisão
`18fdab8329b2f680c731a1929132caed67e33587`.

`xadrez.service.ts` preserva os bytes comparados com `git show` dessa revisão.
O catálogo registra origem, hash, papel e seleção; o dossiê contém quatro
afirmações manuais, referências e lacunas. O snapshot permite reconferência
sem depender do clone temporário. Não editar esta edição para acompanhar o upstream.

Na raiz do renderer, com destino novo:

```sh
cargo run --locked --bin validate_evidence -- ai/acervo/xadrez-v1/dossier.json --fact F_INIT --fact F_POSITION --fact F_PLAY --fact F_MOVE --context 1 --question ai/acervo/xadrez-v1/question.txt --bundle ./xadrez-bundle
```

O Graph Engine da revisão `a009af8` confere a fonte e exporta quatro fatos:

| ID | Chamada declarada no código |
| --- | --- |
| F_INIT | GET `/api/init` |
| F_POSITION | GET `/api/row/` + posição |
| F_PLAY | PUT `/api/rows/jogada`, enviando a posição selecionada |
| F_MOVE | PUT `/api/rows/mover/` + posição, enviando a posição selecionada |

Isso descreve declarações do serviço. Não prova chamada realizada, backend
disponível, regras legais de xadrez ou funcionamento da interface. Não instalamos
dependências Angular nem executamos a aplicação. O cadastro reutiliza o contrato
para outro projeto; ainda não integra o Librarian ao código da aplicação Angular.

A seleção é por IDs, não por interpretação automática da pergunta. Nenhuma
avaliação LLM foi feita. Antes de uma futura resposta LLM, definir rubrica para
verbo/rota/corpo, referências corretas e preservação desses limites.
