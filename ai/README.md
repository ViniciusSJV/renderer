# Laboratório de IA, evidências e performance

**Retomada do trabalho:** leia [RETOMADA.md](RETOMADA.md) para o estado validado
e o próximo passo após os experimentos de CPU e as duas consultas no Windows.

Este laboratório usa um ray tracer em Rust baseado em *The Ray Tracer Challenge*
para estudar código, testes, medições e explicações de um modelo local. As 25
aulas formam uma sequência: configurar o modelo, atomizar informações, conferir
referências, registrar execuções, investigar custos e realizar a primeira
comunicação Rust–Ollama.

## Objetivos e arquitetura

O objetivo imediato é construir um ciclo verificável de explicação:

```text
Fonte → ficha → conferência → seleção/contexto → consulta
                                                   ↓
Avaliação manual ← resposta preservada ← modelo via Ollama
```

- **Graph Engine — “Testa sem explicar”:** organiza fontes, fichas e relações;
  confere referências, hashes, capturas e limites; executa testes e medições.
- **LLM Engine — “Explica sem interpretar”:** explica o material recebido,
  distinguindo fato, inferência e hipótese, sem apresentar deduções como observações.

`Source` é a obra; `lines` são os trechos numerados; `Fact` é a ficha com uma
afirmação e um endereço. Não existe um tipo Rust separado chamado `Lines`.
Uma referência válida não significa que a evidência sustenta a afirmação.
Evidência não é conclusão, código não é execução e resposta não é validação.

Os objetivos posteriores registrados são otimizar o renderer em CPU com
profiling, correção e benchmarks e criar cenas por linguagem natural a partir
de uma estrutura validada. Nenhum desses objetivos está concluído pela integração
com Ollama. Os ganhos medidos nas aulas são do Bibliotecário, não do renderer.

## Pré-requisitos gerais

| Recurso | Uso e momento |
| --- | --- |
| Git, editor e terminal | Obter e inspecionar o projeto, desde a Aula 1. |
| Ollama e Qwen | Experimentos manuais desde a Aula 1; cliente Rust na Aula 25. |
| Rust, Cargo e linker do sistema | Compilar e testar a partir da Aula 3. |
| Linux/Bash e utilitários Unix | Capturas reutilizáveis a partir da Aula 13. |
| Python 3, biblioteca padrão | Montar dossiê/cargas locais nas aulas 16 e 21; sem pip. |
| PowerShell no Windows | Roteiro da chamada real Ollama na Aula 25. |

Os registros incluem Linux e Windows. O capturador é Unix e não compila
nativamente no Windows. O cliente HTTP foi testado no Windows antes da extração
final do módulo compartilhado; a versão final do coordenador foi testada em
Linux com servidor simulado. WSL e instalação por Dev Container não têm
procedimento validado no repositório.

**Limite da preparação:** as fontes não preservam instaladores, bootstrap,
requisitos de hardware do modelo ou uma versão mínima de Rust validada.
As aulas indicam as ferramentas a instalar e as verificações de disponibilidade,
mas não apresentam instalação externa como execução comprovada. Os registros
citam Rust/Cargo 1.98.1 e Ollama 0.34.2; esses números não são versões mínimas.

## Obter e preparar o projeto

Em um terminal com Git, a partir de uma pasta de projetos, clone para um destino
novo. O comando baixa o repositório e cria a pasta:

```bash
git clone https://github.com/ViniciusSJV/renderer.git renderer-local
```

Entre na raiz; o comando muda somente o diretório do terminal:

```bash
cd renderer-local
```

O endereço é o remote público registrado. Um clone contém apenas o que foi
publicado: se faltarem aulas ou binários, a edição remota ainda não contém toda
a árvore descrita aqui. Alterações locais não são transferidas automaticamente.
Confira `Cargo.toml`, `Cargo.lock`, `src`, `tests` e `ai` no editor.

Comece pela Aula 1; Rust será preparado na Aula 3. Depois de instalá-lo, confirme
na raiz a versão do compilador, sem modificar arquivos:

```bash
rustc --version
```

Confira também Cargo:

```bash
cargo --version
```

Na Aula 3, obtenha as dependências do lockfile. Isso usa rede e grava cache:

```bash
cargo fetch --locked
```

Compile o Bibliotecário. O comando grava em `target`, sem consultar o modelo:

```bash
cargo build --locked --bin validate_evidence
```

Sucesso confirma as ferramentas necessárias a esse alvo. `--locked` preserva a
resolução do lockfile; `--offline` só deve ser usado com dependências já no cache.
No Windows, selecione binários explicitamente para evitar o capturador Unix.
Ausência de `link.exe` no alvo MSVC exige concluir a instalação das ferramentas
de compilação; Rust disponível sozinho não garante linker disponível.

## Como acompanhar as aulas

Leia em ordem e execute os comandos a partir da raiz, salvo indicação explícita.
Cada aula apresenta o problema, a implementação existente, a prática e os limites
da conclusão. O clone já contém a evolução do código: não é necessário substituir
fontes por recortes didáticos nem reconstruir cada versão antiga.

Resultados com notas, tempos e contagens históricas descrevem as versões e entradas
registradas. A implementação atual pode exportar mais campos ou fazer menos
leituras. Não use resultados antigos como saída prometida de comandos atuais.
Uma nova resposta recebe avaliação própria, com critérios anteriores à geração.

Use destinos novos para exportações e tentativas. A partir da Aula 14, a prática
produz uma captura local; a Aula 16 liga essa captura a um dossiê novo, usado nos
ensaios seguintes. Isso permite executar sem reescrever hashes e caminhos das
capturas históricas. Alterar os arquivos observados pode impedir nova conferência.

## Índice das aulas

| Aula | Conteúdo |
| --- | --- |
| [1 — Entendendo o Modelfile do Ollama](aulas/01-modelfile-ollama.md) | Obtenção do projeto, modelo configurado, categorias e limites das instruções. |
| [2 — Evidências pequenas e critérios explícitos](aulas/02-evidencias-atomizadas.md) | Atomização do exemplo de mutex e avaliação de referências e premissas. |
| [3 — Construindo o Bibliotecário em Rust](aulas/03-bibliotecario-em-rust.md) | Preparação Rust/Cargo, estruturas, JSON e validação estrutural. |
| [4 — Uma obra real: tuplas, código e evidências](aulas/04-tuplas-codigo-e-evidencias.md) | Tuplas, hashes de fontes e distinção entre procedimento e resultado. |
| [5 — Seleção e contexto pelo Bibliotecário](aulas/05-selecao-e-contexto.md) | Seleção por ID, janela de linhas e exportação sem sobrescrita. |
| [6 — Preservar a natureza e a procedência declarada das fontes](aulas/06-metadados-das-fontes.md) | Metadados opcionais e diferença entre declaração e comprovação. |
| [7 — Identidade, limites e uma consulta avaliável](aulas/07-identidade-limites-e-consulta.md) | Identidade do dossiê, questões abertas, pergunta e avaliação histórica 2/5. |
| [8 — Código não é execução](aulas/08-codigo-nao-e-execucao.md) | Registro identificado e comparação de campos com o cabeçalho. |
| [9 — Reunir evidências sem misturar seus papéis](aulas/09-reunir-evidencias.md) | Seleção múltipla e união de janelas sem misturar fontes. |
| [10 — Avaliar uma consulta com código e resultado](aulas/10-avaliar-codigo-e-resultado.md) | Rubrica prévia, identidade e sustentação; avaliação histórica 3/6. |
| [11 — Investigar assert_equivalent!](aulas/11-investigar-assert-equivalent.md) | Cadeia da macro à tolerância absoluta estrita de f64. |
| [12 — Observar a fronteira da tolerância](aulas/12-observar-fronteira-tolerancia.md) | Quatro testes e significado de aprovação com pânico esperado. |
| [13 — Captura de execução](aulas/13-captura-de-execucao.md) | Capturador Unix, saída binária e distinção dos códigos de término. |
| [14 — Associação entre código e execução](aulas/14-associar-codigo-e-execucao.md) | Captura local com hashes antes/depois, sem prova de compilação. |
| [15 — Conferir o registro de captura](aulas/15-conferir-registro-captura.md) | Conferência do formato 2, saída e arquivos atuais. |
| [16 — Levar a conferência ao dossiê](aulas/16-levar-conferencia-ao-dossie.md) | Dossiê local, ligação à captura e exportação dos limites. |
| [17 — Avaliar a explicação da captura](aulas/17-avaliar-explicacao-captura.md) | Avaliação de identidades, pânico e autenticação; histórico 3/6. |
| [18 — Medir o custo da conferência](aulas/18-medir-custo-conferencia.md) | Medidor Rust, aquecimento, amostras e custo total da CLI. |
| [19 — Investigar conferências repetidas](aulas/19-investigar-conferencias-repetidas.md) | Instrumentação de chamadas e bytes lógicos, separada de latência. |
| [20 — Reaproveitar a conferência dentro de uma exportação](aulas/20-reaproveitar-conferencia.md) | Cache por fonte e limites da comparação histórica de tempo. |
| [21 — Testar o reaproveitamento com várias fontes](aulas/21-testar-varias-fontes.md) | Cargas locais com 100 fichas e ligação inválida. |
| [22 — Separar fonte documental de captura compartilhada](aulas/22-separar-fonte-e-captura.md) | Responsabilidades e identidade composta da captura. |
| [23 — Reaproveitar a captura entre fontes](aulas/23-reaproveitar-captura-entre-fontes.md) | Cache local por captura, verificações individuais e contagens. |
| [24 — Contrato e critérios de conclusão dos engines](aulas/24-contrato-dos-engines.md) | Doze critérios para distinguir implementação, transporte e avaliação. |
| [25 — Primeira comunicação Rust–Ollama](aulas/25-primeira-comunicacao-ollama.md) | Preparação, serviço/modelo, requisição, resposta, registros e integração simulada. |

## Estrutura, dependências e comandos gerais

`src/lib.rs` reúne a biblioteca do renderer; seus módulos contêm testes unitários.
`src/bin` contém executáveis do livro e do laboratório; `tests` contém testes de
integração. `ai/experimentos` preserva dados, consultas, respostas, capturas e
medições; `ai/ollama/Modelfile` configura o modelo; `ai/contratos` registra o contrato.

`Cargo.toml` declara Rayon, itertools e png para o renderer; serde/serde_json
para dados; sha2 para hashes; reqwest para HTTP. `Cargo.lock` fixa as versões
resolvidas. Cargo obtém essas dependências; não é preciso instalá-las manualmente.

Após preparar Rust, execute a suíte da biblioteca na raiz. O comando compila e
usa `target`; não inicia Ollama nem os executáveis de cenas:

```bash
cargo test --locked --lib
```

Para o Bibliotecário, selecione seu binário de testes:

```bash
cargo test --locked --bin validate_evidence
```

Observe os resumos de aprovação e investigue falhas. Os comandos de captura,
benchmark e HTTP aparecem nas aulas correspondentes com entradas e destinos
explícitos; não devem ser executados antes de seus pré-requisitos.

## Estado ao final da Aula 25

O Bibliotecário confere referências, conteúdo e capturas, exporta contexto e
preserva origem. O cliente Rust registra requisição, resposta e diagnósticos.
Uma chamada real Windows teve HTTP 200, estado `completed`, 7,683 s no cliente
e avaliação retrospectiva 3/6, com originais conferidos. Esses dados descrevem
uma tentativa, não confiabilidade geral do modelo.

O coordenador une conferência/exportação e envio com rubrica anterior à resposta,
origem e seleção. A suíte desses componentes tem 99 testes aprovados no registro
final Linux, com dois gravadores ignorados por padrão. A integração preservada
é simulada: não comprova um ciclo real completo no Windows.

## Limites e próximos passos registrados

Conferência não autentica execução, não garante snapshot atômico, não comprova
entradas do compilador nem valida semanticamente fichas ou respostas. Não há
banco de grafos, extração geral de fatos ou geração de cenas por texto.
A captura histórica ligada à Aula 17 é recusada na árvore atual porque
Cargo.toml mudou; seus hashes não foram reescritos.

O adaptador HTTP não usa autenticação, HTTPS, proxy ou retries automáticos.
Timeout cobre HTTP; o subprocesso validador do coordenador ainda não tem prazo
próprio. `completed` não significa explicação correta nem processamento integral
comprovado do contexto. Configuração do modelo não garante resposta determinística.

A próxima etapa registrada é a Aula 26: executar o coordenador real no Windows
com dossiê atual e rubrica prévia, preservar e avaliar a resposta e revisar os
critérios pendentes do [contrato](contratos/engines-v1.md). Depois do fechamento
dos engines, o roteiro prevê performance CPU e cenas por linguagem natural.
Uma futura avaliação de DeepSeek via Ollama é hipótese, sem troca realizada nem
vantagem demonstrada sobre o Qwen.
