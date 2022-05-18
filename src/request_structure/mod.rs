use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};

// Structure for converting rust Request Event into a json struct
#[derive(Deserialize, Serialize, Debug)]
pub struct Records {
    pub Records: Vec<Items>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Items {
    pub awsRegion: String,
    pub eventName: String,
    pub eventSource: String,
    eventTime: String,
    eventVersion: String,
    pub s3: S3,
}
 
#[derive(Deserialize, Serialize, Debug)]
pub struct S3 {
    pub bucket: Bucket,
    configurationId: String,
} 

#[derive(Deserialize, Serialize, Debug)]
pub struct Bucket {
    pub arn: String,
    pub name: String,
}