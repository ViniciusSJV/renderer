# Fechamento determinístico da versão 0.1

Escopo: cadastro explícito de fontes textuais e afirmações fornecidas pelo
consumidor, conferência, seleção e exportação. O número 0.1.0 já consta dos
manifests; este documento define aceitação funcional, não anuncia uma release.

## Critérios de aceitação

| Critério objetivo | Evidência / estado |
| --- | --- |
| Contratos reutilizáveis fora do renderer | Core genérico e exemplo de manutenção independente. |
| Rejeitar IDs vazios/duplicados, afirmações vazias e referências inexistentes | Testes do core e `public_pipeline`; fechamento adiciona validação de campos em todos os fatos. |
| Conferir fontes fora da seleção e propagar falhas do armazenamento | `public_pipeline`, incluindo fonte alterada não selecionada. |
| Seleção explícita, ordenada, sem duplicatas; contexto limitado à fonte | Testes públicos; raio zero permitido e raio máximo sem overflow. |
| Preservar pergunta, dossiê, seleção, limites e hashes; recusar destino existente | `PreparedQuery::write_bundle` e testes de exportação. |
| Consumidor independente exporta arquivos reais sem LLM | `maintenance-example` e exemplo genérico `export`. |
| Consumidor renderer mantém consultas válidas compatíveis | Acervo e relatório do experimento 23 no renderer; revisão Git permanece fixa. |

## Auditoria da API pública

- `librarian-core`: `LibrarianFact`, `Source<Execution, Capture>`, `Evidence`,
  `Selection`, `EvidenceBundle`, `QueryExport`, `ValidationOutcome` e alias
  `BibliotecarioFact`. Campos públicos permitem montar valores inválidos;
  construir ou desserializar não equivale a validar.
- Caminho recomendado: cadastrar `Evidence` → serializar → `prepare_query`
  com callback → `PreparedQuery::write_bundle`. `validate_dossier` permite
  conferência isolada. Cada operação deve receber um novo `SourceChecks`.
- `SourceChecks` confere bytes/linhas e capturas quando presentes. Fontes inline
  sem path/hash são permitidas; a aplicação precisa exigir esses campos para
  acervos de arquivos. O exemplo `export` faz essa exigência. `git_commit`,
  `kind` e autoria são declarações; comparar conteúdo ao Git é tarefa do adapter.
- `EvidenceBundle::to_query`, `query_json`, `bundle::write` e helpers públicos
  de seleção são APIs de baixo nível. Não garantem a validação integral do
  dossiê. Não usar essas funções como selo de evidência conferida.
- `Selection::validate` do contrato legado exige contexto positivo; o Graph
  Engine aceita raio zero. São interfaces diferentes; preferir `prepare_query`
  para dossiês multifonte. Nenhuma migração silenciosa neste fechamento.
- `Review`/`validate_review` conferem a ligação de um parecer, não a correção
  semântica do veredito. `capture` confere consistência documental; `metrics`
  instrumenta operações, sem medir verdade ou qualidade da explicação.
- Erros são `String`; não há códigos estáveis de erro. Formatos legados são
  mantidos. A escrita pode deixar diretório parcial e não reconfere fontes.
- A query de uma ficha tem `evidence.source.context`; múltiplas fichas usam
  `evidence.selections` e `evidence.contexts`. Consumidores precisam tratar
  ambos os formatos existentes; esta etapa não os unifica.

## Correção necessária identificada

O Graph Engine conferia existência de referências, mas não aplicava a validação
dos campos de `LibrarianFact`. A correção rejeita IDs e afirmações vazias,
inclusive fora da seleção, e IDs de fonte vazios. É um endurecimento de entradas
inválidas; formatos e saída de entradas válidas são preservados.

## Fora do escopo e pendências de publicação

Não são requisitos desta 0.1: banco NoSQL/grafo, catálogo persistente, extração
automática de fatos, biblioteca LLM, garantia semântica ou autenticação de execução.
O acervo inicial usa edições manuais e hashes; não promete gestão geral de versões.

A suíte local e o consumidor validam este fechamento funcional. Publicação/tag,
CI da nova revisão em Windows e eventual avanço da revisão do renderer são etapas
separadas, ainda não realizadas. A compatibilidade observada no acervo não substitui
os checks completos necessários quando essa dependência for atualizada.
