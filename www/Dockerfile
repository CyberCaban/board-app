# Build stage
FROM node:20-slim AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY .env ./

# Install dependencies
RUN npm i --force

ARG NEXT_PUBLIC_API_BASE_URL
ENV NEXT_PUBLIC_API_BASE_URL=$NEXT_PUBLIC_API_BASE_URL

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Production stage
FROM node:20-slim AS runner

WORKDIR /app

# Install curl for healthcheck
RUN apt-get update && apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/*

ENV NODE_ENV=production

# Create a non-root user
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs 
USER nextjs

# Copy necessary files from builder
COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/api/toro || exit 1

# Start the application
CMD ["node", "server.js"]
