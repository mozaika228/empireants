.PHONY: up down logs ps restart validate smoke

up:
	docker compose up -d --build

down:
	docker compose down

logs:
	docker compose logs -f --tail=200

ps:
	docker compose ps

restart:
	docker compose restart

validate:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --all-targets
	python -m py_compile scripts/analyze.py scripts/plot_heatmap.py scripts/experiments.py scripts/analyze_validation.py

smoke:
	cargo run --release --bin scale_benchmark -- 10k 5
	cargo run --release --bin scientific_validation
