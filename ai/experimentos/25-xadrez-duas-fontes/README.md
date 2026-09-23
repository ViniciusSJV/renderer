# Consulta com duas fontes reais

O acervo `ai/acervo/xadrez-v2` combina o serviço e o interceptador na mesma
revisão Git, sem modificar a edição 1. Cadastro e pares de fatos são manuais.

A CLI do renderer, consumindo Librarian `a009af8`, exportou 11 fatos em duas
fontes. Foram conferidos ordem da seleção, fonte/trecho de cada referência,
lacunas preservadas, contextos contra as linhas da respectiva fonte e hashes
dos três arquivos identificados em origin.json. Resultado: aprovado.

`export.txt`, `validation.json` e `bundle/` preservam essa execução. A suíte
de testes anterior não foi repetida. Não executamos Angular, chamadas HTTP,
partidas, benchmarks ou LLM. A conferência não valida semanticamente os pares
de rotas nem comprova que o interceptor está registrado na aplicação.
