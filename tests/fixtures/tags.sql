INSERT INTO tag_category
    (id, name)
VALUES
    (1, 'bestuur'),
    (2, 'toren');

INSERT INTO tag
    (id, name, description, category)
VALUES
    (1, 'bestuur', 'Ik ben huidig bestuur', 1),
    (2, 'boekentoren', 'Ik ben een boekentoren', 2),
    (3, 'eiffel', 'Ik ben een eiffeltoren', 2);

INSERT INTO user_tag
    (user_id, tag_id)
VALUES
    (2, 1),
    (2, 2);
