FROM node:24-slim

WORKDIR /app

RUN apt-get update && apt-get install -y git

# We'll use a volume to mount the web directory
EXPOSE 5173

CMD ["npm", "run", "dev"]
