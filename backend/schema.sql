CREATE TABLE IF NOT EXISTS UserLogins (
  id BIGSERIAL PRIMARY KEY,
  tag TEXT NOT NULL UNIQUE, 
  hash TEXT NOT NULL, 
  permissions INT DEFAULT 0
);

CREATE TABLE IF NOT EXISTS UserSessions (
  id BIGSERIAL PRIMARY KEY,
  token TEXT NOT NULL UNIQUE, 
  UserId INT NOT NULL, 
  started TIMESTAMP DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS Runs (
  id SERIAL PRIMARY KEY, 
  UserId INT NOT NULL, 
  trailId INT NOT NULL,
  start_time TIMESTAMP, 
  finish_time TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Trails ( 
  id SERIAL PRIMARY KEY, 
  name text NOT NULL UNIQUE,
  len FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS Devices (
  id SERIAL PRIMARY KEY, 
  trailId INT NOT NULL, 
  token TEXT NOT NULL UNIQUE
);
