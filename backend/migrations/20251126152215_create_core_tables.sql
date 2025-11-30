CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY,
    phone           TEXT NOT NULL UNIQUE,
    name            TEXT,
    date_of_birth   DATE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE conversations (
    id              UUID PRIMARY KEY,
    user_id         BIGINT REFERENCES users(id),
    started_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE messages (
    id              UUID PRIMARY KEY,
    conversation_id UUID NOT NULL REFERENCES conversations(id),
    sender          TEXT NOT NULL, -- 'user' | 'system'
    text            TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE bookings (
    id              UUID PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id),
    business_name   TEXT NOT NULL,
    service_name    TEXT NOT NULL,
    scheduled_at    TIMESTAMPTZ NOT NULL,
    status          TEXT NOT NULL, -- 'pending' | 'confirmed' | 'cancelled'
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
