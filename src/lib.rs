/*!
Validador de CPF e CNPJ para Rust.

Alguns características importantes devem ser destacadas nessa biblioteca:

- Analisa repetições de dígitos como `111.111.111-11` ou `000.000.000-00`.
- Ignora caracteres especiais.
- Valida antecipadamente a quantidade de dígitos numéricos

## Instalação

Adicione essa dependência no seu `Cargo.toml`:

```toml
[dependencies]
cpf_cnpj = "0.2"
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

cpf::generate();
// 25524893033

cnpj::generate();
// 76071265000142
```
*/

pub mod cnpj;
pub mod cpf;
