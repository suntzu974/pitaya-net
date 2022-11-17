with inserted_comment as (
    -- Insert a new record into comment
    insert into comments (body,slug,author ) 
	values ($1, $2,$3) returning *
) select c.id,c.created_at,c.updated_at,c.body,
	a.slug as article,u.slug as author 
	from inserted_comment c
	left join users u 
	on c.author=u.slug
	left join articles a
	on c.slug = a.slug;

