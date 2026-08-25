use zbus::proxy;

#[proxy(
    interface = "com.lingmoos.LingmoGreeter",
    default_service = "com.lingmoos.LingmoGreeter",
    default_path = "/com/system76/CosmicGreeter"
)]
pub trait Greeter {
    async fn initial_setup_end(&mut self, new_user: String) -> Result<(), zbus::Error>;
}
