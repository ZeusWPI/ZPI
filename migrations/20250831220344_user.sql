CREATE TABLE user
(
    id       INTEGER PRIMARY KEY NOT NULL,
    username TEXT                NOT NULL,
    about    TEXT		 NOT NULL DEFAULT ''
);
