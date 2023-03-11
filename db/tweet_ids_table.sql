CREATE TABLE tweet_ids (
    -- twitter stores tweet ids as 64 bit unsigned ints,
    -- but postgres does not support this type. We can
    -- get around this by using a varchar.
    tweet_id varchar(21) NOT NULL UNIQUE
);