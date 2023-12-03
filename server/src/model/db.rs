use lazy_static::lazy_static;
use sea_orm::{Database, DatabaseConnection};
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref DB: DatabaseConnection = {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            Database::connect("sqlite://circuit.db?mode=rwc").await.unwrap()
        })
    };
}