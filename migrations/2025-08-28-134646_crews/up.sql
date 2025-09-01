CREATE TABLE crews (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  name TEXT NOT NULL,
  specialty TEXT NOT NULL,
  dm_id TEXT NOT NULL,
  FOREIGN KEY (dm_id) REFERENCES users(username)
);

CREATE TABLE crew_members (
  user_id TEXT NOT NULL,
  crew_id INTEGER NOT NULL,
  display_name TEXT NOT NULL,
  PRIMARY KEY (user_id, crew_id),
  FOREIGN KEY (user_id) REFERENCES users(username),
  FOREIGN KEY (crew_id) REFERENCES crews(id)
);
