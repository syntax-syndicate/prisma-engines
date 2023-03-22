.PHONY: build run

build: node_modules/.prisma/client/index.js

run: build
	/node/node index.js

node_modules/.prisma/client/index.js: index.js prisma/schema.prisma package.json package-lock.json
	npm install
	npx prisma generate
