# cpf_cnpj

Validador e Gerador de CPF e CNPJ para Rust.

[![Crates.io](https://img.shields.io/crates/v/cpf_cnpj)](https://crates.io/crates/cpf_cnpj) &bull; [![Crates.io](https://img.shields.io/crates/l/cpf_cnpj)](https://github.com/andrelmlins/cpf_cnpj/blob/master/LICENSE) &bull; [![Build Status](https://travis-ci.com/andrelmlins/cpf_cnpj.svg?branch=master)](https://travis-ci.com/andrelmlins/cpf_cnpj) &bull; [![API](https://docs.rs/cpf_cnpj/badge.svg)](https://docs.rs/cpf_cnpj)

## Características

- Analisa repetições de dígitos como `111.111.111-11` ou `000.000.000-00`
- Ignora caracteres especiais de formatação (`.`, `-`, `/`)
- Valida antecipadamente a quantidade de dígitos
- **Suporte ao CNPJ Alfanumérico** (novo formato a partir de julho/2026 - IN RFB nº 2.229/2024)

## Instalação

Adicione essa dependência no seu `Cargo.toml`:

```toml
[dependencies]
cpf_cnpj = "0.3"
```

## Uso básico

```rust
use cpf_cnpj::cpf;
use cpf_cnpj::cnpj;

// Validação de CPF
cpf::validate("255.248.930-33"); // true
cpf::validate("25524893033");    // true
cpf::validate("99999999999");    // false

// Validação de CNPJ (formato tradicional)
cnpj::validate("36.002.518/0001-01"); // true
cnpj::validate("36002518000101");     // true

// Validação de CNPJ Alfanumérico (novo formato 2026)
cnpj::validate("12.ABC.345/01DE-35"); // true
cnpj::validate("12abc34501de35");     // true (case-insensitive)

// Geração de CPF
cpf::generate(); // "25524893033"

// Geração de CNPJ (formato tradicional)
cnpj::generate(); // "76071265000142"

// Geração de CNPJ Alfanumérico
cnpj::generate_alphanumeric(); // "A1B2C3D40001XX"
```

## CNPJ Alfanumérico

A partir de **julho de 2026**, a Receita Federal do Brasil implementará o CNPJ Alfanumérico conforme a [Instrução Normativa RFB nº 2.229/2024](https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/documentos-tecnicos/cnpj/manual-dv-cnpj.pdf).

### Características do novo formato

- **12 primeiras posições**: podem conter letras (A-Z) ou dígitos (0-9)
- **2 últimas posições (DV)**: sempre numéricas (0-9)
- **Case-insensitive**: letras maiúsculas e minúsculas são aceitas na validação
- **Compatibilidade**: CNPJs numéricos existentes continuam válidos

### Algoritmo de cálculo do DV

O cálculo dos dígitos verificadores segue o módulo 11, com a seguinte conversão de caracteres:

| Caractere | Valor |
|-----------|-------|
| 0-9       | 0-9   |
| A-Z       | 17-42 (código ASCII - 48) |

### Exemplo

```
CNPJ: 12.ABC.345/01DE-??

Posições: 1  2  A   B   C   3  4  5  0  1  D   E
Valores:  1  2  17  18  19  3  4  5  0  1  20  21
Pesos:    5  4  3   2   9   8  7  6  5  4  3   2
Produtos: 5  8  51  36  171 24 28 30 0  4  60  42

Soma: 459
Resto: 459 % 11 = 8
DV1: 11 - 8 = 3

CNPJ completo: 12.ABC.345/01DE-35
```

## Desenvolvimento

### Comandos disponíveis (Makefile)

```bash
make test        # Roda os testes
make lint        # Roda o clippy (lint)
make lint-strict # Clippy com checks rigorosos
make fix         # Aplica correções automáticas do clippy
make fmt         # Formata o código
make fmt-check   # Verifica formatação sem modificar
make check       # Roda fmt-check + lint-strict + test (CI)
make clean       # Limpa artefatos de build
```

### Executando testes

```bash
make test
```

## Licença

**cpf_cnpj** é um software Open Source [licenciado pelo MIT](https://github.com/andrelmlins/cpf_cnpj/blob/master/LICENSE).
