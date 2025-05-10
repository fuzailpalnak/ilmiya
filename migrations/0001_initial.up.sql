BEGIN;

CREATE TABLE IF NOT EXISTS exams (
    id SERIAL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS exam_descriptions (
    id SERIAL PRIMARY KEY,
    exam_id INTEGER NOT NULL REFERENCES exams(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    duration INTEGER NOT NULL,          -- in minutes?
    passing_score INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sections (
    id SERIAL PRIMARY KEY,
    details_id INTEGER NOT NULL REFERENCES exam_descriptions(id) ON DELETE CASCADE,
    title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS questions (
    id SERIAL PRIMARY KEY,
    section_id INTEGER NOT NULL REFERENCES sections(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    description TEXT,
    marks INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS options (
    id SERIAL PRIMARY KEY,
    question_id INTEGER NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    is_correct BOOLEAN
);

-- TODO Not sure if this is redundant with `is_correct`, but kept as-is
CREATE TABLE IF NOT EXISTS correct_options (
    option_id INTEGER PRIMARY KEY REFERENCES options(id) ON DELETE CASCADE
);

COMMIT;
