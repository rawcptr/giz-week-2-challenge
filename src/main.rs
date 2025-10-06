#[cfg(feature = "async-rt")]
mod async_server;

#[cfg(not(feature = "async-rt"))]
mod sync_server;

pub mod common;

#[cfg(feature = "async-rt")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    async_server::run().await
}

#[cfg(not(feature = "async-rt"))]
fn main() -> std::io::Result<()> {
    sync_server::run()
}
