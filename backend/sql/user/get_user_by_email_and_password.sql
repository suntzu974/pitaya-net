SELECT $table_fields FROM users
WHERE
   email = $1 AND password = $2;