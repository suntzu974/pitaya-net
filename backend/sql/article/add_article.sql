with inserted_article as (
    -- Insert a new record into articles
    insert into articles (slug,title,description,body,tag_list,author ) 
	values ($1, $2,$3,$4,$5,$6) returning *
) select a.slug,a.title,a.description,a.body,a.tag_list,a.created_at,a.updated_at,a.favorited,
	a.favorites_count,u.username,u.bio,u.image,u.following 
	from inserted_article a
	left join users u 
	on a.author=u.slug;

