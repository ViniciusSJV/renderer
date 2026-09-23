# Librarian 0.1: auditoria e acervo determinístico

## Estado inspecionado

Renderer em `42e6d4dcd2ce1b2fdd827e3d038b356e02c8f5e3`; Librarian em
`822a7edd0e82e69b07a175d6cae2de12cee5c331`, acima da revisão consumida
`4e84f004061b1df84ae5d8f22af9007aa2d99962`. Ambos começaram limpos.
As referências locais de acompanhamento apontavam para os mesmos HEADs.
A consulta posterior `git ls-remote` confirmou esses mesmos hashes nas branches
remotas master e main: os dois commits citados do renderer já estão publicados,
diferentemente da informação inicial sobre commits sem push.

Foram lidos os READMEs dos dois projetos e do laboratório, os relatórios 21/22,
o relatório CPU e o dossiê externo; foram inspecionados contratos, validação,
filesystem/capturas, exportação, exemplo independente e testes públicos.

## Critério de fechamento

O escopo utilizável é cadastro explícito → validação integral → seleção ordenada
com contexto limitado → bundle sem sobrescrita. Não exige banco, extração de
fatos ou LLM. A auditoria detalhada e os critérios ficam em `Librarian/V0.1.md`
no repositório independente. Sua cópia e o patch desta etapa estão preservados
aqui porque esse checkout é ignorado pelo renderer.

A primeira falha encontrada era a aceitação de IDs/afirmações vazios no Graph
Engine, apesar da validação disponível no core. Foi corrigida com testes de
campos inválidos fora da seleção. Também foram exercitados ID de fonte duplicado,
pergunta vazia, erro do callback e contexto máximo.

Limites documentados: callbacks definem o alcance da conferência de armazenamento;
fontes inline são permitidas; exports de baixo nível não validam tudo; contexto
zero difere do contrato legado; formato de uma ficha difere do formato múltiplo;
hashes e pareceres não comprovam verdade ou execução. A publicação de uma release
e a CI da nova revisão no Windows continuam pendentes.

## Acervo e validação

O [acervo edição 1](../../acervo/renderer-v1/README.md) contém sete fontes,
nove afirmações manuais e oito selecionadas. Código CPU, protocolo declarado,
amostras, estatísticas e regressão têm identidade, edição e procedência explícitas.
Categorias documentais, hipóteses e avaliações ficam separadas no cadastro.

O script confere bytes contra hashes, linhas e conteúdo da revisão Git; recalcula
as sete rodadas e os totais; exporta pela API pública e verifica ordem, limites e
hashes do bundle. Resultado: 35 amostras, mediana 13.232.986 ns, mínimo
12.330.831 ns e máximo 21.777.221 ns. Não é uma nova medição.

- Librarian local: **23 testes aprovados**, zero falhas.
- Integração do renderer com a dependência fixa: **2 aprovados**, zero falhas.
- Exemplo de manutenção independente: exportação concluída em destino temporário.
- Query do novo acervo: bytes idênticos entre biblioteca local corrigida e CLI
  do renderer com revisão fixa. `validation.json` registra o hash.
- `git diff --check` passou nos dois repositórios.

Estes números não são uma repetição da suíte histórica completa de 338 testes.
A revisão da dependência não foi atualizada. A compatibilidade exercitada cobre
os cenários descritos; uma futura atualização exige validação própria do consumidor.

`bundle/` preserva a exportação local; logs registram os dois caminhos. Os caminhos
absolutos em `origin.json` são rótulos da execução, não requisitos de portabilidade.
Nenhum arquivo histórico foi reescrito, benchmark ou consulta Ollama foi repetido.
Uma rubrica prévia de seis critérios está no README do acervo; avaliação semântica
permanece não realizada e separada da validação determinística.
