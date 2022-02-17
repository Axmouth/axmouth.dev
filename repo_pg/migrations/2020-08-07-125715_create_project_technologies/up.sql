

CREATE TABLE technologies (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE projects_technologies (
  id SERIAL PRIMARY KEY,
  technology_id INTEGER NOT NULL,
  project_id INTEGER NOT NULL,
  CONSTRAINT technology_project_fk
    FOREIGN KEY(technology_id) 
    REFERENCES technologies(id),
  CONSTRAINT project_technology_fk
    FOREIGN KEY(project_id) 
	REFERENCES projects(id)
);

CREATE UNIQUE INDEX idx_technologies_id 
ON technologies(id);

CREATE UNIQUE INDEX idx_technologies_name 
ON technologies(name);

CREATE INDEX idx_projects_technologies_project_id
ON projects_technologies(project_id);

CREATE INDEX idx_projects_technologies_technology_id
ON projects_technologies(technology_id);