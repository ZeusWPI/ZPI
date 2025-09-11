INSERT INTO achievement
    (id, name, service)
VALUES
    (1, 'Achievements', 1),
    (2, 'Profile Picture', 1),
    (3, 'Votes', 2);

INSERT INTO goal
    (id, description, achievement, sequence)
VALUES
    (1, 'Get 1 achievement', 1, 1),
    (2, 'Get 2 achievements', 1, 2),
    (3, 'Upload a profile picture', 2, 1),
    (4, 'Vote 1 time', 3, 1);