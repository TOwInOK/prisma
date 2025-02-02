pub trait Save {
    fn save() -> impl std::future::Future<Output = ()> + Send + Sync {
        async {}
    }
}
