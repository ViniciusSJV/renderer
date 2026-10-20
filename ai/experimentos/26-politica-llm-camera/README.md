# Política restritiva do LLM — consulta da câmera

Em 27/09/2026 foram executadas duas consultas reais pelo PLAYME, com a pergunta
“Onde a câmera calcula a direção dos raios?”. Ambas receberam apenas E1:
`impl Camera::ray_from_pixel`, `src/camera.rs:75–89`. A busca preserva outros
candidatos e grafo, mas eles não foram enviados ao modelo.

## Alterações e resultado

O Modelfile passou de orientação ampla a ray tracing/performance para uma política
de explicação das evidências. INFERENCE/HYPOTHESIS requerem pedido explícito;
lacunas não autorizam preenchimento externo; nenhuma categoria é obrigatória.
PLAYME e TESTME receberam instruções e rubrica compatíveis. O contrato registra
o escopo novo sem atribuí-lo retroativamente ao caminho formal PreparedQuery.

| Tentativa | Política | Transporte | Avaliação |
| --- | --- | --- | --- |
| [01](https://github.com/ViniciusSJV/renderer/blob/a6e56ba19213d879f5de6e3e07f8a75ef84fe569/ai/consultas/policy-camera-20260927-01/EVALUATION-01.md) | Categorias condicionadas ao pedido e fatos restritos à evidência | HTTP 200, completed, 10.132,3363 ms no cliente | Reprovada: atribui espaços geométricos, omite fórmula literal e acrescenta lacuna irrelevante. |
| [02](https://github.com/ViniciusSJV/renderer/blob/a6e56ba19213d879f5de6e3e07f8a75ef84fe569/ai/consultas/policy-camera-20260927-02/EVALUATION-02.md) | Acrescenta descrição literal, concisão em localização e proibição de deduzir espaço geométrico dos nomes | HTTP 200, completed, 3.094,9217 ms no cliente | Reprovada: fórmula correta, mas ainda afirma espaço do mundo e produz lacuna imprecisa sobre operações presentes. |

As latências descrevem chamadas diferentes, com estado de carga/cache potencialmente
distinto; não demonstram melhoria de desempenho. Não houve repetição controlada.
Query e SYSTEM mudaram entre tentativas, impedindo atribuição causal isolada.
Ausência de títulos INFERENCE/HYPOTHESIS não implica ausência de inferências.

## Artefatos e verificação

Cada pasta preserva edição, busca/grafo, query/prompt, critérios anteriores à
geração, Modelfile solicitado e reportado, parâmetros, logs, request exato,
retorno bruto, texto e resultado do transporte. `verification.json` registra
14 checks aprovados por tentativa: quatro hashes do transporte, correspondências
prompt/query/texto, query preparada, prévia do request, hashes de busca/manifesto,
trecho reconstruído do snapshot, linhas, hash da fonte e SYSTEM reportado.

A prévia de prepare_ollama tem um LF final adicional em relação ao request efetivo;
essa diferença é produzida explicitamente pelo preparador. Ambos preservam o mesmo
prompt. Não são apresentados como arquivos byte a byte idênticos.

As métricas reportadas pelo provedor ficam no resumo; o array context continua
somente no retorno bruto. Métricas não são evidência sobre performance do renderer.
Conferência de hashes não autentica execução nem aprova afirmações do modelo.
Os registros originais continuam com avaliação pendente; os pareceres separados
identificam request_id e hash do texto. Nenhuma resposta foi promovida a fato.

Build dos clientes e do ingestor passou nas duas execuções. PLAYME e os 15 blocos
PowerShell do TESTME passaram na análise sintática. `git diff --check` passou.
Nenhum código Rust mudou e a suíte Rust completa não foi repetida.
Os artefatos novos têm proteção -text em .gitattributes para preservar seus hashes.

## Reproduzir e continuar

Na raiz do renderer, com diretório de saída novo:

```powershell
.\PLAYME.ps1 -Question 'Onde a câmera calcula a direção dos raios?'
```

O comando usa a política atual, correspondente à segunda tentativa, e gera outra
resposta a ser avaliada. A configuração não garante obediência. O PLAYME inicia
Ollama se necessário, aplica o Modelfile e deixa o servidor disponível.

Etapa 1 não fechada. Próximo experimento deve testar se uma saída mais restrita
às afirmações/citações verificáveis evita as extrapolações observadas; critérios
e configuração precisam ser registrados antes da resposta. Não trocar silenciosamente
o modelo nem apagar falhas para obter aprovação. Permanecem os casos integral,
parcial, impossível, inferência e hipótese, com repetições e avaliação por afirmação.

A extração para Librarian ainda não foi realizada. Seu objetivo é reutilizar o
preparador/transporte existentes e seus testes, mantendo modelo/parâmetros como
configuração e Ollama como adaptador. Não incluir domínio renderer no núcleo,
nem misturar completed com aprovação semântica. DeepSeek será outra configuração
a avaliar; trocar modelo não deve alterar o Graph Engine. Nenhuma etapa de patches
ou edição automática de código foi implementada.
