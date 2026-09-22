# Render CPU: empréstimos no lugar de cópias do World

## Conceito e revisão do estado anterior

Validação inicial: `cargo test --lib`, 210 testes passaram.
A estrutura existente é um protótipo funcional, com limites ainda abertos:

- `bibliotecario.rs` conhece os adapters por meio de `from_renderer_artifact`
  e `from_chess_artifact`; `core` apenas reexporta os tipos.
- Os dois adapters comprovam conversão de dados, não ainda um fluxo completo
  de reutilização. Os extractors são construtores de fatos fornecidos pelo chamador.
- `EvidenceBundle::validate` não confere referências entre fonte, fatos e seleção,
  nem verifica se as linhas existem na fonte.
- `SceneSpec` valida alguns campos, mas não verifica transformações finitas e
  invertíveis, tipos de objeto suportados ou FOV menor que pi. Ainda não há
  conversão validada do schema para `World`/`Camera` neste módulo.

Esses limites não foram modificados nesta etapa de otimização de CPU.

## Implementação mínima

A inspeção identificou cópias do mundo por pixel e durante o shading.
`color_at`, `shade_hit`, `is_shadowed`, `reflected_color` e `refracted_color`
agora recebem `&self`; render e shading reutilizam o mesmo mundo imutável.
A matemática de interseção, iluminação e recursão permanece a mesma.
A chamada usual desses métodos continua válida; tipos explícitos de ponteiros
para métodos que recebiam `World` por valor precisam adaptar a assinatura.

Antes da baseline comparável, o benchmark foi ajustado para clonar o mundo
antes do cronômetro e liberar o canvas depois da medição. `black_box` consome
os resultados. O trecho medido inclui `Camera::render`, alocação do canvas,
trabalho paralelo e finalização do render; não inclui I/O nem montagem da cena.

## Teste de regressão

Antes da otimização, foi capturado um SHA-256 das cores de todos os pixels
(quantizadas em passos de `EPSILON`) de uma cena 24×16 com câmera transformada,
duas esferas e plano reflexivo/transparente. O teste
`render_preserves_complete_reflective_scene` exige o mesmo resultado após a mudança.
Ele complementa os testes existentes de sombras, reflexão, refração e recursão.
Depois da alteração: `cargo test --lib`, 211 passaram, zero falhas.

## Medição antes/depois

Ambiente: Linux, rustc 1.98.1, perfil `release`, `RAYON_NUM_THREADS=4`.
Mesma cena original de `render_benchmark`: 64×64, duas esferas e uma luz;
câmera na origem, dentro das esferas. Essa cena é limitada e não representa
uma cena externa complexa com muitos objetos ou grupos.

- `original.txt`: execução do benchmark anterior, antes do ajuste de medição.
- `before.txt`: baseline salva antes da otimização, com medição ajustada.
- `comparison.json`: sete execuções por versão, alternando a ordem; cada
  execução tem três warmups e cinco amostras. Nenhum teste foi executado em
  paralelo com a coleta. Resultados brutos preservados (35 amostras por versão).

| Versão | Mediana das 35 amostras |
| --- | ---: |
| Antes | 4.876.415 ns |
| Depois | 4.132.936 ns |

Redução observada: **15,25%** na mediana (aproximadamente 1,18×).
É uma observação local, sujeita à variação do ambiente; não uma garantia geral.
Os números antigos em debug não são comparáveis com estas medições em release.

Para medir a versão atual:

```bash
cargo build --release --bin bench_render
RAYON_NUM_THREADS=4 target/release/bench_render
```

`optimization.patch` registra apenas a alteração de performance, separada do
ajuste do benchmark e do teste. Em uma cópia isolada da árvore atual, aplicar
esse patch ao contrário recupera a implementação anterior com o mesmo harness.

## Próximo passo

Medir cenas externas maiores e cenas com grupos/reflexão antes de generalizar
o ganho. Investigar o mutex por pixel, `par_bridge` e a inversão da câmera por
raio como candidatos; ainda não há perfil de CPU que determine o hotspot dominante.
Antes da geração de cenas por linguagem natural, completar e testar o contrato
SceneSpec → validação → construção, mantendo os engines separados do render.

Validação final: `cargo test` completo passou com 327 testes aprovados,
zero falhas e dois ignorados. A primeira tentativa no sandbox bloqueou dois
testes que abrem servidor HTTP local; a repetição com a permissão existente
para `cargo test` concluiu sem falhas. `git diff --check` também passou.
