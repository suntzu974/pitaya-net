with inserted_comment as (
    -- Insert a new record into comment
    insert into comments (body,slug,author ) 
	values ($1, $2,$3) returning *
) select c.id,c.created_at,c.updated_at,c.body,
	u.username,u.bio,u.image,u.following  
	from inserted_comment c
	left join users u 
	on c.author=u.slug;

