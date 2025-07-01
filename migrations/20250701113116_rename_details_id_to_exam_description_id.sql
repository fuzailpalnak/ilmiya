-- Add migration script here
-- Step 1: Drop the existing foreign key constraint
ALTER TABLE sections
DROP CONSTRAINT IF EXISTS sections_details_id_fkey;

-- Step 2: Rename the column
ALTER TABLE sections
RENAME COLUMN details_id TO exam_description_id;

-- Step 3: Add the new foreign key constraint
ALTER TABLE sections
ADD CONSTRAINT sections_exam_description_id_fkey
FOREIGN KEY (exam_description_id) REFERENCES exam_descriptions(id) ON DELETE CASCADE;
