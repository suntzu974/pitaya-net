select a.slug,a.title,a.description,a.body,a.tag_list,a.created_at,
a.updated_at,a.favorited,a.favorites_count,
u.username,u.bio,u.image,u.following 
from articles a, users u
where u.slug = a.author 
and u.token = $3
limit $1 offset $2 ;
