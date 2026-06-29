# Simulador De Gravidade 2D

Simulação de gravitação N-corpos em Rust utilizando Macroquad.

## O que é? 

Uma simulação física que modeal a atração gravitacional utilizando um modelo newtoniano entre múltiplos objetos.

## Features

- Cálculo de força gravitacional entre objetos.
- Colisão entre objetos (rebote alteravel).
- Colisão com bordas da tela.

## Como rodar

### Pré-requisitos

- Rust 1.70+
- Cargo

### Instalação e execução

```bash
git clone https://github.com/GuilhermeHerrer/simulacao_gravidade.git
cd simulacao_gravidade
cargo run --release
```

## Estrutura do código

- `Objeto` - Struct que representa cada corpo na simulação.
- `desenhar()` - Desenha os corpos
- `calcular_gforce()` - Calcula força gravitacional
- `checar_colisao()` - Detecta colisões e retorna um Option<>
- `colisoes_paredes()` - Colisão com bordas
- `atualizar_pos() e acelerar()` - Integração numérica

## Autor

Guilherme Francisco Herrera

---