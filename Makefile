run:
	cargo run -- $(ARGS)

test:
	cargo test

format:
	cargo fmt

lint:
	cargo clippy -- -D warnings

prepare: 
	format lint

help:
	@echo "Команди:"
	@echo "  make run ARGS='...' - Запустити програму з аргументами ARGS"
	@echo "  make test          - Запустити тести"
	@echo "  make format        - Форматувати код"
	@echo "  make lint          - Перевірити код на відповідність стандартам"
	@echo "  make prepare       - Виконати форматування та перевірку коду"
