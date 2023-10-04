/*
A rust version of DynamoDB some CRUD
*/
use aws_sdk_dynamodb::operation::create_table::CreateTableOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, ScalarAttributeType};
use aws_sdk_dynamodb::{Client, Error};
use std::time::Duration;

pub struct Item {
    pub username: String,
}

#[derive(Debug, PartialEq)]
pub struct ItemOut {
    pub username: Option<AttributeValue>,
}
const TABLE_NAME: &str = "customers";

//create a Table in DynamoDB
async fn create_table() -> Result<CreateTableOutput, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let ad = AttributeDefinition::builder()
        .attribute_name("username")
        .attribute_type(ScalarAttributeType::S)
        .build();
    let ks = KeySchemaElement::builder()
        .attribute_name("username")
        .key_type(aws_sdk_dynamodb::types::KeyType::Hash)
        .build();
    let pt = aws_sdk_dynamodb::types::ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();
    let request = client
        .create_table()
        .table_name(TABLE_NAME)
        .attribute_definitions(ad)
        .key_schema(ks)
        .provisioned_throughput(pt);
    let resp = request.send().await?;
    println!("{:?}", resp);
    Ok(resp)
}

async fn delete_table() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let resp = client.delete_table().table_name(TABLE_NAME).send().await?;
    println!("{:?}", resp);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    //create a table, then wait for 10 seconds and delete it
    println!("Creating table and sleeping 10 seconds");
    create_table().await?;
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("Deleting table");
    delete_table().await?;
    Ok(())
}
