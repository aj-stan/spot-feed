-- Enable extensions (only needs to be done once per database)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS postgis;

-- Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    is_guest BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT now()
);

-- Profiles
CREATE TABLE profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    display_name TEXT,
    profile_pic_url TEXT,
    phone TEXT,
    social_links JSONB,
    masked_links JSONB,
    updated_at TIMESTAMP DEFAULT now()
);

-- Joints (Groups)
CREATE TABLE joints (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    creator_id UUID REFERENCES users(id),
    name TEXT NOT NULL,
    type TEXT CHECK (type IN ('public', 'private')),
    is_hidden BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT now(),
    expires_at TIMESTAMP,
    status TEXT CHECK (status IN ('active', 'expired', 'deleted')) DEFAULT 'active',
    location geography(Point, 4326)  -- <-- This enables geospatial search!
);

-- Joint Members
CREATE TABLE joint_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    joint_id UUID REFERENCES joints(id),
    user_id UUID REFERENCES users(id),
    role TEXT CHECK (role IN ('admin', 'moderator', 'member')) DEFAULT 'member',
    joined_at TIMESTAMP DEFAULT now(),
    suspended_until TIMESTAMP,
    left_at TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE
);

-- Messages
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    joint_id UUID REFERENCES joints(id),
    sender_id UUID REFERENCES users(id),
    content TEXT,
    media_url TEXT,
    media_type TEXT,
    created_at TIMESTAMP DEFAULT now(),
    is_deleted BOOLEAN DEFAULT FALSE
);
