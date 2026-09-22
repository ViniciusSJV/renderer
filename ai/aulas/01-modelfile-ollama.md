# Aula 1 — Entendendo o Modelfile do Ollama

## Objetivo

Preparar o modelo `renderer-analyst` e distinguir configuração de comportamento
verificado. O primeiro exercício usa texto enviado manualmente; a comunicação
pelo cliente Rust será apresentada na Aula 25.

## Contexto e arquitetura

O projeto é um ray tracer em Rust baseado em *The Ray Tracer Challenge*.
O laboratório estuda como transformar código, testes e medições em evidências
que possam ser explicadas sem ultrapassar seus limites.

- **Graph Engine — “Testa sem explicar”:** organiza fontes e relações e executa
  conferências determinísticas. Não decide automaticamente a verdade de uma frase.
- **LLM Engine — “Explica sem interpretar”:** explica o material recebido,
  separando observações de deduções e possibilidades ainda não verificadas.

“Explica sem interpretar” é uma restrição sobre as conclusões apresentadas,
não a afirmação de que o modelo seja incapaz de interpretar texto.
O princípio é **condensar fatos a partir de vapores de nuances**.
Uma otimização só poderá ser chamada de mais rápida com evidência mensurável.

## Pré-requisitos e obtenção do projeto

Use um editor de texto e um terminal. A sequência completa inclui Linux para
as capturas das aulas 13–23 e PowerShell no Windows para a chamada Ollama
registrada na Aula 25. Não há configuração Dev Container versionada que instale
automaticamente todas as ferramentas. WSL não teve instalação validada nas fontes.

Git será usado para obter e inspecionar o repositório. Em qualquer diretório,
verifique sua disponibilidade; este comando não modifica arquivos:

```bash
git --version
```

A saída deve identificar uma versão do Git. Se o comando não existir, instale
Git pelo instalador para Windows ou pelo gerenciador de pacotes da distribuição
Linux e abra outro terminal. O repositório não registra versões de instaladores
nem um procedimento de instalação de Git testado; valide repetindo o comando.

Em uma pasta de projetos, clone para uma pasta nova. O endereço é o remote
público registrado no projeto; o comando baixa arquivos e cria `renderer-local`:

```bash
git clone https://github.com/ViniciusSJV/renderer.git renderer-local
```

Entre na pasta criada; isso muda apenas o diretório do terminal:

```bash
cd renderer-local
```

**Daqui em diante, execute os comandos na raiz do repositório**, salvo indicação
contrária. Confira a presença de `Cargo.toml`, `src`, `tests` e `ai` no editor.
O material descreve a árvore com as 25 aulas e os binários da Aula 25: alterações
locais ainda não publicadas não são obtidas por um clone. Se faltarem arquivos,
a edição disponível no remoto não contém toda essa sequência; não invente os
arquivos ausentes nem considere o roteiro integralmente disponível nessa edição.

## Preparação do Ollama no Windows

Ollama serve o modelo; `renderer-analyst` será uma configuração da base Qwen.
No PowerShell, verifique a instalação, sem gerar texto ou alterar arquivos:

```powershell
ollama --version
```

Se não existir, instale o aplicativo Ollama para Windows e reabra o PowerShell.
**Limitação das fontes:** o repositório registra uso do Ollama instalado, mas não
preserva seu instalador, os passos de instalação nem requisitos de hardware
validados. A instalação do aplicativo precisa ser concluída pelo procedimento
do fornecedor; não há script de instalação verificável neste repositório.
Repita a verificação de versão após instalar. Não há alternativa Linux/WSL de
instalação do Ollama comprovada pelos registros deste laboratório.

Confira o serviço e os modelos locais, sem geração:

```powershell
ollama list
```

Se a conexão falhar, inicie o servidor em uma segunda janela e mantenha-a aberta:

```powershell
ollama serve
```

Volte à primeira janela e repita a listagem. Se a porta estiver ocupada, use o
diagnóstico de API da Aula 25 antes de iniciar outra instância. `localhost`
identifica o ambiente do processo: um terminal remoto não alcança por esse nome
o Ollama do Windows. Para esta prática, mantenha projeto e Ollama no Windows.

Se `qwen2.5-coder:14b` não estiver na lista, obtenha a base. Este comando usa rede
e grava o modelo no armazenamento do Ollama; duração e espaço necessário variam:

```powershell
ollama pull qwen2.5-coder:14b
```

Repita a listagem e confirme o nome. Disponibilidade remota e capacidade do
hardware não foram verificadas nesta revisão; um erro no download ou na carga
impede prosseguir com esse modelo, não autoriza substituí-lo silenciosamente.

## Conceitos: uma receita de execução

Abra [ai/ollama/Modelfile](../ollama/Modelfile) no editor.
O arquivo configura execução; não treina os pesos do modelo.

| Instrução | Papel |
| --- | --- |
| `FROM qwen2.5-coder:14b` | Base usada pelo laboratório. |
| `temperature 0` | Redução da variabilidade, sem garantia de verdade. |
| `seed 42` | Semente declarada, sem prova de repetibilidade entre ambientes. |
| `num_ctx 4096` | Janela de contexto em tokens. |
| `num_predict 1024` | Limite de geração em tokens. |
| `SYSTEM` | Instruções sobre domínio, evidências e limites. |

Tokens são unidades de texto, não necessariamente palavras. Esses valores não
foram demonstrados como ótimos. As categorias pedidas no SYSTEM são:

- **FACT:** afirmação sustentada por evidência identificada, com origem e escopo.
- **INFERENCE:** conclusão derivada, com premissas e limites explícitos.
- **HYPOTHESIS:** possibilidade ainda dependente de teste.

Um rótulo FACT não valida a frase. O Modelfile não fornece acesso aos arquivos.

## Passo a passo: registrar e experimentar

Na raiz do clone Windows, registre a configuração. O comando lê o Modelfile e
cria ou atualiza o nome no Ollama, sem alterar o código do projeto:

```powershell
ollama create renderer-analyst -f ai/ollama/Modelfile
```

O registro histórico terminou com `success`. Confirme o modelo na listagem;
mensagens de download ou reaproveitamento de camadas podem variar. Repita o
registro quando alterar o Modelfile, para aplicar a nova configuração.

Inicie a interface interativa, ainda no PowerShell. Ela gera texto e pode
carregar o modelo na memória; não edita o repositório:

```powershell
ollama run renderer-analyst
```

Envie este texto na interface do modelo, não no terminal de comandos:

```text
Um renderer Rust usa Rayon para processar pixels e adquire um Mutex
compartilhado para gravar a cor de cada pixel. Não há código,
tempos de execução ou resultados de benchmark fornecidos.
Remover esse Mutex tornará o renderer mais rápido?
Organize a análise em FACT, INFERENCE e HYPOTHESIS.
Proponha um teste e uma medição, sem apresentar resultados inventados.
```

Preserve a resposta em um arquivo novo pelo editor. Avalie se ela identifica
sua fonte, reconhece a ausência de medição e evita recomendar uma representação
de cor desconhecida. Como segundo exercício na mesma sessão do modelo, envie:

```text
Revise a resposta anterior.
1. A descrição fornecida equivale a inspecionar o código?
2. Há informação sobre a representação das cores para recomendar AtomicU32?
3. Disputa pelo mesmo Mutex exige escrever no mesmo pixel?
4. Acesso exclusivo a pixels distintos exige operações atômicas?
5. Como comparar versões preservando correção e condições equivalentes?
Separe fatos, inferências e hipóteses e explicite as lacunas.
```

## Validação e limites

A avaliação histórica identificou categorias corretas na apresentação, mas
suposições sobre cores de 32 bits e confusão entre disputa pela trava e escrita
no mesmo pixel. A revisão reconheceu lacunas, mas manteve contradições. Os textos
acima orientam nova prática; não prometem reproduzir literalmente aquela resposta.

Pixels distintos podem disputar o mesmo mutex. Isso não demonstra espera em
uma execução específica nem que a trava seja um gargalo. Trocar a representação
de cores exige investigar precisão e equivalência. Medir uma alternativa exige
preservar cena, resolução, threads e perfil de compilação e repetir execuções.

Não houve teste Rust, benchmark do renderer ou medição de latência do modelo
nesta etapa histórica. A segunda pergunta acrescenta orientação e não constitui
repetição controlada da primeira. Configuração não garante correção.

## Resultado da aula e próxima aula

O modelo configurado pode receber texto manualmente. A
[Aula 2](02-evidencias-atomizadas.md) reduz o problema a duas tarefas e registra
pequenas afirmações com referências, para avaliar uma conclusão por vez.
