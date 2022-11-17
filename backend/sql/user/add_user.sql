INSERT INTO users(slug,username, email,password,token)
VALUES ($1,$2,$3,$4,$1)
RETURNING $table_fields;