//Build a script to list the buckets in my S3 account.

use aws_sdk_s3 as s3;

#[::tokio::main]
async fn main() -> Result<(), s3::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    //list buckets
    let resp = client.list_buckets().send().await?;
    println!("Buckets:");
    for bucket in resp.buckets.unwrap_or_default() {
        println!("  {}", bucket.name.unwrap_or_default());
    }

    Ok(())
}