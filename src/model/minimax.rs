use serde::Deserialize;

// API 响应结构体
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub base_resp: BaseResp,
    pub model_remains: Vec<ModelRemain>,
}

#[derive(Deserialize, Debug)]
pub struct BaseResp {
    pub status_code: i32,
    pub status_msg: String,
}

#[derive(Deserialize, Debug)]
pub struct ModelRemain {
    pub current_interval_total_count: i32,
    pub current_interval_usage_count: i32,
    pub end_time: i64,
    pub model_name: String,
    pub remains_time: i64,
    pub start_time: i64,
}
