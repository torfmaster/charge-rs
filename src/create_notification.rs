use tokio::process::Command;

pub async fn send(text: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let child = Command::new("/usr/bin/notify-send").arg(text).spawn();

    let future = child?;
    future.await?;
    Ok(())
}
