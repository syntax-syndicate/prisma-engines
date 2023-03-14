.PHONY: run

run: node_modules/.prisma/client/index.js
	node index.js

node_modules/.prisma/client/index.js: index.js prisma/schema.prisma package.json package-lock.json
	npm install
	npx prisma generate
