-- Your SQL goes here
CREATE TABLE songs (
	id INTEGER PRIMARY KEY NOT NULL,
	ytid TEXT UNIQUE,
	name TEXT NOT NULL,
	artist INTEGER,
	album INTEGER,
	duration INTEGER NOT NULL, -- Milliseconds
	FOREIGN KEY (artist) REFERENCES artists (id),
	FOREIGN KEY (album) REFERENCES albums (id)
);

CREATE TABLE albums (
	id INTEGER PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	artist INTEGER,
	length INTEGER NOT NULL,
	FOREIGN KEY (artist) REFERENCES artists (id)
);

CREATE TABLE artists (
	id INTEGER PRIMARY KEY NOT NULL,
	name TEXT NOT NULL
);
