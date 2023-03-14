const { PrismaClient } = require(".prisma/client");

async function main() {
  const prisma = new PrismaClient();
  console.log(await prisma.a.findMany());
}

main();
