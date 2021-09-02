use regex::Regex;

pub fn gen_enum_json(prev: &[String], params: &[crate::Argument]) -> String {
    if let Some(cur) = prev.first() {
        let inner = gen_enum_json(&prev[1..prev.len()], params);
        format!("{{ \"{}\": {} }}", cur, inner)
    } else {
        let inner = params
            .iter()
            .map(|r| r.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{ {} }}", inner)
    }
}

// 對 typescript-definitions 生成的 typescript type 再做一次轉換
pub fn type_convert(s: &str) -> String {
    let re = Regex::new(r"DateTime< *Utc *>").unwrap();
    let result = re.replace_all(s, "string");
    result.to_owned().to_string()
}

pub fn to_typescript_type(path: &str) -> String {
    let re = Regex::new(r"\w*::").unwrap();
    let result = re.replace_all(path, "");
    let re = Regex::new(r"Vec").unwrap();
    let result = re.replace_all(result.as_ref(), "Array");
    let re = Regex::new(r"\(\)").unwrap();
    let result = re.replace_all(result.as_ref(), "null");
    let re = Regex::new(r"(usize|i32|u32|i64|u64|f32|f64)").unwrap();
    let result = re.replace_all(result.as_ref(), "number");
    let re = Regex::new(r"bool").unwrap();
    let result = re.replace_all(result.as_ref(), "boolean");
    let re = Regex::new(r"String").unwrap();
    let result = re.replace_all(result.as_ref(), "string");
    // 處理時間
    let re = Regex::new(r"DateTime< *Utc *>").unwrap();
    let result = re.replace_all(result.as_ref(), "string");
    // 處理 tuple
    let re = Regex::new(r"\(").unwrap();
    let result = re.replace_all(result.as_ref(), "[");
    let re = Regex::new(r"\)").unwrap();
    let result = re.replace_all(result.as_ref(), "]");
    // TODO: 其它基礎型別的轉換
    result.to_owned().to_string()
}
