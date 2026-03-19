use reqwest::{Client, header};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::model::ApiResponse;
use crate::utils::loader::load_api_key;

// 缓存结构
struct CacheEntry {
    value: String,
    timestamp: Instant,
}

pub struct UsageClient {
    api_key: String,
    cache: Arc<RwLock<Option<CacheEntry>>>,
    ttl: Duration,
}

impl UsageClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            cache: Arc::new(RwLock::new(None)),
            ttl: Duration::from_secs(30),
        }
    }

    /// 获取用量（带30秒缓存）
    pub async fn fetch_usage(&self) -> String {
        // 先检查缓存
        {
            let cache_read = self.cache.read().await;
            if let Some(entry) = cache_read.as_ref()
                && entry.timestamp.elapsed() < self.ttl {
                    // 缓存有效，直接返回
                    return entry.value.clone();
                }
        } // 读锁在这里释放

        // 缓存无效，获取写锁并重新请求
        let mut cache_write = self.cache.write().await;

        // 双重检查（可能其他任务已更新）
        if let Some(entry) = cache_write.as_ref()
            && entry.timestamp.elapsed() < self.ttl {
                return entry.value.clone();
            }

        // 执行真实请求
        let value = self.do_fetch().await;

        // 更新缓存
        *cache_write = Some(CacheEntry {
            value: value.clone(),
            timestamp: Instant::now(),
        });

        value
    }

    /// 强制刷新缓存
    pub async fn refresh(&self) -> String {
        let mut cache = self.cache.write().await;
        let value = self.do_fetch().await;
        *cache = Some(CacheEntry {
            value: value.clone(),
            timestamp: Instant::now(),
        });
        value
    }

    /// 清除缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        *cache = None;
    }

    /// 内部实际请求
    async fn do_fetch(&self) -> String {
        let client = match Client::builder().timeout(Duration::from_secs(30)).build() {
            Ok(c) => c,
            Err(e) => return format!("client error: {}", e),
        };

        let resp = match client
            .get("https://www.minimaxi.com/v1/api/openplatform/coding_plan/remains")
            .bearer_auth(&self.api_key)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => return format!("request error: {}", e),
        };

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return format!("api error: {}", err_text);
        }

        let data: ApiResponse = match resp.json().await {
            Ok(d) => d,
            Err(e) => return format!("parse error: {}", e),
        };

        if let Some(model) = data.model_remains.first() {
            let remaining_percent = if model.current_interval_total_count > 0 {
                ((model.current_interval_total_count - model.current_interval_usage_count) as f64
                    / model.current_interval_total_count as f64
                    * 100.0) as i32
            } else {
                0
            };

            let total_minutes = model.remains_time / 60000;
            let hours = total_minutes / 60;
            let minutes = total_minutes % 60;

            let reset_text = match (hours, minutes) {
                (h, 0) if h > 0 => format!("resets in {}h", h),
                (0, m) if m > 0 => format!("resets in {}m", m),
                (h, m) => format!("resets in {}h {}m", h, m),
            };

            return format!("remaining_percent: {}%, {}", remaining_percent, reset_text);
        }

        "no data".to_string()
    }
}

// 全局单例（可选）
lazy_static::lazy_static! {
    static ref GLOBAL_CLIENT: Arc<RwLock<Option<UsageClient>>> = Arc::new(RwLock::new(None));
}

pub async fn init_global_client(api_key: impl Into<String>) {
    let mut client = GLOBAL_CLIENT.write().await;
    *client = Some(UsageClient::new(api_key));
}

pub async fn fetch_usage() -> String {
    // 懒初始化：如果未初始化，则读取 API key 并初始化
    {
        let client = GLOBAL_CLIENT.read().await;
        if client.is_some() {
            return client.as_ref().unwrap().fetch_usage().await;
        }
    } // 释放读锁

    // 读取 API key
    let api_key = load_api_key();
    if api_key.is_empty() {
        return "API key not found".to_string();
    }

    // 获取写锁并初始化
    let mut client = GLOBAL_CLIENT.write().await;
    *client = Some(UsageClient::new(api_key));

    // 获取写锁后再次检查（可能其他任务已初始化）
    if let Some(c) = client.as_ref() {
        c.fetch_usage().await
    } else {
        "init failed".to_string()
    }
}
