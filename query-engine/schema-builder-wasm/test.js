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

console.log(schema.findQueryField('findFirstUser'))
for (const f of schema.query().getFields()) {
  console.log(f.name)
  for (const arg of f.getArguments()) {
    console.log(' ', arg.name, arg.getFieldTypes())
  }
}