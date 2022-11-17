CREATE TABLE comments 
(
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL  default (now() at time zone 'utc'),
    updated_at TIMESTAMPTZ NOT NULL  default (now() at time zone 'utc'),
    body varchar(255) NOT NULL,
    slug varchar(255) NOT NULL,
    author varchar(255) NOT NULL,
    FOREIGN KEY (slug) REFERENCES articles (slug),
    FOREIGN KEY (author) REFERENCES users (slug)
);