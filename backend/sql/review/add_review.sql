INSERT INTO reviews(title,description,
original,thumbnail,web,deleted)
VALUES ($1,$2,$3,$4,$5,$6)
RETURNING $table_fields;