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