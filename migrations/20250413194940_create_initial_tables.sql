-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users Table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

-- Communities Table
CREATE TABLE communities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    owner_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
);

CREATE INDEX idx_communities_name ON communities (name);

-- Community Members Table (Many-to-Many: Users <-> Communities)
CREATE TABLE community_members (
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    community_id UUID NOT NULL REFERENCES communities (id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    PRIMARY KEY (user_id, community_id)
);
