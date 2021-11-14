
CREATE TYPE admin_log_action as enum('Create', 'Update', 'Delete');

ALTER TABLE admin_logs
  ADD action admin_log_action NOT NULL DEFAULT 'Update';

UPDATE admin_logs 
    SET action = 'Create'
    WHERE action_flag = 0;

UPDATE admin_logs 
    SET action = 'Update'
    WHERE action_flag = 1;

UPDATE admin_logs 
    SET action = 'Delete'
    WHERE action_flag = 2;

ALTER TABLE admin_logs
  DROP COLUMN change_message;

ALTER TABLE admin_logs
  DROP COLUMN action_flag;

ALTER TABLE admin_logs
  ADD new_data VARCHAR;

ALTER TABLE admin_logs
  ADD old_data VARCHAR;

ALTER TABLE admin_logs
  ADD base_link VARCHAR NOT NULL DEFAULT '';