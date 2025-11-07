use colored::*;
use serde_json::Value;
use std::error::Error;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ServError {
    Exception {
        code: String,
        name: String,
        data: Option<Value>,
    },
    Io(std::io::Error),
}

#[allow(dead_code)]
impl ServError {
    pub fn missing_input(data: Option<Value>) -> Self {
        ServError::Exception {
            code: "E972".into(),
            name: "MissingInput".into(),
            data,
        }
    }

    pub fn invalid_input(data: Option<Value>) -> Self {
        ServError::Exception {
            code: "E111".into(),
            name: "InvalidInput".into(),
            data,
        }
    }
}

impl fmt::Display for ServError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServError::Exception { code, name, data } => {
                let code_colored: ColoredString = code.red().bold();
                let name_colored: ColoredString = name.yellow().bold();

                if let Some(d) = data {
                    let issue = d
                        .get("issue")
                        .and_then(|v| v.as_str())
                        .unwrap_or("<no issue field>");

                    write!(f, "[{}] {}: {}", code_colored, name_colored, issue.cyan())
                } else {
                    write!(f, "[{}] {}", code_colored, name_colored)
                }
            }
            ServError::Io(err) => write!(f, "{}", err.to_string().red()),
        }
    }
}

impl Error for ServError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServError::Io(e) => Some(e),
            _ => None,
        }
    }
}
