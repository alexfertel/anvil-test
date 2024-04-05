#[tokio::test]
async fn spawns_anvil() {
    let (api, handle) = anvil::spawn(Default::default()).await;
}
