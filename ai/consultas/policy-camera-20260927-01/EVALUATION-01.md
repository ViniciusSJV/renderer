# Avaliação da primeira política restritiva — câmera

Pergunta: Onde a câmera calcula a direção dos raios?
Request: PLAYME_308f655721c34c1d9aa44d2160ace15b
SHA-256 do texto: 55dd79d337406ff41dfe924f6b3bd010a5b9ed804d050e9b6426d607265b9052
Rubrica anterior: criteria-before.md. Resposta original: ollama-01/response.txt.

Resultado: transporte completed; resposta semanticamente reprovada.
A avaliação foi realizada pelo assistente após a geração, sem reescrever o retorno.

| Critério prévio | Resultado | Evidência |
| --- | --- | --- |
| Localização/citação | Aprovado | Identifica impl Camera::ray_from_pixel, E1, src/camera.rs e 75–89. |
| Fórmula/operandos | Parcial | Diz “subtraindo a origem do ponto e normalizando”; ordem compatível, mas não reproduz a fórmula explicitamente solicitada. |
| Fatos sustentados | Reprovado | “coordenadas no mundo” e “no espaço da câmera” atribuem semântica geométrica não demonstrada por E1. |
| Ausência de inferências/hipóteses não solicitadas | Reprovado | Não há esses títulos, mas a interpretação de espaços foi apresentada dentro de FACT. |
| Categorias autorizadas | Não aplicável | A pergunta não pede inferência ou hipótese; ausência dos títulos não basta para aprovação. |
| Execução/desempenho | Aprovado | Não alega execução de teste ou medição. |
| Lacunas pertinentes | Reprovado | Inicialização da câmera e definição de sua transformação não são necessárias para localizar a operação pedida. |
| Parte respondível/sem propostas | Parcial | Localiza corretamente e não propõe mudanças; a descrição excede a evidência e omite a fórmula literal. |

Exame das afirmações: o passo 1 resume as expressões de offset, mas “tamanho do
pixel” é uma leitura do nome pixel_size, não uma definição fornecida. O passo 2
deve ser limitado às subtrações half_width - offset_x e half_height - offset_y;
os nomes world_x/world_y não provam sistema de coordenadas. O passo 3 deve citar
as multiplicações escritas e as chamadas Tuple::point, sem atribuir espaço
geométrico nem garantir o comportamento interno de inverse. O passo 4 descreve
a ordem correta, mas deveria reproduzir (pixel - origin).normalize(). O passo 5
pode afirmar que o código retorna a chamada Ray::new(origin, direction), sem
validar sua implementação ausente. A resposta termina com uma lacuna desnecessária.

Não foi feita comparação controlada com manual-001: query e SYSTEM mudaram.
O resultado sustenta somente que remover títulos obrigatórios não elimina
interpretações inseridas em FACT. Não há fechamento da Etapa 1.

A rubrica preservada contém um erro de exibição em “respondível” devido à leitura
do script pelo Windows PowerShell; o sentido do critério permanece identificável.
O script foi corrigido para usar ASCII nesse ponto; a rubrica original foi mantida.

Próxima tentativa: manter pergunta e trecho, explicitar descrição literal de
expressões e evitar converter identificadores em interpretação geométrica.
Preservar nova query, configuração, resposta e rubrica em outro diretório.
