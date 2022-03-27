CREATE TABLE IF NOT EXISTS TODO (
    ID UUID PRIMARY KEY,
    content TEXT NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    overdue_time TIMESTAMP WITH TIME ZONE,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO TODO (
    ID,
    content,
    start_time
) VALUES (
    gen_random_uuid(),
    'setup env for AmpPlus and run it',
    CURRENT_TIMESTAMP
);

INSERT INTO TODO (
    ID,
    content,
    start_time
) VALUES (
    gen_random_uuid(),
    'test postgresql date and time',
    CURRENT_TIMESTAMP
);


INSERT INTO TODO (
    ID,
    content,
    start_time
) VALUES (
    gen_random_uuid(),
    'insert JS Date String Into database',
    '2022-03-25T04:36:56.755Z'
);

