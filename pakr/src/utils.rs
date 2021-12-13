// imports
use serde_json::json;

pub fn get_component_json_from_info(info: &mut serde_json::Map<String, serde_json::Value>) -> serde_json::Value {
    let script_folder_value = info.get("$scriptsFolder").unwrap_or(&serde_json::Value::Null);
    let mut result = String::from("");
    if script_folder_value == &serde_json::Value::Null {
        result = "null".to_string();
    } else {
        result = script_folder_value.to_string();
    }
    return json!({
        "$name": info["$name"].to_string().replace("\"", ""),
        "$installDir": info["$installDir"].to_string().replace("\"", ""),
        "$desc": info["$desc"].to_string().replace("\"", ""),
        "$payloadName": info["$payloadName"].to_string().replace("\"", ""),
        "$pkgName": info["$pkgName"].to_string().replace("\"", ""),
        "$selectable": info["$selectable"].as_bool(),
        "$selected": info["$selected"].as_bool(),
        "$visible": info["$visible"].as_bool(),
        "$scriptsFolder": result.replace("\"", "")
    })
}