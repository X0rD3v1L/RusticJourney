fn convert_to_snake_case(getter_names: Vec<String>) -> Vec<String> {
    let mut ans = Vec::new();

    for string in getter_names {
        if string.starts_with("get") {
            let trimmed = &string[3..];
            let mut result = String::new();
            let chars: Vec<char> = trimmed.chars().collect();
            let len = chars.len();

            let mut i = 0;
            while i < len {
                let c = chars[i];

                // Start of new word
                if c.is_uppercase() {
                    if i != 0 {
                        result.push('_');
                    }

                    // Check if this is an acronym (multiple uppercase letters)
                    let start = i;
                    i += 1;
                    while i < len && chars[i].is_uppercase() {
                        i += 1;
                    }

                    // If acronym is followed by lowercase, last capital belongs to next word
                    let end = if i < len && chars[i].is_lowercase() && i - start > 1 {
                        i - 1
                    } else {
                        i
                    };

                    for j in start..end {
                        result.push(chars[j].to_ascii_lowercase());
                    }

                    // Let outer loop continue from current `i`
                    if end < i {
                        i = end;
                    }
                } else {
                    result.push(c);
                    i += 1;
                }
            }

            ans.push(result);
        } else {
            ans.push(string);
        }
    }

    ans
}

fn main() {
    let getter_names = vec![
        "getCurrency".to_string(),
        "getAccountName".to_string(),
        "getLongAccountName".to_string(),
        "getTradeID".to_string(),
        "getSWIFTCode".to_string()
    ];

    //["currency", "account_name", "long_account_name", "trade_id", "swift_code"]
    println!("{:?}", convert_to_snake_case(getter_names));
}
