use aws_config::{defaults, meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::{primitives::ByteStream, Client};

pub async fn read_file(bucket: &str, key: &str) -> ByteStream {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
    let config = defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let s3_client = Client::new(&config);
    println!("Fetching {}/{} from S3", bucket, key);
    let res = s3_client.get_object().bucket(bucket).key(key).send().await;

    return match res {
        Ok(resp) => resp.body,
        Err(e) => {
            let err = e.into_service_error();
            panic!("Got an error reading object: {}", err.to_string());
        }
    };
}
