use serde::{Deserialize, Serialize};

#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    url: String,
    method: String,
    pub body: String,
}

// Use if you want to pass body as json, but current az fn passes it as a string
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Body {
//     id: u32,
//     quantity: u32,
//     color: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Req {
    pub req: Request,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvokeRequest {
    #[serde(rename(deserialize = "Data"))]
    pub data: Req,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringBody {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseOutput {
    pub message: String,
    pub res: StringBody,
}

#[serde(rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct InvokeResponse {
    pub outputs: ResponseOutput,
    pub logs: Option<String>,
    pub return_value: Option<String>,
}
