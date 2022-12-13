const {  buildSchema } = require('./pkg')

const schema = buildSchema(`
generator client {
    provider        = "prisma-client-js"
  }
  
  datasource db {
    provider = "sqlite"
    url      = "file:./dev.db"
  }
  
  model User {
    id        String   @id @default(uuid())
  }
`)

console.log(schema.query().getFields().map(f => f.name))