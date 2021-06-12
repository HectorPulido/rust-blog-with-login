CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  slug varchar NOT NULL unique,
  author_id SERIAL NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  created_at timestamp not null default current_timestamp,
  updated_at timestamp not null default current_timestamp,
  foreign key (author_id) references users(id)
)