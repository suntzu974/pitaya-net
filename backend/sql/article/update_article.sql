with updated_article as (
    update articles 
        SET title = $1,
        description = $2,
        body = $3,
        tag_list = $4
        where slug = $5
        returning *
) select a.slug,a.title,a.description,a.body,a.tag_list,a.created_at,a.updated_at,a.favorited,
	a.favorites_count,u.username,u.bio,u.image,u.following 
	from updated_article a
	left join users u 
	on a.author=u.slug;
