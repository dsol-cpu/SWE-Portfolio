SHELL := /bin/bash

build: down
	docker compose build
build-frontend:
	docker compose build frontend
build-backend:
	docker compose build backend
up:
	docker compose up
down:
	docker compose down
clean: down
	docker system prune -f
kube-up:
	kubectl get namespace portfolio || kubectl apply -f k8s/namespace.yaml
	@echo "Checking namespace 'portfolio' status..."
	@sleep 2 # Small delay to allow status update
	@if kubectl get namespace portfolio -o jsonpath='{.status.phase}' | grep -q Active; then \
		echo "Namespace 'portfolio' is Active."; \
	else \
		echo "Waiting for namespace 'portfolio' to become Active..."; \
		kubectl wait --for=condition=Active namespace/portfolio --timeout=60s || { \
			echo "Namespace 'portfolio' did not become Active within timeout."; exit 1; \
		} \
	fi
	kubectl apply -f k8s/

kube-down:
	kubectl delete namespace portfolio
