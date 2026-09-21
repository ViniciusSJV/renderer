# Revisão das aulas 13 a 21

## Onde estamos em relação aos objetivos

Os objetivos finais continuam sendo:

1. Descrever cenas em linguagem natural, transformar a resposta do Qwen em uma
   estrutura validada e construir World, câmera, objetos, materiais e luzes para
   renderizar no projeto Rust.
2. Melhorar o desempenho do Ray Tracer em CPU, sem RTX/GPU, usando profiling,
   testes e benchmarks para demonstrar ganhos no trabalho de renderização.

As aulas 13–21 construíram e exercitaram infraestrutura de evidências. Ainda
não implementamos o fluxo linguagem natural → cena validada → renderer, nem
fizemos profiling ou demonstramos ganho de tempo de renderização nesse ciclo.
Os benchmarks e a otimização realizados mediram o Bibliotecário.

A Aula 4 usou tuple.rs como primeira obra real para acompanhar o livro. Depois,
priorizamos o Bibliotecário, usando tuplas e a fronteira de equivalência como
casos de estudo. Não avançamos por outros capítulos durante estas aulas.

O mecanismo de captura/conferência é reutilizável entre arquivos e comandos:
não depende da semântica de tuplas, câmera ou materiais. Contudo, os dossiês e
capturas concretos dependem dos caminhos, hashes, linhas e arquivos selecionados.
O Bibliotecário não descobre sozinho o significado do código ou todos os arquivos
que influenciaram uma compilação. Reutilizável não significa completo ou autônomo.

## Aula 13 — Captura de execução

Transformamos a captura pontual em um procedimento reutilizável. O protótipo
Python foi substituído pelo binário Rust capture_execution; os registros antigos
não foram reatribuídos ao Rust. O capturador grava comando/argumentos, saída
combinada, resultado, horários e ambiente parcial.

Aprendizado: captura concluída não significa comando aprovado. Código 7 pode
estar corretamente registrado; comando que não iniciou não tem código de término
normal. A versão Rust teve oito testes aprovados. O procedimento tem escopo Unix.

## Aula 14 — Associação entre código e execução

Acrescentamos --source repetível, hashes de arquivos antes/depois e hash da saída,
com formato versão 2. Onze testes do capturador passaram. Uma nova execução,
RUN_EQUIVALENCE_BOUNDARY_2, registrou quatro testes aprovados e cinco fontes com
hashes iguais nas duas leituras.

Aprendizado: hashes iguais nas duas leituras não provam imutabilidade durante
todo o intervalo, nem identificam por si só os bytes usados pelo compilador.

## Aula 15 — Conferir o registro de captura

O Bibliotecário ganhou --capture para conferir a estrutura básica da versão 2,
a saída e as associações registradas. Recalcula tamanho/hash da saída e compara
arquivos atuais com hashes posteriores disponíveis. A suíte passou a 68 testes.

Aprendizado: consistência local não é autenticação. Um arquivo atual divergente
impede essa conferência, mas não refuta automaticamente o registro histórico.

## Aula 16 — Levar a conferência ao dossiê

Ligamos uma fonte de saída à captura por caminho, SHA-256 do registro e ID de
execução. Um dossiê separado reuniu uma fonte e duas fichas, sem referências
inválidas. A exportação passou a incluir a conferência e seus limites. Foram
72 testes aprovados.

Aprendizado: TEST_BOUNDARY_2 identifica a fonte documental;
RUN_EQUIVALENCE_BOUNDARY_2 identifica a execução. Um link não resolve divergência
de hash e uma ficha referenciada não se torna semanticamente verdadeira.

## Aula 17 — Avaliar a explicação da captura

Definimos seis critérios antes da resposta, geramos a consulta e recebemos a
resposta do Qwen manualmente. A avaliação foi 3/6: acertou identidades, distinção
entre comando/conferência e limites de generalização; errou a interpretação do
pânico esperado, omitiu referências de fichas e atendeu parcialmente aos limites.

Aprendizado: should panic ... ok registra aprovação com pânico esperado. Não
autenticar a execução não elimina o significado do resultado relatado. A nota
da Aula 10 também permanece 3/6, sem comparação controlada entre tarefas.

## Aula 18 — Medir o custo da conferência

Criamos bench_evidence em Rust e um protocolo anterior ao ensaio. Medimos a CLI
release com uma fonte e 1, 10 e 100 fichas: três aquecimentos e 15 amostras por
carga. As medianas foram 1,563 ms, 3,558 ms e 22,687 ms. Os dois testes do medidor
passaram.

Aprendizado: medimos inicialização, leitura, conferência e exportação em conjunto,
não o custo isolado de hashes e não o tempo de renderização. A saída também
cresce com as fichas. Ambiente compartilhado e caches limitam a interpretação.

## Aula 19 — Investigar conferências repetidas

Instrumentamos chamadas e bytes nas leituras de conferência. Com 1, 10 e 100
fichas da mesma fonte, observamos 2, 11 e 101 conferências da captura. Os bytes
instrumentados foram 41.880, 227.910 e 2.088.210. Os 72 testes passaram e os hashes
das exportações permaneceram iguais.

Aprendizado: localizamos trabalho repetido. Bytes entregues à aplicação não são
tráfego físico de disco, e contagens não determinam a participação no tempo total.

## Aula 20 — Reaproveitar dentro de uma exportação

Passamos a reutilizar o resultado por ID de fonte durante uma exportação,
sem cache entre exportações/processos. As três cargas passaram a duas conferências
e 41.880 bytes instrumentados cada. Foram 73 testes aprovados, com exportações
idênticas às anteriores.

Na comparação local entre binários anterior e novo:

| Fichas | Mediana anterior | Mediana nova |
| --- | --- | --- |
| 1 | 1,569 ms | 1,592 ms |
| 10 | 3,619 ms | 2,044 ms |
| 100 | 26,513 ms | 5,997 ms |

Aprendizado: observamos ganho nas cargas maiores do Bibliotecário, sem garantia
universal. A comparação foi sequencial, sem isolamento da máquina. Reutilizar
uma leitura não oferece snapshot atômico e pode deixar alterações posteriores
sem detecção durante aquela exportação.

## Aula 21 — Testar várias fontes

Mantivemos 100 fichas e distribuímos entre 1, 10 e 100 fontes documentais da
mesma captura. Observamos 2, 20 e 200 conferências, com 41.880, 418.800 e
4.188.000 bytes instrumentados. Uma ligação inválida foi recusada sem exportação.
Não alteramos Rust nem medimos tempo nesta aula.

Aprendizado: o reaproveitamento é por fonte, não por captura compartilhada.
IDs documentais distintos não significam arquivos físicos ou execuções distintos.

## O que já podemos usar e o que falta

Já podemos registrar comandos de testes/benchmarks, identificar arquivos
selecionados, conferir registros, ligar resultados a fichas e exportar contexto
para análise manual com o Qwen. Não há banco de grafos nem integração automática
com o Ollama. Não autenticamos execução nem comprovamos retrospectivamente
RUN_VECTOR_1.

Para o objetivo de CPU, falta escolher cenas/cargas do renderer, registrar uma
linha de base de renderização, fazer profiling, escolher melhorias e comparar
correção e desempenho. A infraestrutura existente já pode apoiar esse trabalho.
Continuar refinando o cache do Bibliotecário não é um pré-requisito obrigatório.

Para cenas por linguagem natural, falta definir o contrato da cena, implementar
validação estrutural e de domínio e a construção das estruturas do renderer,
integrar a geração pelo Qwen e testar o fluxo completo. A avaliação de explicações
de evidências não mede a capacidade do Qwen de gerar cenas corretas.

## Pausa e retomada

Paramos após a Aula 21, a pedido do usuário. A Aula 22 sobre captura compartilhada
ficou como proposta, não como trabalho iniciado ou dependência dos objetivos.
Ao retomar, vale escolher explicitamente entre continuar esse refinamento e
aplicar a base existente ao renderer em CPU, mantendo a criação de cenas por
linguagem natural na sequência acordada. Nenhuma nova implementação foi feita
para avançar a aula nesta revisão.
