FROM node:20-slim AS builder

WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy project files
COPY . .

# Build the application
RUN npm run build

# Production stage
FROM node:20-slim AS production

WORKDIR /app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install production dependencies
RUN npm ci --omit=dev

# Copy built assets from the builder stage
COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./

# Expose the port the app runs on
EXPOSE 3000

# Command to run the application with explicit host binding to make it accessible
ENV PORT=3000
ENV HOST=0.0.0.0

# SvelteKit application entry point using the start script
CMD ["npm", "run", "start"]