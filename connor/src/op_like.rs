fn like(condition: &str, data: &str) -> bool {
    let mut has_prefix = false;
    let mut has_suffix = false;
    let mut cn = condition.to_string();

    if cn.starts_with('%') {
        has_prefix = true;
        cn.remove(0);
    }
    if cn.ends_with('%') {
        has_suffix = true;
        cn.pop();
    }

    match (has_prefix, has_suffix) {
        (true, true) => data.contains(&cn),
        (true, false) => data.ends_with(&cn),
        (false, true) => data.starts_with(&cn),
        (false, false) => {
            let start_and_end: Vec<&str> = cn.split('%').collect();
            if start_and_end.len() == 2 {
                data.starts_with(start_and_end[0]) && data.ends_with(start_and_end[1])
            } else {
                data == cn
            }
        }
    }
}

pub fn handle(condition: &core::DocField, doc_field: &core::DocField) -> bool {
    match (condition, doc_field) {
        (core::DocField::String(str_cond), core::DocField::String(str_val)) => {
            like(str_cond, str_val)
        }
        _ => false,
    }
}
