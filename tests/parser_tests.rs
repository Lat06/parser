use anyhow::Result;
use tasklist_parser::{TaskStatus, parse_tasks};

#[test]
fn test_parse_full_task() -> Result<()> {
    let input = "- [x] Complicated task (high) #rust #parser\n";
    let tasks = parse_tasks(input)?;

    assert_eq!(tasks.len(), 1);
    let task = &tasks[0];

    assert_eq!(task.status, TaskStatus::Done);
    assert_eq!(task.description, "Complicated task");
    assert_eq!(task.priority, Some("high".to_string()));
    assert_eq!(task.tags, vec!["rust".to_string(), "parser".to_string()]);

    Ok(())
}

#[test]
fn test_parse_task_no_priority_or_tags() -> Result<()> {
    let input = "- [ ] Simple task\n";
    let tasks = parse_tasks(input)?;

    assert_eq!(tasks.len(), 1);
    let task = &tasks[0];

    assert_eq!(task.status, TaskStatus::Pending);
    assert_eq!(task.description, "Simple task");
    assert_eq!(task.priority, None);
    assert!(task.tags.is_empty());

    Ok(())
}

#[test]
fn test_parse_only_priority() -> Result<()> {
    let input = "- [ ] Task with priority (low)\n";
    let tasks = parse_tasks(input)?;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].priority, Some("low".to_string()));
    assert!(tasks[0].tags.is_empty());
    Ok(())
}

#[test]
fn test_parse_only_tags() -> Result<()> {
    let input = "- [ ] Task with tags #work\n";
    let tasks = parse_tasks(input)?;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].priority, None);
    assert_eq!(tasks[0].tags, vec!["work".to_string()]);
    Ok(())
}

#[test]
fn test_invalid_line_fails() {
    let input = "[ ] Invalid task\n";
    let result = parse_tasks(input);
    assert!(result.is_err());
}
