CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  key VARCHAR(20) NOT NULL,
  tree JSONB NOT NULL,

  UNIQUE (key)
);

CREATE TABLE files (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  kind SMALLINT NOT NULL,
  name VARCHAR NOT NULL
);

CREATE TABLE file_assocs (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  dir_id BIGINT NOT NULL REFERENCES files (id),
  child_id BIGINT NOT NULL REFERENCES files (id),
  child_name VARCHAR NOT NULL,

  UNIQUE (dir_id, child_name)
);