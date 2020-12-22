use std::env;
use warp::Filter;

mod types;
use types::*;

async fn order(invoke_req: InvokeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("Incoming request: \n {:#?}", invoke_req);

    let res = StringBody {
        body: "Order enqueued".to_string(),
    };
    let outputs = ResponseOutput {
        message: invoke_req.data.req.body,
        res,
    };
    let invoke_res = InvokeResponse {
        outputs,
        logs: None,
        return_value: None,
    };

    log::debug!(
        "Outgoing response: \n {}",
        serde_json::to_string_pretty(&invoke_res).unwrap()
    );
    Ok(warp::reply::json(&invoke_res))
}
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "handler=debug");
    }
    pretty_env_logger::init();

    let port = env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .ok()
        .and_then(|val| val.parse::<u16>().ok())
        .unwrap_or(3030);

    let order = warp::post()
        .and(warp::path("order"))
        .and(warp::body::json())
        .and_then(order);

    warp::serve(order).run(([127, 0, 0, 1], port)).await;
}
