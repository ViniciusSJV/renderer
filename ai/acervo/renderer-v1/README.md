# Acervo inicial do renderer — edição 1

`catalog.json` identifica sete fontes por repositório, revisão Git, caminho,
SHA-256 e papel documental. `dossier.json` cadastra nove afirmações manuais;
oito são selecionadas. O código completo de câmera e mundo fica cadastrado,
mas apenas os trechos pertinentes entram no contexto exportado.

Os papéis distinguem descrição de código, protocolo declarado, medição registrada,
estatística derivada, resultado de teste relatado e limite declarado. Hipóteses
e avaliações semânticas têm listas separadas, vazias nesta edição. Esses papéis
são metadados do adapter: o Graph Engine não classifica nem comprova afirmações.

A revisão `42e6d4dcd2ce1b2fdd827e3d038b356e02c8f5e3` identifica os bytes
cadastrados, não o binário da medição histórica. Não há promessa de snapshot
atômico ou autenticação. Não alterar hashes para acomodar mudanças futuras:
criar outra edição de acervo e preservar esta.

## Reproduzir a consulta determinística

Na raiz do renderer, com destino novo:

```sh
python3 ai/acervo/renderer-v1/validate.py /tmp/renderer-bundle-novo
```

O script confere identidade, bytes, linhas e edição Git, recalcula cada rodada
e as estatísticas globais e usa a CLI fixada no renderer para validar o dossiê
inteiro, selecionar e exportar. Depois confere ordem, limites e hashes do bundle.
Para testar um checkout independente do Librarian:

```sh
python3 ai/acervo/renderer-v1/validate.py /tmp/librarian-bundle-novo --librarian Librarian
```

O cadastro é explícito nos JSONs, não uma extração de fatos. Os caminhos das fontes
são relativos à raiz do renderer; o script estabelece esse diretório. O bundle
preserva textos, mas nova conferência de arquivos ainda exige as fontes disponíveis.

## O que os registros permitem responder

O protocolo/código descreve render em 96×64, três aquecimentos e cinco amostras
por execução (`F_EXTERNAL_PROTOCOL`, `SRC_EXTERNAL_BENCH`), com release e quatro
threads declarados (`F_ENVIRONMENT`, `SRC_PROTOCOL`). O recálculo das sete rodadas
confirma 35 amostras e mediana de **13.232.986 ns**, mínimo de 12.330.831 ns e
máximo de 21.777.221 ns (`SRC_EXTERNAL_BASELINE`, `SRC_EXTERNAL_STATS`).

O cronômetro cobre a chamada de render, após preparar a cópia do mundo
(`F_TIMING`, `SRC_CPU_CAMERA`). A regressão relatada passou; seu código usa
32×24 e compara digests quantizados (`F_EXTERNAL_VISUAL_DIGEST`,
`F_VISUAL_SCOPE`). Isso não demonstra equivalência universal (`F_EXTERNAL_LIMIT`).

Esta resposta é uma síntese documental revisável, não uma avaliação de LLM.
Recalcular não é medir novamente. Não há comparação antes/depois desta cena,
hotspot dominante demonstrado nem autenticação da máquina/binário/execução.
O ganho histórico de 15,25% pertence a outra cena e não deve ser transferido.

## Rubrica prévia para eventual LLM

Uma futura tentativa deve preservar entrada, resposta e avaliação em novo diretório.
Avaliar separadamente seis critérios, cada um como aprovado/parcial/reprovado,
com trecho da resposta que justifique a nota:

1. Informa resolução, aquecimentos, amostras e ambiente com procedência explícita.
2. Informa 35 amostras e mediana correta, sem chamar mediana de média.
3. Distingue código, protocolo, registro histórico e recálculo atual.
4. Expõe limites de execução, regressão e ausência de comparação antes/depois.
5. Cita IDs pertinentes e não generaliza ganho ou hotspot.
6. Não preenche lacunas nem promove hipótese a fato.

Critérios 3–6 reprovados impedem aceitação semântica. Aceitação plena exige os
seis aprovados; parciais não contam como aprovação. Transporte/HTTP e hashes
recebem avaliação técnica separada. Nenhuma consulta LLM foi feita nesta etapa.
