CREATE TABLE service
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE achievement
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    service INTEGER NOT NULL,

    FOREIGN KEY (service) REFERENCES service(id)
);

CREATE TABLE goal
(
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    achievement INTEGER NOT NULL,
    sequence INTEGER NOT NULL,

    FOREIGN KEY (achievement) REFERENCES achievement(id)
);

CREATE TABLE unlock
(
    user INTEGER NOT NULL,
    goal INTEGER NOT NULL,
    time DATETIME NOT NULL,

    PRIMARY KEY (user, goal),

    FOREIGN KEY (user) REFERENCES user(id),
    FOREIGN KEY (goal) REFERENCES goal(id)
);