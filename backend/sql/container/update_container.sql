UPDATE containers
SET 
    container = $2 ,
    facture = $3,
    article = =$4,
    designation = $5,
    poids_colis= $6,
    poids_commande= $7,
    volume= $8,
    pcb= $9,
    spcb= $10,
    pv= $11,
    pvconseil= $12,
    qte= $13,
    montant= $14,
    date= $15,
    palette= $16,
    origine= $17,
    ean= $18,
    theme= $19,
    codedouanier= $20,
    commande= $21,
    libunivers= $22,
    univers= $23,
    libfamille= $24,
    famille= $25,
    libsfamille= $26,
    sfamille= $27

WHERE
    id = $1 
RETURNING $table_fields;
with inserted_container as (
    -- Insert a new record into articles
    insert into containers (
        container,facture,article,designation,
        poids_colis,poids_commande,volume,
        pcb,spcb,pv,pvconseil,qte,montant,
        date,palette,origine,ean,theme,
        codedouanier,commande,libunivers,univers,
        libfamille,famille,libsfamille,sfamille) 
	values ($1, $2,$3,$4,$5,$6,
            $6,$7,$8,$9,$10,$11,$12,$13,
            $14,$15,$16,$17,$18,$19,$20,
            $21,$22,$23,$24,$25,$26) returning *
) select a.container,a.facture,a.article,a.designation,
        a.poids_colis,a.poids_commande,a.volume,
        a.pcb,a.spcb,a.pv,a.pvconseil,a.qte,a.montant,
        a.date,a.palette,a.origine,a.ean,a.theme,
        a.codedouanier,a.commande,a.libunivers,a.univers,
        a.libfamille,a.famille,a.libsfamille,a.sfamille
        from inserted_container &;