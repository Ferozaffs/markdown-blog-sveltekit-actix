# Use a Node.js LTS image as the base image for the frontend
FROM node:18-alpine as builder

WORKDIR /app

# Copy the package.json and package-lock.json files to the container
COPY package*.json ./

# Install frontend dependencies
RUN npm install

# Copy the rest of the frontend application code to the container
COPY . .

# Build the SvelteKit application
RUN npm run build

FROM node:18-alpine as deploy

# Prepare node deployment
WORKDIR /app
RUN rm -rf ./*
COPY --from=builder /app/package.json .
COPY --from=builder /app/package-lock.json .
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules

CMD ["node", "build"]