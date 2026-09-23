# Cena externa — dossiê inicial

Este pacote registra a primeira baseline da cena externa maior, antes de qualquer nova otimização.

## Escopo

- Cena: plano e nove esferas, câmera fora dos objetos.
- Resolução: 96x64.
- Perfil: release.
- Threads: `RAYON_NUM_THREADS=4`.
- Aquecimentos: 3 por execução.
- Amostras: 5 por execução, 7 execuções, 35 amostras no total.
- Mediana global: 13.232.986 ns.
- Regressão visual em 32x24: digest `f8f450596cf93972fc99ffee22cbaf4126055156ac3a511170f0088bcae4abf6`.

O log de benchmark registra a saída observada, mas não autentica retrospectivamente os binários nem a máquina que executou a coleta. A regressão visual é um teste determinístico local, não uma prova universal de equivalência.

## Ordem do ciclo

1. Bibliotecário: fatos e seleção pequena.
2. Graph Engine: conferência de fontes, hashes, linhas e seleção.
3. LLM Engine: interpretação limitada da baseline e hipóteses de custo.
4. Avaliação: rubrica prévia, sem transformar hipóteses em resultados medidos.

## Consulta e avaliação

O envio integrado foi concluído no Windows em `attempt-02`: HTTP 200, corpo
completo, `evidence_rechecked=true` e modelo `renderer-analyst:latest`.

A avaliação está em `avaliacao-windows-01.json`: quatro critérios aprovados e
um parcial. A resposta acertou protocolo, mediana e limites, mas não citou
explicitamente os IDs `F_EXTERNAL_*` e `SRC_EXTERNAL_*`. Fora da rubrica, usou
"médio" ao descrever uma mediana. A resposta não foi promovida a fato novo.

## Tentativa 03

O ciclo foi repetido no Windows após a separação física do Graph Engine.
`attempt-03` terminou com HTTP 200, `completed`, `evidence_rechecked=true` e
corpo completo. A consulta e o texto da resposta são idênticos aos da
`attempt-02`; esta tentativa confirma a integração pós-migração, mas não é uma
nova avaliação semântica nem uma comparação controlada de modelo.

## Disponibilidade dos registros

O arquivo `ai/experimentos/attempt-02.zip` citado na avaliação não está disponível
nesta árvore. A avaliação foi preservada como registro anterior. A `attempt-03`
está preservada e seus hashes foram conferidos; a igualdade com a tentativa 02
se apoia nos hashes registrados, sem nova comparação direta dos dois ZIPs.
