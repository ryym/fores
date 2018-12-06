ALTER TABLE file_assocs
DROP CONSTRAINT file_assocs_child_id_fkey,
ADD CONSTRAINT file_assocs_child_id_fkey
  FOREIGN KEY (child_id) REFERENCES files (id);

ALTER TABLE file_owners
DROP CONSTRAINT file_owners_file_id_fkey,
ADD CONSTRAINT file_owners_file_id_fkey
  FOREIGN KEY (file_id) REFERENCES files (id);
