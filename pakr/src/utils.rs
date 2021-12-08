// imports
use serde_json::json;

pub fn get_component_json_from_info(info: &mut serde_json::Map<String, serde_json::Value>) -> serde_json::Value {
    return json!({
        "$name": info["$name"].to_string().replace("\"", ""),
        "$installDir": info["$installDir"].to_string().replace("\"", ""),
        "$desc": info["$desc"].to_string().replace("\"", ""),
        "$payloadName": info["$payloadName"].to_string().replace("\"", ""),
        "$pkgName": info["$pkgName"].to_string().replace("\"", ""),
        "$selectable": false,
        "$selected": true,
        "$visible": true
    })
}