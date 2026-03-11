use serde_json::Value;

pub fn load_api_key() -> String {
    // 获取家目录
    let home = dirs::home_dir().ok_or("无法获取家目录").unwrap();

    // 构建文件路径
    let file_path = home.join(".claude").join("settings.json");

    // 读取文件
    let content = std::fs::read_to_string(&file_path).unwrap();

    // 解析 JSON
    let json: Value = serde_json::from_str(&content).unwrap();

    json.get("env")
        .unwrap()
        .get("ANTHROPIC_AUTH_TOKEN")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}
