ALTER TABLE admin_logs
  DROP COLUMN new_data;

ALTER TABLE admin_logs
  DROP COLUMN old_data;

ALTER TABLE admin_logs
  DROP COLUMN base_link;

ALTER TABLE admin_logs
  ADD action_flag INT NOT NULL DEFAULT 1;

UPDATE admin_logs 
    SET action_flag = 0
    WHERE action = 'Create';

UPDATE admin_logs 
    SET action_flag = 1
    WHERE action = 'Update';

UPDATE admin_logs 
    SET action_flag = 2
    WHERE action = 'Delete';

ALTER TABLE admin_logs
  DROP COLUMN action;

ALTER TABLE admin_logs
  ADD change_message VARCHAR NOT NULL DEFAULT '';

DROP TYPE admin_log_action;