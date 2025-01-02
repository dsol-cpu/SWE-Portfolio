FROM node:18-alpine

WORKDIR /app

# Copy package files first
COPY front_end/package*.json ./

# Install pnpm and dependencies
RUN npm install -g pnpm \
    pnpm install

# Copy all files and directories, using . instead of *
COPY front_end/. .

# Add .dockerignore if you haven't already to exclude node_modules
EXPOSE 5173

CMD ["pnpm", "run", "dev", "--host"]
