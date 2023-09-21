use oneblog::error;

#[tokio::main]
async fn main() -> Result<(), crate::error::OneBlogError> {
    oneblog::launch::launch_until_stopped().await?;


    Ok(())
}
