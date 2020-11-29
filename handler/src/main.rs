use warp::Filter;
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(rename(deserialize = "Url"))]
    url: String,
    #[serde(rename(deserialize = "Method"))] 
    method: String,
    #[serde(rename(deserialize = "Body"))] 
    body: String,
}


// Use if you want to pass body as json, but current az fn passes it as a string
// #[derive(Serialize, Deserialize, Debug)]
// struct Body {
//     id: u32,
//     quantity: u32,
//     color: String,
// }

#[derive(Serialize, Deserialize, Debug)]
struct Req {
    req: Request,
}

#[derive(Serialize, Deserialize, Debug)]
struct InvokeRequest {
    #[serde(rename(deserialize = "Data"))] 
    data: Req,
}

#[derive(Serialize, Deserialize, Debug)]
struct StringBody {
    body: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseOutput {
    message: String,
    res: StringBody,
}

#[derive(Serialize, Deserialize, Debug)]
struct InvokeResponse {
    #[serde(rename(serialize = "Outputs"))] 
    outputs: ResponseOutput,
    #[serde(rename(serialize = "Logs"))] 
    logs: Option<String>,
    #[serde(rename(serialize = "ReturnValue"))] 
    return_value: Option<String>
}

async fn order(invoke_req: InvokeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("Incoming request: \n {:#?}", invoke_req);

    let res = StringBody {body: "Order enqueued".to_string()};
    let outputs = ResponseOutput {message: invoke_req.data.req.body, res};
    let invoke_res = InvokeResponse {outputs, logs: None, return_value: None};

    log::debug!("Outgoing response: \n {}", serde_json::to_string_pretty(&invoke_res).unwrap());
    Ok(warp::reply::json(&invoke_res))

}
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "handler=debug");
    }
    pretty_env_logger::init();

    let port = env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
    .map(|p| p.parse::<u16>().unwrap_or(3030_u16)).unwrap_or(3030_u16);

    let order = warp::post()
        .and(warp::path("order"))
        .and(warp::body::json())
        .and_then(order);

    warp::serve(order)
        .run(([127, 0, 0, 1], port))
        .await;
}