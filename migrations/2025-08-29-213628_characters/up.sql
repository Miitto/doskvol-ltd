CREATE TABLE characters (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  user_id TEXT NOT NULL,
  crew_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  look TEXT NOT NULL DEFAULT '',
  heritage TEXT NOT NULL,
  background TEXT NOT NULL,
  vice TEXT NOT NULL,
  stress INTEGER CHECK(stress >= 0 AND stress <= 9) NOT NULL DEFAULT 0,
  trauma INTEGER NOT NULL DEFAULT 0,
  healing INTEGER CHECK(healing >= 0 AND healing <= 4) NOT NULL DEFAULT 0,
  armor INTEGER NOT NULL DEFAULT 0,
  notes TEXT NOT NULL DEFAULT '',
  class TEXT CHECK(class IN ('Cutter', 'Hound', 'Leech', 'Lurk', 'Slide', 'Spider', 'Whisper')) NOT NULL,
  stash INTEGER CHECK(stash >= 0 AND stash <= 40) NOT NULL DEFAULT 0,
  coin INTEGER CHECK(coin >= 0 AND coin <= 4) NOT NULL DEFAULT 0,
  load INTEGER CHECK(load < 3 AND load >= 0),
  items INTEGER NOT NULL DEFAULT 0,

  FOREIGN KEY (user_id) REFERENCES users(username) ON DELETE CASCADE,
  FOREIGN KEY (crew_id) REFERENCES crews(id) ON DELETE CASCADE
);

CREATE TABLE character_harm (
  character_id INTEGER PRIMARY KEY NOT NULL,
  harm_1_1 TEXT NOT NULL DEFAULT '',
  harm_1_2 TEXT NOT NULL DEFAULT '',
  harm_2_1 TEXT NOT NULL DEFAULT '',
  harm_2_2 TEXT NOT NULL DEFAULT '',
  harm_3 TEXT NOT NULL DEFAULT '',
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_abilities (
  id INTEGER PRIMARY KEY NOT NULL,
  character_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_contacts (
  id INTEGER PRIMARY KEY NOT NULL,
  character_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  friend BOOLEAN NOT NULL,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_class_items (
  id INTEGER PRIMARY KEY NOT NULL,
  character_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_xp (
  character_id INTEGER PRIMARY KEY NOT NULL,
  playbook INTEGER CHECK(playbook >= 0 AND playbook <= 8) NOT NULL DEFAULT 0,
  insight INTEGER CHECK(insight >= 0 AND insight <= 6) NOT NULL DEFAULT 0,
  prowess INTEGER CHECK(prowess >= 0 AND prowess <= 6) NOT NULL DEFAULT 0,
  resolve INTEGER CHECK(resolve >= 0 AND resolve <= 6) NOT NULL DEFAULT 0,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE character_dots (
  character_id INTEGER PRIMARY KEY NOT NULL,
  
  hunt INTEGER CHECK(hunt >= 0 AND hunt <= 4) NOT NULL DEFAULT 0,
  study INTEGER CHECK(study >= 0 AND study <= 4) NOT NULL DEFAULT 0,
  survey INTEGER CHECK(survey >= 0 AND survey <= 4) NOT NULL DEFAULT 0,
  tinker INTEGER CHECK(tinker >= 0 AND tinker <= 4) NOT NULL DEFAULT 0,
  finesse INTEGER CHECK(finesse >= 0 AND finesse <= 4) NOT NULL DEFAULT 0,
  prowl INTEGER CHECK(prowl >= 0 AND prowl <= 4) NOT NULL DEFAULT 0,
  skirmish INTEGER CHECK(skirmish >= 0 AND skirmish <= 4) NOT NULL DEFAULT 0,
  wreck INTEGER CHECK(wreck >= 0 AND wreck <= 4) NOT NULL DEFAULT 0,
  attune INTEGER CHECK(attune >= 0 AND attune <= 4) NOT NULL DEFAULT 0,
  command INTEGER CHECK(command >= 0 AND command <= 4) NOT NULL DEFAULT 0,
  consort INTEGER CHECK(consort >= 0 AND consort <= 4) NOT NULL DEFAULT 0,
  sway INTEGER CHECK(sway >= 0 AND sway <= 4) NOT NULL DEFAULT 0,
  FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);
