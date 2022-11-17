select c.id,c.created_at,
c.updated_at,c.body,c.slug,
u.username as author 
from comments c, users u
where u.slug = c.author 
and c.slug = $1 ;
