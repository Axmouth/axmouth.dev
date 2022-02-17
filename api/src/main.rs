#![allow(dead_code)]
use backend_api::app;
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    #[cfg(target_os = "linux")]
    tokio_uring::start(async move {
        app::start().await;
    });

    #[cfg(not(target_os = "linux"))]
    {
        let rt = Runtime::new()?;
        rt.block_on(async {
            app::start().await;
        });
    }

    Ok(())
}
