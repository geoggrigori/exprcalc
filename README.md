<!-- ══════════════════════════ TÍTULO ══════════════════════════ -->
<div align="center">
  <img src="docs/title-banner.svg" width="100%" alt="exprcalc"/>
</div>

<!-- ══════════════════════ IDIOMAS / LANGUAGES ══════════════════════ -->
<div align="center">
<a href="README.md"><img src="https://img.shields.io/badge/Português-1987F0?style=for-the-badge" alt="Português"/></a>
<a href="README.en.md"><img src="https://img.shields.io/badge/English-555555?style=for-the-badge" alt="English"/></a>
<a href="README.es.md"><img src="https://img.shields.io/badge/Español-555555?style=for-the-badge" alt="Español"/></a>
</div>

<div align="center">
  <img src="assets/banner.svg" width="100%" alt="exprcalc"/>
</div>

<h1 align="center">exprcalc</h1>
<p align="center"><em>Avaliador de expressões aritméticas rápido, escrito em Rust</em></p>
<p align="center"><strong>Lexer → parser recursivo-descendente → avaliador → f64</strong></p>

<div align="center">
<a href="https://github.com/geoggrigori/exprcalc/actions/workflows/ci.yml"><img src="https://github.com/geoggrigori/exprcalc/actions/workflows/ci.yml/badge.svg" alt="CI"/></a>
<img src="https://img.shields.io/badge/Rust-1.70%2B-000000?style=flat-square&logo=rust&logoColor=white" alt="rust"/>
<img src="https://img.shields.io/badge/dependencies-zero-4A1E86?style=flat-square" alt="zero deps"/>
<img src="https://img.shields.io/badge/License-MIT-2E7D32?style=flat-square" alt="license"/>
</div>

<div align="center">
<a href="#sobre"><img src="https://img.shields.io/badge/▸_SOBRE-1987F0?style=for-the-badge" alt="sobre"/></a>
<a href="#como-funciona"><img src="https://img.shields.io/badge/▸_COMO_FUNCIONA-000000?style=for-the-badge" alt="funciona"/></a>
<a href="#gramática"><img src="https://img.shields.io/badge/▸_GRAMÁTICA-1987F0?style=for-the-badge" alt="gramatica"/></a>
<a href="#uso"><img src="https://img.shields.io/badge/▸_USO-000000?style=for-the-badge" alt="uso"/></a>
</div>

<br/>

> 🦀 **Só standard library** — zero dependências externas em todo o crate.

## Sobre

**exprcalc** é um avaliador de expressões aritméticas pequeno e rápido, escrito em Rust. Pega uma string como `2 + 3 * (4 - 1)`, passa por um **lexer** feito à mão, um **parser recursivo-descendente**, e um **avaliador**, e devolve um `f64`.

**Destaques:**
- Números (inteiros e ponto flutuante), operadores `+ - * / % ^`, menos unário e parênteses.
- Precedência correta: `^` liga mais forte que `* / %`, que ligam mais forte que `+ -`.
- Exponenciação e menos unário associativos à direita (`2 ^ 3 ^ 2` = `512`; `--5` = `5`).
- `Error` enum limpo (`LexError`, `ParseError` com posição, `DivisionByZero`).
- Usável como **biblioteca** (`pub fn eval(input: &str) -> Result<f64, Error>`) ou **CLI** (modo direto e REPL).
- Totalmente testado: testes unitários, de integração e de documentação.

## Como Funciona

```mermaid
flowchart LR
    A["string de entrada"] --> B["Lexer"]
    B -->|tokens| C["Parser"]
    C -->|AST| D["Avaliador"]
    D -->|resultado f64| E["saída"]

    B -.->|caractere inválido| X["Error::LexError"]
    C -.->|entrada malformada| Y["Error::ParseError"]
    D -.->|divisão por zero| Z["Error::DivisionByZero"]
```

## Gramática

```ebnf
expr    = term , { ( "+" | "-" ) , term } ;
term    = power , { ( "*" | "/" | "%" ) , power } ;
power   = unary , [ "^" , power ] ;
unary   = "-" , unary | primary ;
primary = number | "(" , expr , ")" ;
```

Precedência (menor pra maior): aditiva (`+ -`) → multiplicativa (`* / %`) → exponenciação (`^`) → menos unário → primária. Como o menos unário liga mais forte que `^`, `-2 ^ 2` = `(-2) ^ 2 = 4`; use `2 ^ (-1)` pra expoente negativo.

## Uso

```sh
cargo build --release      # binário otimizado em target/release/exprcalc
cargo install --path .     # instalar no PATH
```

**Direto:**
```sh
$ exprcalc "2 + 3 * (4 - 1)"
11
$ exprcalc "2 ^ 3 ^ 2"
512
```

**REPL:**
```sh
$ exprcalc
> 2 + 2
4
```

**Como biblioteca:**
```rust
use exprcalc::eval;
match eval("2 + 3 * 4") {
    Ok(value) => println!("= {value}"),
    Err(err) => eprintln!("error: {err}"),
}
```

**Testes:**
```sh
cargo test
```

## Licença

[MIT](LICENSE).

<div align="center">
  <img src="https://file.loading.io/color/feature/thumb/Blues-8.png?" width="100%" height="10px" alt="divider"/>
</div>

<p align="center"><sub>Desenvolvido por <strong><a href="https://github.com/geoggrigori">Grigori</a></strong> · 2026</sub></p>
