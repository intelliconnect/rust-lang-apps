use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use rusoto_core::ByteStream;
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
use serde_json::json;

use crate::model::dbmethods;
use crate::model::structs;

#[tokio::main]
pub async fn lambda_example_synchronus() -> HttpResponse {
    //create AWS Lambda client with Region same as region of Lambda funmction (This uses AWS credentials from .env file)
    let client = LambdaClient::new(Region::ApSoutheast1);
    //Create body of Invocation Request
    let request = InvocationRequest {
        function_name: "helloWorldForRust".to_string(),
        invocation_type: Some("RequestResponse".to_string()),
        ..Default::default()
    };

    match client.invoke(request).await {
        Ok(response) => match response.payload {
            Some(b_body) => {
                let vec_body = b_body.to_vec();
                match String::from_utf8(vec_body) {
                    Ok(body) => {
                        let body = serde_json::from_str::<structs::Lambdastruct>(&body);
                        match body {
                            Ok(rbody) => HttpResponse::Ok().json(rbody),
                            Err(err) => {
                                warn!("{:?}", err);
                                let errs = err.to_string();
                                HttpResponse::InternalServerError().json(json!({ "Error": [errs] }))
                            }
                        }
                    }
                    Err(err) => {
                        warn!("{:?}", err);
                        let errs = err.to_string();
                        HttpResponse::InternalServerError().json(json!({ "Error": [errs] }))
                    }
                }
            }
            None => {
                warn!("empty response from lambda");
                HttpResponse::InternalServerError().json("No response from lambda".to_string())
            }
        },
        Err(err) => {
            warn!("{:?}", err);
            let errs = err.to_string();
            HttpResponse::InternalServerError().json(json!({ "Error": [errs] }))
        }
    }
}

pub async fn upload_file(mut payload: Multipart) -> HttpResponse {
    //get field(Actix streaming body) from  payload
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disp = field.content_disposition().unwrap();

        //get filename from field
        match content_disp.get_filename() {
            Some(filename) => {
                //create filename for file to be saved
                let filename_tobe_saved = format!("{} - {}", chrono::Local::now(), filename,);
                //get chunks from field and upload them to AWS S3
                while let Some(chunk) = field.next().await {
                    //Convert from bytes to bytestream
                    let data = chunk.unwrap().to_vec();
                    let bst = ByteStream::from(data);
                    //upload file to AWS S3
                    return match dbmethods::send_to_s3(bst, filename_tobe_saved.clone()) {
                        Ok(response) => {
                            HttpResponse::Ok().json(format!("Uploaded - {:?}", response))
                        }
                        Err(err) => HttpResponse::InternalServerError()
                            .json(format!("Failed to upload {:?}", err)),
                    };
                }
            }
            None => {
                warn!("no upload file found");
                return HttpResponse::Ok().body("no upload file found");
            }
        }
    }
    HttpResponse::Ok().body("Upload failed")
}

pub async fn dynamodb_example(jsondata: web::Json<structs::Dynamouserid>) -> HttpResponse {
    match dbmethods::list_data(jsondata.id.clone()) {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().json(format!("{:?}", err)),
    }
}
