# Aula 24 — Contrato e critérios de conclusão dos engines

## Conceito

Separar três resultados: evidências conferidas, resposta recebida e resposta
avaliada. A primeira etapa não garante correção semântica das fichas; a segunda
não garante correção da explicação. Cada etapa precisa de identidade e limites.

## Contrato documentado

Criamos [engines-v1.md](../contratos/engines-v1.md), com responsabilidades,
entrada/saída, ligação entre artefatos, estados de erro e critérios de conclusão.
A consulta existente mantém question, evidence e instructions; a futura tentativa
terá envelope separado. O contrato não é ainda código nem especificação da API
Ollama: os detalhes dessa API serão conferidos na implementação.

O cliente será Rust e usará Ollama, inicialmente com Qwen. DeepSeek permanece
uma possibilidade a avaliar, sem troca realizada. O modelo será configurável.
Precisamos definir a conexão entre cliente e Ollama considerando Codespaces e
Windows antes da chamada real; nenhum ajuste de rede foi executado nesta aula.

## Revisão do desenho

Examinamos o formato produzido por query_json, o Modelfile e o estado registrado
no README. O contrato cobre quatro situações conceituais:

- Evidência inválida impede envio.
- Falha de comunicação não vira resposta avaliada nem tenta novamente em silêncio.
- Resposta recebida pode ser incompleta ou semanticamente errada.
- Consulta histórica não é promovida a conferência atual apenas por possuir status.

Isso é revisão do desenho, não teste executável da futura integração. Não
alteramos Rust, executamos testes, medimos latência ou chamamos o Ollama.

## Medição e explicação

A matriz tem **12 critérios: 3 implementados, 2 parciais e 7 pendentes**.
A contagem não representa porcentagem de esforço. Os três implementados dizem
respeito à base de evidências, testada até a Aula 23. As principais pendências
estão no cliente, no registro de tentativas e no ciclo integrado.

Não precisamos de banco de grafos ou otimização adicional do cache para fechar
o contrato v1. Também não exigimos acerto perfeito do Qwen: precisamos conseguir
identificar e avaliar erros com evidências, sem confundi-los com falhas de transporte.
Os objetivos de CPU e cenas continuam posteriores ao fechamento dos engines,
podendo servir como casos de validação quando necessário.

## Fechamento e próxima aula

A **Aula 24 está concluída como definição de contrato**, não como implementação
da integração. Graph Engine e LLM Engine ainda não estão ambos fechados.

Na **Aula 25 — Primeira comunicação Rust–Ollama**, vamos definir a configuração
de conexão, implementar o adaptador e testar uma chamada. Se a topologia exigir
uma decisão do usuário, preparar a parte local e os testes independentes antes
de depender da conexão real. Não presumir que o localhost do Codespaces seja
o Ollama do Windows. A próxima aula continua dedicada aos engines, não inicia
automaticamente os objetivos finais.
