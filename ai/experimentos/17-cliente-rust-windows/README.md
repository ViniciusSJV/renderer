# Experimento 17 — primeira chamada concluída pelo cliente Rust no Windows

Origem: saídas de result.json e response.txt coladas pelo usuário para
OLLAMA_WINDOWS_RUST_02. Não é captura direta do assistente no Windows.

- HTTP 200, completed, corpo completo segundo o registro.
- Modelo solicitado: renderer-analyst:latest; modelo reportado não conferido,
  pois response.bin ainda não foi recebido.
- Tempo medido no cliente: 7682,6093 ms (7,683 s arredondados).
- Consulta: 8203 bytes; requisição: 8739 bytes; corpo recebido: 18171 bytes.
- Timeout: 120000 ms; limite: 1048576 bytes; nenhuma opção de geração explícita.
- Consulta histórica da Aula 17: evidence_rechecked=false, origem/seleção nulas.
- Tentativa 01 preservada no Windows, sem result.json segundo o usuário.

resultado-transcrito.json normaliza os escapes Markdown nos nomes dos campos
e o link inserido na URL pelo texto colado. Não é cópia byte a byte do arquivo
Windows. Os caminhos em files referem-se à pasta Windows da tentativa, não a
arquivos recebidos nesta pasta do Codespaces.

resposta-transcrita.md preserva o conteúdo textual recebido com escapes Markdown
nos IDs. Seu hash local difere do text_sha256 declarado pelo cliente; portanto
não representa os bytes originais de response.txt. Isso não demonstra corrupção:
a passagem pelo chat e a transcrição podem alterar formatação/quebras de linha.
Não foram conferidos os hashes do corpo HTTP nem do texto originais.

avaliacao.json aplica retrospectivamente os seis critérios preexistentes da
Aula 17 à nova resposta: **3/6** (C1, C4 e C6). Não houve nova rubrica registrada
especificamente antes desta geração. As notas históricas não foram alteradas;
igualdade numérica não é comparação controlada nem ganho demonstrado.

C2 é parcial por explicação insuficiente do pânico esperado, não por negação
explícita: “não gerou um pânico inesperado” não explica que should panic ... ok
registra aprovação por pânico esperado. C3 omite as fichas e C5 omite limites e
contagens exigidos. A avaliação do conteúdo não altera o estado completed do
transporte nem reescreve semantic_evaluation no registro original.

Próximo passo: receber os artefatos originais da tentativa 02 e conferir seus
hashes. Depois avançar à integração com evidências recém-conferidas, origem e
seleção registradas, e avaliação previamente vinculada à nova consulta.
Aula 25 e engines ainda não declarados concluídos.

## Originais recebidos e conferidos

O ZIP fornecido em `ai/aulas/tentativa-ollama-rust-02.zip` foi inspecionado;
seus sete arquivos foram preservados byte a byte em [originais](originais/).
A [conferência](conferencia-originais.json) confirmou os quatro hashes, tamanhos,
identidade da tentativa, coerência prepared/result, consulta histórica exata,
prompt integral no corpo enviado e texto extraído igual ao response do JSON bruto.
O result transcrito corresponde ao original. A resposta transcrita difere por
escapes Markdown e espaço final; a leitura do original mantém a avaliação 3/6.

O retorno bruto confirma renderer-analyst:latest, done=true e done_reason=stop.
Métricas declaradas pelo servidor: 7,6739318 s totais, 0,0020681 s de carga,
2998 tokens de prompt e 450 gerados. A latência do cliente foi 7,6826093 s.
Esses campos não provam ausência de truncamento interno do prompt nem vantagem
sobre a chamada anterior. Não fizemos nova geração para esta conferência.

A pendência de receber originais foi resolvida. Próximo passo: integração de
conferência/exportação atual com envio, origem/seleção e avaliação prévia.
O registro original continua semantic_evaluation=pending; a avaliação manual
fica no arquivo separado, sem reescrever o artefato histórico.
