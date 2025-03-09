-- This file should undo anything in `up.sql`
ALTER TABLE song
ADD COLUMN path TEXT NOT NULL DEFAULT '/home/user/.local/share/resonance/songs/YTID';
