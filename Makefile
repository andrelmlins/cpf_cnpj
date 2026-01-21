.PHONY: all test lint lint-strict fix fmt check clean

all: fmt lint test

# Roda os testes
test:
	cargo test

# Roda o clippy (lint)
lint:
	cargo clippy

# Roda o clippy com checks mais rigorosos
lint-strict:
	cargo clippy -- -D warnings -D clippy::unwrap_used

# Aplica correções automáticas do clippy
fix:
	cargo clippy --fix --allow-dirty --allow-staged

# Formata o código
fmt:
	cargo fmt

# Verifica formatação sem modificar
fmt-check:
	cargo fmt -- --check

# Roda fmt + lint + test (CI)
check: fmt-check lint-strict test

# Limpa artefatos de build
clean:
	cargo clean
