import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { Row, Posicao } from '../_modelo/posicao';

@Injectable()
export class XadrezService {

  constructor( private http: HttpClient ) { }

  init() {
    return this.http.get<Row[]>('/api/init');
  }

  getPosicao(posicao: string) {
    return this.http.get<Posicao>('/api/row/' + posicao);
  }

  criarJogada(posicaoSelecionada: Posicao) {
    return this.http.put<Row[]>('/api/rows/jogada', posicaoSelecionada);
  }

  moverPosicao(posicaoSelecionada: Posicao, posicao: string) {
    return this.http.put<Row[]>('/api/rows/mover/' + posicao, posicaoSelecionada);
  }

}
