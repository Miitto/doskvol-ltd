CREATE TABLE users (
  username TEXT NOT NULL PRIMARY KEY,
  totp_secret TEXT NOT NULL
);
