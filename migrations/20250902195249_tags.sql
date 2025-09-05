CREATE TABLE tag_category
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE tag
(
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category INTEGER NOT NULL,

    FOREIGN KEY (category) REFERENCES tag_category (id)
);

CREATE TABLE user_tag
(
    user_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id)  REFERENCES tag (id) ON DELETE CASCADE,

    PRIMARY KEY (user_id, tag_id)
);
