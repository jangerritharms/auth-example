pub struct Storage {}

impl Storage {
    fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::options::ClientOptions;
    use mongodb::bson::Document;
    use mongodb::Client;

    #[actix_rt::test]
    async fn test_storage() {
        let options = ClientOptions::parse("mongodb://root:rootpassword@localhost:27017")
            .await
            .unwrap();
        let client = Client::with_options(options).unwrap();

        let db = client.database("local");
        for coll_name in db.list_collection_names(None).await.unwrap() {
            let coll = db.collection::<Document>(&coll_name);
            let result = coll.find(None, None).await.unwrap();
            println!("collection: {:?}", result);
        }
        assert!(false)
    }
}
