#[derive(PartialEq)]
pub enum HttpRequestType {
    GET,
    POST,
    PUT,
    UPDATE,
}

pub struct HttpRequest {
    pub request_type: HttpRequestType,
    pub path: String,
}

pub enum HttpResponseStatus {
    Ok200,
    NotFound404,
}

pub struct HttpResponse {
    pub status: HttpResponseStatus,
    pub body: String,
}
