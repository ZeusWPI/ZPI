CREATE TABLE tag_type
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE tag
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    type INTEGER NOT NULL,

    FOREIGN KEY (type) REFERENCES type (id)
);

CREATE TABLE user_tag
(
    user_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id)  REFERENCES tag (id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, tag_id)
);
