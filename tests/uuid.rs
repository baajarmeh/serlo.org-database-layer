#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use database_layer_actix::create_database_pool;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_add() {
        // TODO: need to move data creation out of main.
        let pool = create_database_pool().await.unwrap();
        let app = App::new()
            .data(pool.clone())
            .configure(database_layer_actix::uuid::init);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/uuid/1").to_request();
        let mut resp = test::call_service(&mut app, req).await;

        // assert!(resp.status().is_success());

        // first chunk
        let (bytes, _) = resp.take_body().into_future().await;
        assert_eq!(
            bytes.unwrap().unwrap(),
            web::Bytes::from_static(b"data: 5\n\n")
        );

        // assert!(resp.status().is_success());
    }
}
