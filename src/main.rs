use std::collections::HashMap;
use std::fs;
use std::path::Path;
use actix_web::Responder;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, Keys};
use serde::{Deserialize, Serialize};
use serde::ser::Error;
use serde_json::{from_str, json, Value};
use serde_json::de::Read;
use serde_json::map::Values;

#[derive(Serialize, Deserialize, Debug)]
struct Filter{
    offer_id: String,
    product_id: String,
    visibility:String
}
#[derive(Serialize, Deserialize, Debug)]
struct  ReqwestProduct{
    #[serde(rename = "responseItems")]
    filter:Filter,
    last_id: String,
    limit: i64
}
#[derive(Serialize, Deserialize, Debug)]
struct ResponseProduct{
    product_id: i64,
    offer_id: String,
    is_fbo_visible: bool,
    is_fbs_visible:bool,
    archived: bool,
    is_discounted: bool,
    // status:bool
}
#[derive(Serialize, Deserialize, Debug)]
struct ResWrapper {
    result: Res,
}
#[derive(Serialize, Deserialize, Debug)]
struct Res{
    items:Vec<ResponseProduct>,
    total: i32,
    last_id:String
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    ///Create req
    let client = Client::new();
    let url = "https://api-seller.ozon.ru/v2/product/list";
    ///Headers
    let mut headers = HeaderMap::new();
    headers.insert("Client-Id", HeaderValue::try_from(dotenv::var("CLIENT_ID_NUMBER").expect("Err set client id")).expect("Err read client id"));
    headers.insert("Api-Key", HeaderValue::try_from(dotenv::var("CLIENT_API_KEY").expect("Err set apikey")).expect("Err read client apikey"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    ///Req
    let filter = Filter{
        offer_id: "NU505718".to_string(),
        product_id: "".to_string(),
        visibility: "ALL".to_string(),
    };
    let json_reqwest_products = ReqwestProduct{
        filter,
        last_id: "".to_string(),
        limit: 1,
    };
    ///stable req body
    let json_data = serde_json::json!({"limit": 10});
    ///variable req body
    let json_data_2 = json!({
        "filter":
        {
            //"offer_id":[json_reqwest_products.filter.offer_id]
        },
        "limit":json_reqwest_products.limit});

    /*let new_result: Vec<ResWrapper> = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&json_data_2)
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", new_result);*/

    let response = client.post(url)
        .headers(headers.clone())
        .json(&json_data_2)
        .send()
        .await?;

    //Обработка ответа
    if response.status().is_success(){
        //для проверки
        /*let text_resp = response.text().await.expect("err to text");
        println!("{}", text_resp);
        let deser_response: ResWrapper = serde_json::from_str(&*text_resp).expect("");
        println!("{:#?}", deser_response);*/

        let text_resp:ResWrapper = response.json().await.expect("err");
        println!("{:#?}", text_resp);

}else {
      println!("Err {:?} {}", response.status(), response.text().await.expect("err post error response"));
    }


    Ok(())
}




