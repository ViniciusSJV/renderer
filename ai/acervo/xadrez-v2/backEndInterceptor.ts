import { Injectable } from '@angular/core';
import { HttpRequest, HttpResponse, HttpHandler, HttpEvent, HttpInterceptor, HTTP_INTERCEPTORS } from '@angular/common/http';
import { Observable } from 'rxjs/Observable';
import 'rxjs/add/observable/of';
import 'rxjs/add/observable/throw';
import 'rxjs/add/operator/mergeMap';
import 'rxjs/add/operator/materialize';
import 'rxjs/add/operator/dematerialize';

import { Row, Posicao, PecaType } from '../_modelo/posicao';

@Injectable()
export class BackendInterceptor implements HttpInterceptor {

    constructor() { }

    getRow(posicaoId: string, rows: Row[]) {
      for (let y = 0; y < rows.length; y++) {
        const row = rows[y];

        if ( row.id === posicaoId.charAt(0) ) {
          return row;
        }

      }
    }

    getPosicao(row: Row, posicaoId: string) {
      let posicoesFiltradas: Posicao[] = [];

      posicoesFiltradas = row.posicoes.filter(posicao => {
          return posicao.posicao === posicaoId;
      });

      return posicoesFiltradas[0];
    }

    limpaJogada(rows: Row[]) {
      for ( let y = 0; y < rows.length; y++ ) {
        for ( let x = 0; x < rows[y].posicoes.length; x++ ) {
          const newClass = rows[y].posicoes[x].class.replace(' select', '');
          rows[y].posicoes[x].class = newClass;
        }
      }
      return rows;
    }

    criarJogada(posicaoSelecionada: Posicao, rows: Row[]) {

      const posicoesPossiveis: any[] = this.getPosicoesPossiveis(posicaoSelecionada, rows);

      for (let count = 0; count < posicoesPossiveis.length; count++) {
        const posicaoPosivel = posicoesPossiveis[count];
        rows[posicaoPosivel.y].posicoes[posicaoPosivel.x].class = rows[posicaoPosivel.y].posicoes[posicaoPosivel.x].class.replace(
          rows[posicaoPosivel.y].posicoes[posicaoPosivel.x].class,
          rows[posicaoPosivel.y].posicoes[posicaoPosivel.x].class + ' select');
      }

      return rows;
    }

    getPosicoesPossiveis(posicaoSelecionada: Posicao, rows: Row[]) {

      let posicoesPossiveis: any[] = [];

      const posicao = posicaoSelecionada.posicao;
      const indexRow = rows.findIndex( row => row.id === posicao.charAt(0) );

      const x = parseInt(posicao.charAt(1), 10) - 1;
      const y = indexRow;

      // init regra peao
      if ( posicaoSelecionada.type === PecaType.Peao ) {

        posicoesPossiveis.push({
          y: posicaoSelecionada.class === 'white' ? y + 1 : y - 1,
          x: x
        });
        // regra do primeiro movimento do peao, pode avancar duas casas
        if (posicaoSelecionada.primeiroMovimento) {
          posicoesPossiveis.push({
            y: posicaoSelecionada.class === 'white' ? y + 2 : y - 2,
            x: x
          });
        }
        posicoesPossiveis = this.getPosicoesPecasPeaoPodeComer(y, x, rows, posicoesPossiveis, posicaoSelecionada);
      }
      // fim regra peao

      // init regra cavalo
      if ( posicaoSelecionada.type === PecaType.Cavalo ) {

        posicoesPossiveis.push({
          y: y + 2,
          x: x + 1
        });
        posicoesPossiveis.push({
          y: y + 2,
          x: x - 1
        });
        posicoesPossiveis.push({
          y: y - 2,
          x: x + 1
        });
        posicoesPossiveis.push({
          y: y - 2,
          x: x - 1
        });

        posicoesPossiveis.push({
          y: y + 1,
          x: x + 2
        });
        posicoesPossiveis.push({
          y: y + 1,
          x: x - 2
        });
        posicoesPossiveis.push({
          y: y - 1,
          x: x + 2
        });
        posicoesPossiveis.push({
          y: y - 1,
          x: x - 2
        });

      }
      // fim regra cavalo

      // init regra rei
      if ( posicaoSelecionada.type === PecaType.Rei ) {
        const posicaoRei = posicaoSelecionada.posicao;
        const indexRowRei = rows.findIndex( row => row.id === posicaoRei.charAt(0) );

        posicoesPossiveis.push({
          y: y + 1,
          x: x
        });
        posicoesPossiveis.push({
          y: y + 1,
          x: x + 1
        });
        posicoesPossiveis.push({
          y: y + 1,
          x: x - 1
        });

        posicoesPossiveis.push({
          y: y,
          x: x - 1
        });
        posicoesPossiveis.push({
          y: y,
          x: x + 1
        });

        posicoesPossiveis.push({
          y: y - 1,
          x: x
        });
        posicoesPossiveis.push({
          y: y - 1,
          x: x + 1
        });
        posicoesPossiveis.push({
          y: y - 1,
          x: x - 1
        });
      }
      // fim  regra rei

      // init regra bispo e rainha
      if ( posicaoSelecionada.type === PecaType.Bispo || posicaoSelecionada.type === PecaType.Rainha ) {

        for (let count = 1; count <= 8; count ++) {
          posicoesPossiveis.push({
            y: y + count,
            x: x + count
          });
          posicoesPossiveis.push({
            y: y + count,
            x: x - count
          });
          posicoesPossiveis.push({
            y: y - count,
            x: x + count
          });
          posicoesPossiveis.push({
            y: y - count,
            x: x - count
          });
        }
      }
      // fim regra bispo e rainha

      // init regra torre e rainha
      if ( posicaoSelecionada.type === PecaType.Torre || posicaoSelecionada.type === PecaType.Rainha ) {

        for (let count = 1; count <= 8; count ++) {
          posicoesPossiveis.push({
            y: y + count,
            x: x
          });
          posicoesPossiveis.push({
            y: y - count,
            x: x
          });
          posicoesPossiveis.push({
            y: y,
            x: x + count
          });
          posicoesPossiveis.push({
            y: y,
            x: x - count
          });
        }
      }
      // fim regra torre e rainha

      posicoesPossiveis = this.removePosicoesForaDoTabuleiro(posicoesPossiveis);

      // cavalo é a unica peca q pode pular as outras
      if ( posicaoSelecionada.type !== PecaType.Cavalo ) {
        posicoesPossiveis = this.removePosicaoBloqueada(posicoesPossiveis, rows, posicaoSelecionada);
      }

      posicoesPossiveis = this.removePosicaoOcupadaPecaMesmaCor(posicoesPossiveis, rows, posicaoSelecionada.class);

      // posicao da peca selecionada é sempre uma possicao posicaoPosivel
      // caso o movimento for pra mesma posicao da peca selecionada, o famoso 'passar a vez'
      posicoesPossiveis.push({
        y: y,
        x: x
      });

      return posicoesPossiveis;
    }

    removePosicoesForaDoTabuleiro(posicoesPossiveis: any[]) {
      return posicoesPossiveis.filter(posicao => (posicao.x <= 7 && posicao.x >= 0)  && (posicao.y <= 7 && posicao.y >= 0 ));
    }

    removePosicaoOcupadaPecaMesmaCor(posicoesPossiveis: any[], rows: Row[], cor: string) {
      return posicoesPossiveis.filter(posicao => rows[posicao.y].posicoes[posicao.x].class !== cor);
    }

    removePosicaoBloqueada(posicoesPossiveis: any[], rows: Row[], posicaoSelecionada: Posicao) {
      const x = parseInt(posicaoSelecionada.posicao.charAt(1), 10) - 1;
      const y = rows.findIndex( row => row.id === posicaoSelecionada.posicao.charAt(0) );

      if ( posicaoSelecionada.type === PecaType.Peao ) {

        posicoesPossiveis = posicoesPossiveis.filter(posicao => {

          if ( posicao.x === x ) {
            if ( rows[posicao.y].posicoes[posicao.x].type === PecaType.Vazio ) {
              return posicao;
            }
          } else {
            return posicao;
          }

        });
      }

      if ( posicaoSelecionada.type === PecaType.Torre || posicaoSelecionada.type === PecaType.Rainha ) {
        let posicaoComPecaSima;
        let posicaoComPecaBaixo;
        let posicaoComPecaEsq;
        let posicaoComPecaDir;

        for ( let count = 0; count < posicoesPossiveis.length; count ++ ) {
          const posicaoPossivel = posicoesPossiveis[count];
          const posicao = rows[posicaoPossivel.y].posicoes[posicaoPossivel.x];

          if ( posicao.type !== PecaType.Vazio ) {
            const x1 = parseInt(posicao.posicao.charAt(1), 10) - 1;
            const y1 = rows.findIndex( row => row.id === posicao.posicao.charAt(0) );

            if ( y === y1 ) {
              if ( x1 > x ) {
                if ( posicaoComPecaDir ) {
                  posicaoComPecaDir = posicaoComPecaDir.x > x1 ? {x: x1, y: y1} : posicaoComPecaDir;
                } else {
                    posicaoComPecaDir = {x: x1, y: y1};
                }
              } else if ( x1 < x ) {
               if ( posicaoComPecaEsq ) {
                 posicaoComPecaEsq = posicaoComPecaEsq.x < x1 ? {x: x1, y: y1} : posicaoComPecaEsq;
               } else {
                   posicaoComPecaEsq = {x: x1, y: y1};
               }
             }
           }

           if ( x === x1 ) {
             if ( y1 > y ) {
              if ( posicaoComPecaBaixo ) {
                posicaoComPecaBaixo = posicaoComPecaBaixo.y > y1 ? {x: x1, y: y1} : posicaoComPecaBaixo;
              } else {
                posicaoComPecaBaixo =  {x: x1, y: y1};
              }
            } else if ( y1 < y) {
              if ( posicaoComPecaSima ) {
                posicaoComPecaSima = posicaoComPecaSima.y < y1 ? {x: x1, y: y1} : posicaoComPecaSima;
              } else {
                  posicaoComPecaSima =  {x: x1, y: y1};
              }
            }
           }
          }
        }

        if ( posicaoComPecaEsq ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoComPecaEsq.y === posicaoPossivel.y && posicaoComPecaEsq.x > posicaoPossivel.x) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }
        if ( posicaoComPecaDir ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoComPecaDir.y === posicaoPossivel.y && posicaoComPecaDir.x < posicaoPossivel.x) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }
        if ( posicaoComPecaSima ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoComPecaSima.x === posicaoPossivel.x && posicaoComPecaSima.y > posicaoPossivel.y) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }
        if ( posicaoComPecaBaixo ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoComPecaBaixo.x === posicaoPossivel.x && posicaoComPecaBaixo.y < posicaoPossivel.y) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }
      }

      if ( posicaoSelecionada.type === PecaType.Bispo || posicaoSelecionada.type === PecaType.Rainha ) {
        let posicaoComPecaSimaEsq;
        let posicaoComPecaBaixoEsq;
        let posicaoComPecaSimaDir;
        let posicaoComPecaBaixoDir;

        for ( let count = 0; count < posicoesPossiveis.length; count ++ ) {
          const posicaoPossivel = posicoesPossiveis[count];
          const posicao = rows[posicaoPossivel.y].posicoes[posicaoPossivel.x];

          if ( posicao.type !== PecaType.Vazio ) {
            const x1 = parseInt(posicao.posicao.charAt(1), 10) - 1;
            const y1 = rows.findIndex( row => row.id === posicao.posicao.charAt(0) );

            if (x1 < x) {
              if (y1 > y) {
                if ( posicaoComPecaBaixoEsq ) {
                  posicaoComPecaBaixoEsq = posicaoComPecaBaixoEsq.y > y ? posicaoComPecaBaixoEsq : {x: x1, y: y1};
                } else {
                  posicaoComPecaBaixoEsq = {x: x1, y: y1};
                }
              } else if (y1 < y) {
                if ( posicaoComPecaSimaEsq ) {
                  posicaoComPecaSimaEsq = posicaoComPecaSimaEsq.y < y ? posicaoComPecaSimaEsq : {x: x1, y: y1};
                } else {
                  posicaoComPecaSimaEsq = {x: x1, y: y1};
                }
              }
            }

            if (x1 > x) {
              if (y1 < y) {
                if ( posicaoComPecaSimaDir ) {
                  posicaoComPecaSimaDir = posicaoComPecaSimaDir.y < y ? posicaoComPecaSimaDir : {x: x1, y: y1};
                } else {
                  posicaoComPecaSimaDir = {x: x1, y: y1};
                }
              } else if (y1 > y) {
                if ( posicaoComPecaBaixoDir ) {
                  posicaoComPecaBaixoDir = posicaoComPecaBaixoDir.y > y ? posicaoComPecaBaixoDir : {x: x1, y: y1};
                } else {
                  posicaoComPecaBaixoDir = {x: x1, y: y1};
                }
              }
            }
          }
        }

        if ( posicaoComPecaSimaEsq ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoPossivel.y < posicaoComPecaSimaEsq.y && posicaoPossivel.x < posicaoComPecaSimaEsq.x) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }

        if ( posicaoComPecaBaixoEsq ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoPossivel.y > posicaoComPecaBaixoEsq.y && posicaoPossivel.x < posicaoComPecaBaixoEsq.x) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }

        if ( posicaoComPecaSimaDir ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoPossivel.y < posicaoComPecaSimaDir.y && posicaoPossivel.x > posicaoComPecaSimaDir.x) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }

        if ( posicaoComPecaBaixoDir ) {
          for (let count = 0; count < posicoesPossiveis.length; count++) {
            const posicaoPossivel = posicoesPossiveis[count];
            if (posicaoPossivel.y > posicaoComPecaBaixoDir.y && posicaoPossivel.x > posicaoComPecaBaixoDir.x) {
              posicoesPossiveis.splice(count, 1);
              count--;
            }
          }
        }
      }

      return posicoesPossiveis;

    }

    getPosicoesPecasPeaoPodeComer(y: number, x: number, rows: Row[], posicoesPossiveis: any[], posicaoSelecionada: Posicao) {
      // posiveis posicoes que o peao selecionado pode comer uma peca
      const ySoma = y + 1;
      const ySub = y - 1;
      const xSub = x - 1;
      const xSoma = x + 1;

      const pecaBranca = posicaoSelecionada.class === 'white';

      if (pecaBranca) {
        // pula posicoes fora do tabuleiro
        if ((ySoma <= 7 && ySoma >= 0)  && (xSub <= 7 && xSub >= 0 )) {
          if (rows[ySoma].posicoes[xSub].type !== PecaType.Vazio &&
              rows[ySoma].posicoes[xSub].class !== posicaoSelecionada.class ) {

            posicoesPossiveis.push({
              y: ySoma,
              x: xSub
            });
          }
        }
        if ((ySoma <= 7 && ySoma >= 0)  && (xSoma <= 7 && xSoma >= 0 )) {
          if (rows[ySoma].posicoes[xSoma].type !== PecaType.Vazio &&
              rows[ySoma].posicoes[xSoma].class !== posicaoSelecionada.class ) {

            posicoesPossiveis.push({
              y: ySoma,
              x: xSoma
            });
          }
        }
      } else {
        if ((ySub <= 7 && ySub >= 0)  && (xSub <= 7 && xSub >= 0 )) {
          if (rows[ySub].posicoes[xSub].type !== PecaType.Vazio &&
              rows[ySub].posicoes[xSub].class !== posicaoSelecionada.class ) {

            posicoesPossiveis.push({
              y: ySub,
              x: xSub
            });
          }
        }
        if ((ySub <= 7 && ySub >= 0) && (xSoma <= 7 && xSoma >= 0)) {
          if (rows[ySub].posicoes[xSoma].type !== PecaType.Vazio &&
              rows[ySub].posicoes[xSoma].class !== posicaoSelecionada.class ) {

            posicoesPossiveis.push({
              y: ySub,
              x: xSoma
            });
          }
        }
      }

      return posicoesPossiveis;
    }

    atualizaPosicoes (rowFiltrada: Row, posicaoFiltrada: Posicao, rowSelecionada: Row, posicaoSelecionada: Posicao, rows: Row[]) {
      posicaoFiltrada.peca = posicaoSelecionada.peca;
      posicaoFiltrada.type = posicaoSelecionada.type;
      posicaoFiltrada.class = posicaoSelecionada.class;
      posicaoFiltrada.primeiroMovimento = false;

      posicaoSelecionada.type = PecaType.Vazio;
      posicaoSelecionada.peca = '';
      posicaoSelecionada.class = '';

      const x1 = rowSelecionada.posicoes.findIndex( posicaoS => posicaoS.posicao === posicaoSelecionada.posicao );
      rowSelecionada.posicoes[x1] = posicaoSelecionada;

      const x2 = rowFiltrada.posicoes.findIndex( posicaoF => posicaoF.posicao === posicaoFiltrada.posicao );
      rowFiltrada.posicoes[x2] = posicaoFiltrada;

      const y1 = rows.findIndex( row => row.id === rowSelecionada.id );
      rows[y1] = rowSelecionada;

      const y2 = rows.findIndex( row => row.id === rowFiltrada.id );
      rows[y2] = rowFiltrada;

      return rows;
    }

    intercept(request: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {

        let rows: Row[] = JSON.parse(localStorage.getItem('rows')) || [];
        let jogadorAtual = JSON.parse(localStorage.getItem('jogadorAtual')) || 'white';

        return Observable.of(null).mergeMap(() => {

            if (request.url.endsWith('/api/init') && request.method === 'GET') {

              const posicoesA: Posicao[] = [
                  // brancas
                  {type: PecaType.Torre, posicao: 'a1', peca: '&#9820;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Cavalo, posicao: 'a2', peca: '&#9822;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Bispo, posicao: 'a3', peca: '&#9821;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Rei, posicao: 'a4', peca: '&#9819;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Rainha, posicao: 'a5', peca: '&#9818;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Bispo, posicao: 'a6', peca: '&#9821;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Cavalo, posicao: 'a7', peca: '&#9822;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Torre, posicao: 'a8', peca: '&#9820;', class: 'white', primeiroMovimento: true}
              ];

              const posicoesB: Posicao[] = [
                  {type: PecaType.Peao, posicao: 'b1', peca: '&#9823;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'b2', peca: '&#9823;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'b3', peca: '&#9823;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'b4', peca: '&#9823;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'b5', peca: '&#9823;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'b6', peca: '&#9823;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'b7', peca: '&#9823;', class: 'white', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'b8', peca: '&#9823;', class: 'white', primeiroMovimento: true}
                ];

                const posicoesC: Posicao[] = [
                  // posicoes livres
                  {type: PecaType.Vazio, posicao: 'c1', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'c2', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'c3', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'c4', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'c5', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'c6', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'c7', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'c8', peca: '', class: '', primeiroMovimento: true}
                ];

                const posicoesD: Posicao[] = [
                  {type: PecaType.Vazio, posicao: 'd1', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'd2', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'd3', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'd4', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'd5', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'd6', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'd7', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'd8', peca: '', class: '', primeiroMovimento: true}
                ];

                const posicoesE: Posicao[] = [
                  {type: PecaType.Vazio, posicao: 'e1', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'e2', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'e3', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'e4', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'e5', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'e6', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'e7', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'e8', peca: '', class: '', primeiroMovimento: true}
                ];

                const posicoesF: Posicao[] = [
                  {type: PecaType.Vazio, posicao: 'f1', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'f2', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'f3', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'f4', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'f5', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'f6', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'f7', peca: '', class: '', primeiroMovimento: true},
                  {type: PecaType.Vazio, posicao: 'f8', peca: '', class: '', primeiroMovimento: true}
                ];

                const posicoesG: Posicao[] = [
                  {type: PecaType.Peao, posicao: 'g1', peca: '&#9823;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'g2', peca: '&#9823;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'g3', peca: '&#9823;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'g4', peca: '&#9823;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'g5', peca: '&#9823;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'g6', peca: '&#9823;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'g7', peca: '&#9823;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Peao, posicao: 'g8', peca: '&#9823;', class: 'black', primeiroMovimento: true}
                ];

                const posicoesH: Posicao[] = [
                  // pretas
                  {type: PecaType.Torre, posicao: 'h1', peca: '&#9820;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Cavalo, posicao: 'h2', peca: '&#9822;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Bispo, posicao: 'h3', peca: '&#9821;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Rei, posicao: 'h4', peca: '&#9819;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Rainha, posicao: 'h5', peca: '&#9818;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Bispo, posicao: 'h6', peca: '&#9821;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Cavalo, posicao: 'h7', peca: '&#9822;', class: 'black', primeiroMovimento: true},
                  {type: PecaType.Torre, posicao: 'h8', peca: '&#9820;', class: 'black', primeiroMovimento: true}
                ];

                rows = [
                  {id: 'a', posicoes: posicoesA},
                  {id: 'b', posicoes: posicoesB},
                  {id: 'c', posicoes: posicoesC},
                  {id: 'd', posicoes: posicoesD},
                  {id: 'e', posicoes: posicoesE},
                  {id: 'f', posicoes: posicoesF},
                  {id: 'g', posicoes: posicoesG},
                  {id: 'h', posicoes: posicoesH}
                ];

                localStorage.setItem('rows', JSON.stringify(rows));

                return Observable.of(new HttpResponse({ status: 200, body: rows }));
            }

            if (request.url.match(/\/api\/row\/\w+$/) && request.method === 'GET') {

              const urlSplit = request.url.split('/');
              const posicaoId = urlSplit[urlSplit.length - 1].toString();

              const rowFiltrada: Row = this.getRow(posicaoId, rows);
              const posicaoFiltrada: Posicao = this.getPosicao(rowFiltrada, posicaoId);

              if (posicaoFiltrada.type === PecaType.Vazio) {
                return Observable.throw('Selecione uma peça.');
              }

              if (jogadorAtual !== posicaoFiltrada.class) {
                const jogador = jogadorAtual === 'white' ? 'brancas' : 'pretas';
                return Observable.throw('É a vez das peças ' + jogador + ' jogar.');
              }

              return Observable.of(new HttpResponse({ status: 200, body: posicaoFiltrada }));
            }

            if (request.url.endsWith('/api/rows/jogada') && request.method === 'PUT') {

              const posicaoSelecionada = request.body;

              rows = this.criarJogada(posicaoSelecionada, rows);

              localStorage.setItem('rows', JSON.stringify(rows));

              return Observable.of(new HttpResponse({ status: 200, body: rows }));
            }

            if (request.url.match(/\/api\/rows\/mover\/\w+$/) && request.method === 'PUT') {

              const urlSplit = request.url.split('/');
              const posicaoId = urlSplit[urlSplit.length - 1].toString();

              const rowFiltrada: Row = this.getRow(posicaoId, rows);
              const posicaoFiltrada: Posicao = this.getPosicao(rowFiltrada, posicaoId);

              const pecaComidaEORei = posicaoFiltrada.type ===  PecaType.Rei;

              const posicaoSelecionada = request.body;
              const rowSelecionada: Row = this.getRow(posicaoSelecionada.posicao, rows);

              const posicaoClassValida = 'select';
              if (posicaoFiltrada.class.toUpperCase().indexOf(posicaoClassValida.toUpperCase()) < 0) {
                return Observable.throw('Jogada invalida.');
              }

              rows = this.atualizaPosicoes(rowFiltrada, posicaoFiltrada, rowSelecionada, posicaoSelecionada, rows);

              rows = this.limpaJogada(rows);

              jogadorAtual = jogadorAtual === 'white' ? 'black' : 'white';

              localStorage.setItem('jogadorAtual', JSON.stringify(jogadorAtual));
              localStorage.setItem('rows', JSON.stringify(rows));

              if ( pecaComidaEORei ) {
                return Observable.throw('Xeque mate. Atualize a pagina para jogar novamente.');
              }

              return Observable.of(new HttpResponse({ status: 200, body: rows }));
            }

            return next.handle(request);

        })

        .materialize()
        .dematerialize();
    }
}

export let BackendProvider = {
    provide: HTTP_INTERCEPTORS,
    useClass: BackendInterceptor,
    multi: true
};
