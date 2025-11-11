use pest::Parser;
use pest_derive::Parser;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse file: {0}")]
    PestError(#[from] Box<pest::error::Error<Rule>>),

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
    pub priority: Option<String>,
    pub tags: Vec<String>,
}

pub fn parse_tasks(content: &str) -> Result<Vec<Task>, ParseError> {
    let pairs = TaskParser::parse(Rule::file, content).map_err(Box::new)?;
    let mut tasks = Vec::new();

    for pair in pairs {
        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::task_line {
                let mut status: Option<TaskStatus> = None;
                let mut description: Option<String> = None;
                let mut priority: Option<String> = None;
                let mut tags: Vec<String> = Vec::new();

                for part in inner.into_inner() {
                    match part.as_rule() {
                        Rule::status => {
                            status = Some(match part.as_str() {
                                "x" => TaskStatus::Done,
                                " " => TaskStatus::Pending,
                                _ => unreachable!(),
                            });
                        }
                        Rule::description_text => {
                            description = Some(part.as_str().trim().to_string());
                        }
                        Rule::priority => {
                            priority = Some(part.into_inner().next().unwrap().as_str().to_string());
                        }
                        Rule::tag => {
                            tags.push(part.into_inner().next().unwrap().as_str().to_string());
                        }
                        _ => {}
                    }
                }

                if let (Some(status), Some(description)) = (status, description) {
                    tasks.push(Task {
                        status,
                        description,
                        priority,
                        tags,
                    });
                }
            }
        }
    }

    Ok(tasks)
}
