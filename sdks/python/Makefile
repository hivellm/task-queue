# Task Queue Python SDK Development Makefile

.PHONY: help install install-dev test test-cov lint format check clean build publish docs

help: ## Show this help message
	@echo "Task Queue Python SDK Development Commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

install: ## Install the package
	pip install -e .

install-dev: ## Install with development dependencies
	pip install -e ".[dev]"

test: ## Run tests
	pytest

test-cov: ## Run tests with coverage
	pytest --cov=taskqueue --cov-report=html --cov-report=term-missing

test-verbose: ## Run tests with verbose output
	pytest -v

lint: ## Run linting
	flake8 taskqueue tests
	mypy taskqueue

format: ## Format code with black and isort
	black taskqueue tests examples
	isort taskqueue tests examples

check: ## Run all checks (format, lint, test)
	black --check taskqueue tests examples
	isort --check-only taskqueue tests examples
	flake8 taskqueue tests
	mypy taskqueue
	pytest --cov=taskqueue --cov-report=term-missing

clean: ## Clean build artifacts
	rm -rf dist/
	rm -rf build/
	rm -rf *.egg-info/
	rm -rf .coverage
	rm -rf htmlcov/
	rm -rf .pytest_cache/
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true

build: ## Build the package
	python -m build

publish-test: ## Publish to TestPyPI
	twine upload --repository testpypi dist/*

publish: ## Publish to PyPI
	twine upload dist/*

docs: ## Generate documentation
	sphinx-build docs docs/_build/html

docs-serve: ## Serve documentation locally
	sphinx-autobuild docs docs/_build/html

example-sync: ## Run sync client example
	python examples/basic_usage.py

example-cli: ## Show CLI help
	taskqueue --help

example-cli-tasks: ## Show CLI tasks help
	taskqueue tasks --help

security: ## Run security checks
	bandit -r taskqueue

deps: ## Update dependencies
	pip-compile --upgrade
	pip-compile --upgrade requirements-dev.in

# Development workflow
dev-setup: clean install-dev ## Setup development environment
	@echo "✅ Development environment ready!"

dev-check: format lint test-cov ## Run full development check
	@echo "✅ All checks passed!"

# CI/CD simulation
ci: check build ## Simulate CI pipeline
	@echo "✅ CI pipeline passed!"
