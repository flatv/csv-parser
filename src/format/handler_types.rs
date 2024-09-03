trait ValueHandler {
    fn handle(&self, value: &str) -> Option<String>;
}

struct NullHandler;
impl ValueHandler for NullHandler {
    fn handle(&self, value: &str) -> Option<String> {
        match value.eq_ignore_ascii_case("NULL") {
            true => Some("NULL".to_string()),
            false => None,
        }
    }
}

struct IntegerHandler;
impl ValueHandler for IntegerHandler {
    fn handle(&self, value: &str) -> Option<String> {
        match value.parse::<i64>() {
            Ok(_) => Some(value.to_string()),
            Err(_) => None,
        }
    }
}

struct FloatHandler;
impl ValueHandler for FloatHandler {
    fn handle(&self, value: &str) -> Option<String> {
        match value.parse::<f64>() {
            Ok(_) => Some(value.to_string()),
            Err(_) => None,
        }
    }
}

struct BooleanHandler;
impl ValueHandler for BooleanHandler {
    fn handle(&self, value: &str) -> Option<String> {
        match value.to_lowercase().as_str() {
            "true" | "false" => Some(value.to_string()),
            _ => None,
        }
    }
}

struct StringHandler;
impl ValueHandler for StringHandler {
    fn handle(&self, value: &str) -> Option<String> {
        let cleaned_value = value
            .replace("'", "")
            .replace("\u{00A0}", " ")
            .replace("\"", "");
        Some(format!("'{}'", cleaned_value))
    }
}

pub struct ChainHandler {
    handlers: Vec<Box<dyn ValueHandler>>,
}

impl ChainHandler {
    pub fn new() -> Self {
        let mut handlers: Vec<Box<dyn ValueHandler>> = Vec::new();

        handlers.push(Box::new(NullHandler));
        handlers.push(Box::new(IntegerHandler));
        handlers.push(Box::new(FloatHandler));
        handlers.push(Box::new(BooleanHandler));
        handlers.push(Box::new(StringHandler)); // важно что бы он был последним, хрень конечно но увы

        ChainHandler { handlers }
    }

    pub fn handle(&self, value: &str) -> String {
        for handler in &self.handlers {
            if let Some(result) = handler.handle(value) {
                return result;
            }
        }
        "NULL".to_string()
    }
}
