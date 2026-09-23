# Complemento: consulta unitária e documentação de uso

Após os testes registrados no README deste experimento, não houve alteração
adicional de código. Foi exercitado o cenário documental que faltava: exportar
somente `F_RECORD` com raio zero, reutilizando o dossiê de manutenção previamente
gerado em `/tmp/librarian-maintenance-23`.

O exemplo público `export` concluiu com código zero. A inspeção automatizada do
novo bundle conferiu fact_id, linha 3, contexto de uma linha, lacuna do exemplo
fictício e hashes de dossiê/pergunta/query. Resultado: aprovado.

Não foram repetidos os 23 testes da biblioteca nem os dois de integração. A
compatibilidade dos dois consumidores permanece apoiada no registro anterior
de queries idênticas do acervo. CI e publicação continuam pendentes.

Ao final da etapa foram criados TESTME.md nos dois projetos e WORKFLOW.md no
Librarian. O diagrama distingue cadastro, validação, seleção, exportação,
capturas, pareceres e avaliação LLM futura. Os snapshots e o patch anteriores
permanecem como registros de sua etapa; não incluem a documentação acrescentada
depois. Os novos arquivos vivem nos respectivos repositórios de trabalho.
