-- Your SQL goes here
CREATE TABLE clubs (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  body TEXT NOT NULL,
  publish_date timestamp with TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expiry_date timestamp with TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  picture TEXT NOT NULL UNIQUE,
  first_name TEXT NOT NULL UNIQUE,
  last_name TEXT NOT NULL UNIQUE,
  is_admin BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE club_members(
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  club_id INT NOT NULL,
  is_moderator TEXT NOT NULL DEFAULT 'false',
  CONSTRAINT member_user_id_exists FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT member_club_id_exists FOREIGN KEY(club_id) REFERENCES clubs(id) ON DELETE CASCADE
);