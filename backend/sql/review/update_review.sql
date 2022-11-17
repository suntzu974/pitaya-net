UPDATE reviews
SET title = $2 ,
 description = $3
WHERE
   id = $1 
RETURNING $table_fields;