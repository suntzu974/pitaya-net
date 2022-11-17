UPDATE users
SET email = $1,
username = $2,
image = $3,
bio = $4
WHERE  slug = $5 
RETURNING $table_fields;