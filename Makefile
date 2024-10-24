.PHONY: all build-frontend build-backend build up down logs clean \
        install run install-all run-all stop

# Colors for pretty output
GREEN := \033[0;32m
RED := \033[0;31m
YELLOW := \033[0;33m
NC := \033[0m # No Color

# Default target
all: up

# Build frontend container
build-frontend:
	@echo "${YELLOW}Building frontend container...${NC}"
	docker compose build frontend

# Build backend container
build-backend:
	@echo "${YELLOW}Building backend container...${NC}"
	docker compose build backend

# Build both containers in parallel
build:
	@echo "${YELLOW}Building all containers...${NC}"
	docker compose build --parallel frontend backend

# Start the containers
up:
	@echo "${GREEN}Starting all containers...${NC}"
	docker compose up -d

# Start frontend only
up-frontend:
	@echo "${GREEN}Starting frontend container...${NC}"
	docker compose up -d frontend

# Start backend only
up-backend:
	@echo "${GREEN}Starting backend container...${NC}"
	docker compose up -d backend

# Stop all containers
down:
	@echo "${RED}Stopping all containers...${NC}"
	docker compose down

# View logs
logs:
	docker compose logs -f

# View frontend logs
logs-frontend:
	docker compose logs -f frontend

# View backend logs
logs-backend:
	docker compose logs -f backend

# Clean all containers and volumes
clean:
	@echo "${RED}Cleaning all containers and volumes...${NC}"
	docker compose down -v
	docker system prune -f

# Install dependencies for frontend
install-frontend:
	@echo "${YELLOW}Installing frontend dependencies...${NC}"
	docker compose exec frontend npm install

# Install dependencies for backend
install-backend:
	@echo "${YELLOW}Installing backend dependencies...${NC}"
	docker compose exec backend cargo build

# Install all dependencies in parallel
install-all:
	@echo "${YELLOW}Installing all dependencies...${NC}"
	@make -j2 install-frontend install-backend

# Run frontend development server
run-frontend:
	@echo "${GREEN}Running frontend development server...${NC}"
	docker compose exec frontend npm run dev

# Run backend development server
run-backend:
	@echo "${GREEN}Running backend development server...${NC}"
	docker compose exec backend cargo run

# Run both servers in parallel
run-all:
	@echo "${GREEN}Running all development servers...${NC}"
	@make -j2 run-frontend run-backend

# Run tests
test-frontend:
	@echo "${YELLOW}Running frontend tests...${NC}"
	docker compose exec frontend npm test

test-backend:
	@echo "${YELLOW}Running backend tests...${NC}"
	docker compose exec backend cargo test

# Run all tests in parallel
test-all:
	@echo "${YELLOW}Running all tests...${NC}"
	@make -j2 test-frontend test-backend

# Status check
status:
	@echo "${YELLOW}Container Status:${NC}"
	docker compose ps

# Show help
help:
	@echo "Available commands:"
	@echo "  ${GREEN}make build${NC}          - Build all containers in parallel"
	@echo "  ${GREEN}make up${NC}             - Start all containers"
	@echo "  ${GREEN}make down${NC}           - Stop all containers"
	@echo "  ${GREEN}make install-all${NC}    - Install all dependencies in parallel"
	@echo "  ${GREEN}make run-all${NC}        - Run both development servers"
	@echo "  ${GREEN}make test-all${NC}       - Run all tests in parallel"
	@echo "  ${GREEN}make logs${NC}           - View all logs"
	@echo "  ${GREEN}make clean${NC}          - Clean all containers and volumes"
	@echo "  ${GREEN}make status${NC}         - Show container status"
	@echo ""
	@echo "Individual commands:"
	@echo "  ${YELLOW}make build-frontend${NC}     - Build frontend container"
	@echo "  ${YELLOW}make build-backend${NC}      - Build backend container"
	@echo "  ${YELLOW}make up-frontend${NC}        - Start frontend container"
	@echo "  ${YELLOW}make up-backend${NC}         - Start backend container"
	@echo "  ${YELLOW}make install-frontend${NC}   - Install frontend dependencies"
	@echo "  ${YELLOW}make install-backend${NC}    - Install backend dependencies"
	@echo "  ${YELLOW}make run-frontend${NC}       - Run frontend development server"
	@echo "  ${YELLOW}make run-backend${NC}        - Run backend development server"
	@echo "  ${YELLOW}make test-frontend${NC}      - Run frontend tests"
	@echo "  ${YELLOW}make test-backend${NC}       - Run backend tests"
	@echo "  ${YELLOW}make logs-frontend${NC}      - View frontend logs"
	@echo "  ${YELLOW}make logs-backend${NC}       - View backend logs"