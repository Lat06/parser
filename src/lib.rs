use pest::Parser;
use pest_derive::Parser;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse file: {0}")]
    PestError(#[from] pest::error::Error<Rule>),

    #[error("Invalid task line found: {0}")]
    InvalidLine(String),
}

#[derive(Parser)]
#[grammar = "tasks.pest"]
pub struct TaskParser;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Done,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    pub status: TaskStatus,
    pub description: String,
}

pub fn parse_tasks(content: &str) -> Result<Vec<Task>, ParseError> {
    let pairs = TaskParser::parse(Rule::file, content)?;
    let mut tasks = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::task_line {
                let mut inner_pairs = inner.into_inner();

                let status_pair = inner_pairs.next().unwrap();
                let description_pair = inner_pairs.next().unwrap();

                let status = match status_pair.as_str() {
                    "x" => TaskStatus::Done,
                    " " => TaskStatus::Pending,
                    _ => TaskStatus::Pending,
                };

                let description = description_pair.as_str().trim().to_string();

                tasks.push(Task {
                    status,
                    description,
                });
            }
        }
    }

    Ok(tasks)
}
