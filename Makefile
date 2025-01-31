SHELL := /bin/bash
ENV_FILES := --env-file ./src/front_end/.env.dev --env-file ./src/back_end/.env.dev

build: down
	podman-compose $(ENV_FILES) build
build-frontend:
	podman-compose $(ENV_FILES) build frontend
build-backend:
	podman-compose $(ENV_FILES) build backend
up-b: build
	podman-compose $(ENV_FILES) up
up:
	podman-compose $(ENV_FILES) up
down:
	podman-compose $(ENV_FILES) down
clean: down
	podman system prune -f
pod-create:
	podman pod create --name portfolio-pod -p 5173:5173 -p 10000:10000

pod-build: pod-create
	podman build -t frontend -f src/front_end/Dockerfile .
	podman build -t backend -f src/back_end/Dockerfile .

pod-run:
	podman pod exists portfolio-pod || $(MAKE) pod-create
	podman run -d --pod portfolio-pod \
		--env-file ./src/front_end/.env.dev \
		--name frontend --replace frontend
	podman run -d --pod portfolio-pod \
		--env-file ./src/back_end/.env.dev \
		--name backend --replace backend

pod-stop:
	podman pod stop portfolio-pod

pod-down: pod-stop
	podman pod rm portfolio-pod

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
