use gql_client::Client;
use std::collections::HashMap;
use serde_json::Value;
use serde_json::json;
use polars::prelude::*;
//use chrono::prelude::*;
//use polars_excel_writer::ExcelWriter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let endpoint = "https://open.api.woztell.com/v3";
   let mut headers = HashMap::new();
   headers.insert("Authorization", "Bearer [api]}");
   let mut query = r#"
        query getChatHistory ($first: IntMax100) {
            apiViewer {
                members (first: $first) {
                    edges {
                        node {
                            botMeta {
                                subscribe
                            },
                            externalId,
                            tags
                        }
                    },
                    pageInfo {
                        endCursor, 
                        hasNextPage
                    }
                }
            }
        }
    "#;

   let client = Client::new_with_headers(endpoint, headers);
   let mut repeat_variable = json!({"first": 100});
   //let mut data_vec: Vec<Value> = Vec::new();
   let mut subscribe_values = Vec::new();
   let mut external_id_values = Vec::new();
   let mut tag_values = Vec::new();
   let mut has_next_page: bool = true;

   while has_next_page == true {
    let data = client.query_with_vars_unwrap::<Value, Value>(query, repeat_variable.clone()).await.unwrap();
    //data_vec.push(data.clone());
    let end_cursor = data["apiViewer"]["members"]["pageInfo"]["endCursor"].to_string().replace("\"", "");
    has_next_page = data["apiViewer"]["members"]["pageInfo"]["hasNextPage"].to_string().parse().unwrap();
    query = r#"
        query getChatHistory ($after: String, $first: IntMax100) {
            apiViewer {
                members (after: $after, first: $first) {
                    edges {
                        node {
                            botMeta {
                                subscribe
                            },
                            externalId,
                            tags
                        }
                    },
                    pageInfo {
                        endCursor, 
                        hasNextPage
                    }
                }
            }
        }
    "#;
    repeat_variable = json!({"first": 100, "after": end_cursor});
    let responses = data["apiViewer"]["members"]["edges"].as_array().unwrap();

    for response in responses {
        let subscribe = response["node"]["botMeta"]["subscribe"].to_string();
        let external_id = response["node"]["externalId"].to_string().replace("\"", "");
        let tag = response["node"]["tags"].to_string().replace("\"", "").replace("[", "").replace("]", "");
        subscribe_values.push(subscribe);
        external_id_values.push(external_id);
        tag_values.push(tag);
    }
   }
   let mobiles_column = Series::new("mobile", &external_id_values);
    let subscribes_column = Series::new("subscribe", &subscribe_values);
    let tag_column = Series::new("tag", &tag_values);
    let mut df = DataFrame::new(vec![subscribes_column, mobiles_column, tag_column]).unwrap();
    println!("{:?}", df);

   println!("{:?}", external_id_values.len());
   println!("{:?}", subscribe_values.len());
   println!("{:?}", tag_values.len());
    let mut file = std::fs::File::create("member.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();


   

   //let mut mobiles: Vec<String> = Vec::new();
   //for mobile in 0..100 {
      //println!("{}", data["apiViewer"]["members"]["edges"][user]["node"]["externalId"]);
    //  mobiles.push(data["apiViewer"]["members"]["edges"][mobile]["node"]["externalId"].to_string().replace("\"", ""));
    //}

    //let mut subscribes: Vec<String> = Vec::new();
    //for subscribe in 0..100 {
       //println!("{}", data["apiViewer"]["members"]["edges"][user]["node"]["externalId"]);
      // subscribes.push(data["apiViewer"]["members"]["edges"][subscribe]["node"]["botMeta"]["subscribe"].to_string().replace("\"", ""));
     //}


    //let mobiles_column = Series::new("mobile", &mobiles);
    //let subscribes_column = Series::new("subscribe", &subscribes);
    //let mut df = DataFrame::new(vec![mobiles_column, subscribes_column]).unwrap();
    //let mut end_cursor = data["apiViewer"]["members"]["pageInfo"]["endCursor"].to_string();
    //has_next_page = data["apiViewer"]["members"]["pageInfo"]["hasNextPage"].to_string().parse().unwrap();

    //println!("{:?}", has_next_page);
    //println!("{:?}", has_next_page);

    //example1(&mut df).unwrap();
    //let mut file = std::fs::File::create("path.csv").unwrap();
    //CsvWriter::new(&mut file).finish(&mut df).unwrap();


   Ok(())
}