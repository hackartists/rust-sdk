use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ActionRequestField {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, PartialEq)]
pub struct ActionRequest {
    pub name: String,
    pub fields: Vec<ActionRequestField>,
}

/// The Actions struct holds three groups of action requests:
/// - `actions`: for the "action" key
/// - `read_actions`: for the "read_action" key
/// - `action_by_id`: for the "action_by_id" key
#[derive(Debug, PartialEq)]
pub struct Actions {
    pub actions: Vec<ActionRequest>,
    pub read_actions: Vec<ActionRequest>,
    pub action_by_id: Vec<ActionRequest>,
}

/// Implement FromStr for Actions to parse an annotation string.
impl FromStr for Actions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse top-level key=value pairs.
        let pairs = parse_key_value_pairs(s)?;

        let mut actions: Vec<ActionRequest> = Vec::new();
        let mut read_actions: Vec<ActionRequest> = Vec::new();
        let mut action_by_id: Vec<ActionRequest> = Vec::new();

        for (key, value) in pairs {
            let key_lower = key.trim().to_lowercase();
            if key_lower == "action" {
                actions = parse_actions(&value)?;
            } else if key_lower == "read_action" {
                read_actions = parse_actions(&value)?;
            } else if key_lower == "action_by_id" {
                action_by_id = parse_actions(&value)?;
            }
        }

        Ok(Actions {
            actions,
            read_actions,
            action_by_id,
        })
    }
}

/// --- Helper Functions ---

/// Parses the top-level annotation string into key–value pairs,
/// accounting for quotes, parentheses, and square brackets.
fn parse_key_value_pairs(s: &str) -> Result<Vec<(String, String)>, String> {
    let mut pairs = Vec::new();
    let mut token = String::new();
    let mut paren_depth = 0;
    let mut bracket_depth = 0; // Track square brackets.
    let mut in_quote = false;

    for c in s.chars() {
        match c {
            '"' => {
                in_quote = !in_quote;
                token.push(c);
            }
            '(' if !in_quote => {
                paren_depth += 1;
                token.push(c);
            }
            ')' if !in_quote => {
                if paren_depth == 0 {
                    return Err("Unmatched ')' in annotation".to_string());
                }
                paren_depth -= 1;
                token.push(c);
            }
            '[' if !in_quote => {
                bracket_depth += 1;
                token.push(c);
            }
            ']' if !in_quote => {
                if bracket_depth == 0 {
                    return Err("Unmatched ']' in annotation".to_string());
                }
                bracket_depth -= 1;
                token.push(c);
            }
            // Split on comma only when not inside quotes, parentheses, or brackets.
            ',' if !in_quote && paren_depth == 0 && bracket_depth == 0 => {
                if !token.trim().is_empty() {
                    let (k, v) = split_key_value(&token)?;
                    pairs.push((k, v));
                }
                token.clear();
            }
            other => {
                token.push(other);
            }
        }
    }
    if !token.trim().is_empty() {
        let (k, v) = split_key_value(&token)?;
        pairs.push((k, v));
    }
    Ok(pairs)
}

/// Splits a token of the form "key = value" into its key and value components.
fn split_key_value(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid key=value pair: '{}'", s));
    }
    let key = parts[0].trim().to_string();
    let value = parts[1].trim().to_string();
    Ok((key, value))
}

/// Parses the value for an action key into a vector of ActionRequest.
/// The value may or may not be wrapped in square brackets.
fn parse_actions(s: &str) -> Result<Vec<ActionRequest>, String> {
    // Remove surrounding square brackets if present.
    let s = s.trim();
    let s = if s.starts_with('[') && s.ends_with(']') {
        &s[1..s.len() - 1]
    } else {
        s
    };

    // Split the action value into individual action tokens.
    let mut actions = Vec::new();
    let mut token = String::new();
    let mut paren_depth = 0;
    let mut angle_depth = 0;
    for c in s.chars() {
        match c {
            '(' => {
                paren_depth += 1;
                token.push(c);
            }
            ')' => {
                if paren_depth == 0 {
                    return Err("Unmatched ')' in action value".to_string());
                }
                paren_depth -= 1;
                token.push(c);
            }
            '<' => {
                angle_depth += 1;
                token.push(c);
            }
            '>' => {
                if angle_depth == 0 {
                    return Err("Unmatched '>' in action value".to_string());
                }
                angle_depth -= 1;
                token.push(c);
            }
            // At top level, comma separates different actions.
            ',' if paren_depth == 0 && angle_depth == 0 => {
                if !token.trim().is_empty() {
                    actions.push(token.trim().to_string());
                }
                token.clear();
            }
            other => {
                token.push(other);
            }
        }
    }
    if !token.trim().is_empty() {
        actions.push(token.trim().to_string());
    }

    // Parse each action token into an ActionRequest.
    let mut reqs = Vec::new();
    for act in actions {
        if let Some(paren_start) = act.find('(') {
            let paren_end = act
                .rfind(')')
                .ok_or_else(|| format!("Missing closing parenthesis in '{}'", act))?;
            let name = act[..paren_start].trim().to_string();
            let args_str = &act[paren_start + 1..paren_end];
            let fields = parse_fields(args_str)?;
            reqs.push(ActionRequest { name, fields });
        } else {
            let name = act.trim().to_string();
            reqs.push(ActionRequest {
                name,
                fields: vec![],
            });
        }
    }

    Ok(reqs)
}

/// Parses the fields inside an action’s parentheses into a vector of ActionRequestField.
/// Fields should be in the format: `name = Type`, separated by commas.
fn parse_fields(s: &str) -> Result<Vec<ActionRequestField>, String> {
    let mut fields = Vec::new();
    let mut token = String::new();
    let mut angle_depth = 0;
    for c in s.chars() {
        match c {
            '<' => {
                angle_depth += 1;
                token.push(c);
            }
            '>' => {
                if angle_depth == 0 {
                    return Err("Unmatched '>' in field definition".to_string());
                }
                angle_depth -= 1;
                token.push(c);
            }
            // Split on commas at top level (not inside angle brackets).
            ',' if angle_depth == 0 => {
                if !token.trim().is_empty() {
                    let field = parse_field(&token)?;
                    fields.push(field);
                }
                token.clear();
            }
            other => {
                token.push(other);
            }
        }
    }
    if !token.trim().is_empty() {
        let field = parse_field(&token)?;
        fields.push(field);
    }
    Ok(fields)
}

/// Parses a single field of the form "name = Type" into an ActionRequestField.
fn parse_field(s: &str) -> Result<ActionRequestField, String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid field format: '{}'", s));
    }
    let name = parts[0].trim().to_string();
    let ty = parts[1].trim().to_string();
    Ok(ActionRequestField { name, r#type: ty })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_actions_struct() {
        // The complete annotation string with all three action keys.
        let input = r#"
            base = "/auth/v1", action = [signup(code = String), login],
            read_action = read_data, action_by_id = [get_data, update_data],
            table = users, iter_type=QueryResponse
        "#;

        // Parse the string into an Actions struct.
        let parsed = input.parse::<Actions>().expect("Parsing failed");

        // --- Test for "action" ---
        // Expect two actions: "signup" with one field and "login" without parameters.
        assert_eq!(parsed.actions.len(), 2);
        let signup = &parsed.actions[0];
        assert_eq!(signup.name, "signup");
        assert_eq!(signup.fields.len(), 1);
        assert_eq!(signup.fields[0].name, "code");
        assert_eq!(signup.fields[0].r#type, "String");

        let login = &parsed.actions[1];
        assert_eq!(login.name, "login");
        assert!(login.fields.is_empty());

        // --- Test for "read_action" ---
        // Expect one read action: "read_data" with no parameters.
        assert_eq!(parsed.read_actions.len(), 1);
        let read_data = &parsed.read_actions[0];
        assert_eq!(read_data.name, "read_data");
        assert!(read_data.fields.is_empty());

        // --- Test for "action_by_id" ---
        // Expect two actions: "get_data" and "update_data", both without parameters.
        assert_eq!(parsed.action_by_id.len(), 2);
        let get_data = &parsed.action_by_id[0];
        assert_eq!(get_data.name, "get_data");
        assert!(get_data.fields.is_empty());

        let update_data = &parsed.action_by_id[1];
        assert_eq!(update_data.name, "update_data");
        assert!(update_data.fields.is_empty());
    }

    #[test]
    fn test_parse_action_annotation() {
        let input = r#"
            base = "/auth/v1", action = signup(code = String), table = users, iter_type=QueryResponse
        "#;
        let parsed = input
            .parse::<Actions>()
            .expect("Parsing failed for action annotation");

        // "action" key should yield one action: "signup" with one field.
        assert_eq!(parsed.actions.len(), 1);
        let signup = &parsed.actions[0];
        assert_eq!(signup.name, "signup");
        assert_eq!(signup.fields.len(), 1);
        assert_eq!(signup.fields[0].name, "code");
        assert_eq!(signup.fields[0].r#type, "String");

        // Other vectors should be empty.
        assert!(parsed.read_actions.is_empty());
        assert!(parsed.action_by_id.is_empty());
    }

    /// Test case for an annotation with the "read_action" key.
    #[test]
    fn test_parse_read_action_annotation() {
        let input = r#"
            base = "/auth/v1", read_action = get_user(id = i32), table = users, iter_type=QueryResponse
        "#;
        let parsed = input
            .parse::<Actions>()
            .expect("Parsing failed for read_action annotation");

        // "read_action" key should yield one action: "get_user" with one field.
        assert!(parsed.actions.is_empty());
        assert_eq!(parsed.read_actions.len(), 1);
        let get_user = &parsed.read_actions[0];
        assert_eq!(get_user.name, "get_user");
        assert_eq!(get_user.fields.len(), 1);
        assert_eq!(get_user.fields[0].name, "id");
        assert_eq!(get_user.fields[0].r#type, "i32");

        // action_by_id vector should be empty.
        assert!(parsed.action_by_id.is_empty());
    }

    /// Test case for an annotation with the "action_by_id" key.
    #[test]
    fn test_parse_action_by_id_annotation() {
        let input = r#"
            base = "/auth/v1", action_by_id = [get_data, update_data(code = i32)], table = users, iter_type=QueryResponse
        "#;
        let parsed = input
            .parse::<Actions>()
            .expect("Parsing failed for action_by_id annotation");

        // "action_by_id" key should yield two actions.
        assert!(parsed.actions.is_empty());
        assert!(parsed.read_actions.is_empty());
        assert_eq!(parsed.action_by_id.len(), 2);

        // First action: "get_data" with no parameters.
        let get_data = &parsed.action_by_id[0];
        assert_eq!(get_data.name, "get_data");
        assert!(get_data.fields.is_empty());

        // Second action: "update_data" with one parameter.
        let update_data = &parsed.action_by_id[1];
        assert_eq!(update_data.name, "update_data");
        assert_eq!(update_data.fields.len(), 1);
        assert_eq!(update_data.fields[0].name, "code");
        assert_eq!(update_data.fields[0].r#type, "i32");
    }
}
