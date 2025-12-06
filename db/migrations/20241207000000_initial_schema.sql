-- Create User table
CREATE TABLE IF NOT EXISTS "User" (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    "passwordHash" TEXT NOT NULL,
    "createdAt" TIMESTAMPTZ NOT NULL,
    "updatedAt" TIMESTAMPTZ NOT NULL
);

-- Create Balance table
CREATE TABLE IF NOT EXISTS "Balance" (
    id UUID PRIMARY KEY,
    asset TEXT NOT NULL,
    total DOUBLE PRECISION NOT NULL DEFAULT 0,
    reserved DOUBLE PRECISION NOT NULL DEFAULT 0,
    "userId" UUID NOT NULL REFERENCES "User"(id) ON DELETE CASCADE,
    "updatedAt" TIMESTAMPTZ NOT NULL,
    UNIQUE("userId", asset)
);

-- Create Order table
CREATE TABLE IF NOT EXISTS "Order" (
    id UUID PRIMARY KEY,
    "userId" UUID NOT NULL REFERENCES "User"(id) ON DELETE CASCADE,
    side TEXT NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    qty DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    "createdAt" TIMESTAMPTZ NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_balance_user_id ON "Balance"("userId");
CREATE INDEX IF NOT EXISTS idx_order_user_id ON "Order"("userId");
CREATE INDEX IF NOT EXISTS idx_order_status ON "Order"(status);
CREATE INDEX IF NOT EXISTS idx_order_created_at ON "Order"("createdAt");
