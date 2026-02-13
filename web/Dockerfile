FROM node:24-slim

WORKDIR /app

RUN apt-get update && apt-get install -y git

# Install dependencies
COPY package.json package-lock.json ./
RUN npm install

# We'll use a volume to mount the web directory for development
EXPOSE 5173

CMD ["npm", "run", "dev"]
