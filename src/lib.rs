/*!
Validador de CPF e CNPJ para Rust.

Alguns características importantes devem ser destacadas nessa biblioteca:

- Analisa repetições de dígitos como `111.111.111-11` ou `000.000.000-00`.
- Ignora caracteres especiais.
- Valida antecipadamente a quantidade de dígitos numéricos
- **Suporte ao CNPJ Alfanumérico** (novo formato a partir de julho/2026 - IN RFB nº 2.229/2024)

## Instalação

Adicione essa dependência no seu `Cargo.toml`:

```toml
[dependencies]
cpf_cnpj = "0.3"
```

## Uso básico

Abaixo uma forma simples de como utilizar essa biblioteca:

```rust
extern crate cpf_cnpj;

use cpf_cnpj::cpf;
use cpf_cnpj::cnpj;

cpf::validate("255.248.930-33");
// true

cpf::validate("25524893033");
// true

cpf::validate("99999999999");
// false

cnpj::validate("36.002.518/0001-01");
// true

cnpj::validate("36002518000101");
// true

// CNPJ alfanumérico (novo formato 2026)
cnpj::validate("12.ABC.345/01DE-35");
// true

cpf::generate();
// 25524893033

cnpj::generate();
// 76071265000142

// Gera CNPJ no novo formato alfanumérico
cnpj::generate_alphanumeric();
// A1B2C3D4E5F600
```
*/

pub mod cnpj;
pub mod cpf;
