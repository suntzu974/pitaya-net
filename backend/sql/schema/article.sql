

CREATE TABLE articles 
(
    slug VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    body VARCHAR(255) NOT NULL,
    tag_list TEXT[] NOT NULL DEFAULT '{}'::text[],
    created_at TIMESTAMPTZ NOT NULL  default (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL  default (now() at time zone 'utc'),
    favorited BOOL default false,
    favorites_count INT default 0,
    author varchar(255) NOT NULL,
    PRIMARY KEY(slug),
    FOREIGN KEY (author) REFERENCES users (slug)
);