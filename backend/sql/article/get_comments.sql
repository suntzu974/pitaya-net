select c.id,c.created_at,
c.updated_at,c.body,
u.username,u.bio,u.image,u.following 
from comments c, users u
where u.slug = c.author 
and c.slug = $1 ;
