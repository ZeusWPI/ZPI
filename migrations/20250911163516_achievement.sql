CREATE TABLE service (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE achievement (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    service_id INTEGER NOT NULL,
    FOREIGN KEY (service_id) REFERENCES service (id)
);

CREATE TABLE goal (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    achievement_id INTEGER NOT NULL,
    sequence INTEGER NOT NULL,
    FOREIGN KEY (achievement_id) REFERENCES achievement (id)
);

CREATE TABLE unlock (
    user_id INTEGER NOT NULL,
    goal_id INTEGER NOT NULL,
    time DATETIME NOT NULL,
    PRIMARY KEY (user_id, goal_id),
    FOREIGN KEY (user_id) REFERENCES user (id),
    FOREIGN KEY (goal_id) REFERENCES goal (id)
);
