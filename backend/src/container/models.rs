use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use crate::errors::ServiceError;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use mobc_postgres::{tokio_postgres::{Row}};
use actix_multipart_extract::{File, MultipartForm};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,PostgresMapper)]

#[pg_mapper(table="containers")]
pub struct Container {
    pub id:i32,
    pub container:String,
    pub facture:i32,
    pub article:String,
    pub designation:String,
    pub poids_colis:f32,
    pub poids_commande:f32,
    pub volume:f32,
    pub pcb:i32,
    pub spcb:i32,
    pub pv:f32,
    pub pvconseil:f32,
    pub qte:i32,
    pub montant:f32,
    pub date:i32,
    pub palette:String,
    pub origine:String,
    pub ean:String,
    pub theme:String,
    pub codedouanier:String,
    pub commande:String,
    pub libunivers:String,
    pub univers:String,
    pub libfamille:String,
    pub famille:String,
    pub libsfamille:String,
    pub sfamille:String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputContainer {
    pub container:String,
    pub facture:i32,
    pub article:String,
    pub designation:String,
    pub poids_colis:f32,
    pub poids_commande:f32,
    pub volume:f32,
    pub pcb:i32,
    pub spcb:i32,
    pub pv:f32,
    pub pvconseil:f32,
    pub qte:i32,
    pub montant:f32,
    pub date:i32,
    pub palette:String,
    pub origine:String,
    pub ean:String,
    pub theme:String,
    pub codedouanier:String,
    pub commande:String,
    pub libunivers:String,
    pub univers:String,
    pub libfamille:String,
    pub famille:String,
    pub libsfamille:String,
    pub sfamille:String,
}

#[derive(Deserialize, MultipartForm, Debug)]
pub struct ContainerForm {
    #[multipart(max_size = 12MB)]
    pub title: String,
    pub description: String, 
    pub file: File,
}
#[derive(Serialize, Deserialize)]
pub struct FileDownloaded {
    pub name: String,
}

impl Container {

    pub async fn insert(
        client: &Client,
        container: InputContainer
    ) -> Result<Container, ServiceError> {
        let _stmt = include_str!("../../sql/container/add_container.sql");
        let _stmt = _stmt.replace("$table_fields", &Container::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
        .query(
            &stmt,
            &[
                &container.container,
                &container.facture,
                &container.article,
                &container.designation,
                &container.poids_colis,
                &container.poids_commande,
                &container.volume,
                &container.pcb,
                &container.spcb,
                &container.pv,
                &container.pvconseil,
                &container.qte,
                &container.montant,
                &container.date,
                &container.palette,
                &container.origine,
                &container.ean,
                &container.theme,
                &container.codedouanier,
                &container.commande,
                &container.libunivers,
                &container.univers,
                &container.libfamille,
                &container.famille,
                &container.libsfamille,
                &container.sfamille,
            ],
        )
        .await?
        .iter()
        .map(|row| Container::from_row_ref(row).unwrap())
        .collect::<Vec<Container>>()
        .pop()
        .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }
    pub async fn update(
        review_id: i32,
        review: InputReview,
        client: &Client ) -> Result<Container, ServiceError> {
        let _stmt = include_str!("../../sql/review/update_container.sql");
        let _stmt = _stmt.replace("$table_fields", &Container::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &container_id,
                    &container.container,
                    &container.facture,
                    &container.article,
                    &container.designation,
                    &container.poids_colis,
                    &container.poids_commande,
                    &container.volume,
                    &container.pcb,
                    &container.spcb,
                    &container.pv,
                    &container.pvconseil,
                    &container.qte,
                    &container.montant,
                    &container.date,
                    &container.palette,
                    &container.origine,
                    &container.ean,
                    &container.theme,
                    &container.codedouanier,
                    &container.commande,
                    &container.libunivers,
                    &container.univers,
                    &container.libfamille,
                    &container.famille,
                    &container.libsfamille,
                    &container.sfamille,
                ],
            )
            .await?
            .iter()
            .map(|row| Container::from_row_ref(row).unwrap())
            .collect::<Vec<Container>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    
    }
    pub async fn delete_container(container_id: i32, client: &Client) -> Result<Container, ServiceError> {
        let _stmt = include_str!("../../sql/container/delete_container.sql");
        let _stmt = _stmt.replace("$table_fields", &Container::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &container_id,
                ],
            )
            .await?
            .iter()
            .map(|row| Container::from_row_ref(row).unwrap())
            .collect::<Vec<Container>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }

    pub async fn get_container_by_id(container_id: i32, client: &Client) -> Result<Container, ServiceError> {
        let _stmt = include_str!("../../sql/container/get_container_by_id.sql");
        let _stmt = _stmt.replace("$table_fields", &Container::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &container_id,
                ],
            )
            .await?
            .iter()
            .map(|row| Container::from_row_ref(row).unwrap())
            .collect::<Vec<Container>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) 
    }
    pub async fn get_containers(client: &Client) -> Result<Vec<Container>, ServiceError> {
        let _stmt = include_str!("../../sql/container/get_containers.sql");
        let _stmt = _stmt.replace("$table_fields", &Container::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows
            .into_iter()
            .map(|row| Container::from(row))
            .collect())
    }
}
impl From<Row> for Container {
    fn from(row: Row) -> Self {
        Self {
            id : row.get(0),
            container: row.get(1),
            facture:row.get(2),
            article:row.get(3),
            designation: row.get(4),
            poids_colis: row.get(5),
            poids_commande: row.get(6),
            volume: row.get(7),
            pcb: row.get(7),
            spcb: row.get(8),
            pv: row.get(9),
            pvconseil: row.get(10),
            qte: row.get(11),
            montant: row.get(12),
            date: row.get(13),
            palette: row.get(14),
            origine: row.get(15),
            ean: row.get(16),
            theme: row.get(17),
            codedouanier: row.get(18),
            commmande: row.get(19),
            libunivers: row.get(20),
            univers: row.get(21),
            libfamile: row.get(22),
            famille: row.get(23),
            libsfamille: row.get(24),
            sfamille: row.get(25),
        }
    }
}
