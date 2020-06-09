use crate::Request;
pub fn gen_enum_json(prev: &[&'static str], params: &[Request]) -> String {
    if let Some(cur) = prev.last() {
        let inner = gen_enum_json(&prev[0..prev.len() - 1], params);
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
