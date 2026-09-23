# Renomeação para Librarian — 22/09/2026

Pedido explícito do usuário: usar Librarian e conferir via comando o GitHub.

- Checkout: `Librarian/`; crates `librarian-core` e `librarian-graph-engine`.
- Tipo compartilhado: `LibrarianFact`; alias antigo mantido para compatibilidade.
- Renderer adaptado e Cargo.lock atualizado; sem alteração na matemática de render.
- Commit local `118f161`, após `1654bea`.
- Testes: 21 no Librarian + 315 no renderer, zero falhas, dois ignorados.
- `gh repo view` confirmou `ViniciusSJV/librarian`, público e já existente.
- `gh repo rename Librarian` recebeu HTTP 403, sem permissão administrativa.
- `git push -u origin main` recebeu HTTP 403, sem permissão de escrita.
- Não houve publicação do código. Não é necessário criar outro repositório.

É preciso conceder acesso de escrita à integração para enviar os commits.
A grafia canônica do nome pode ser alterada pelo proprietário nas configurações.
O arquivo `Librarian-118f161.tar.gz` contém o código atual sem histórico nem binários.
O arquivo antigo `bibliotecario-1654bea.tar.gz` permanece como registro anterior.
Os caminhos/números de commit no relatório inicial são históricos; prevalece este registro.

## Preferência posterior do usuário

Manter `ViniciusSJV/librarian` no GitHub, sem alterar a capitalização.
O remoto local usa esse endereço. Não há mais pendência de renomeação remota;
a autorização de escrita continua necessária para publicar o código.
