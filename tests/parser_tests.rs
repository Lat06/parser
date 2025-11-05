use anyhow::Result;
use tasklist_parser::{Task, TaskStatus, parse_tasks};

#[test]
fn test_parse_single_done_task() -> Result<()> {
    let input = "- [x] This is a completed task\n";
    let tasks = parse_tasks(input)?;

    assert_eq!(tasks.len(), 1);
    assert_eq!(
        tasks[0],
        Task {
            status: TaskStatus::Done,
            description: "This is a completed task".to_string()
        }
    );
    Ok(())
}

#[test]
fn test_parse_single_pending_task() -> Result<()> {
    let input = "- [ ] This is a pending task\n";
    let tasks = parse_tasks(input)?;

    assert_eq!(tasks.len(), 1);
    assert_eq!(
        tasks[0],
        Task {
            status: TaskStatus::Pending,
            description: "This is a pending task".to_string()
        }
    );
    Ok(())
}

#[test]
fn test_parse_multiple_tasks_and_empty_lines() -> Result<()> {
    let input = "
- [x] Task 1

- [ ] Task 2
- [x] Task 3

";
    let tasks = parse_tasks(input)?;

    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].status, TaskStatus::Done);
    assert_eq!(tasks[1].status, TaskStatus::Pending);
    assert_eq!(tasks[2].status, TaskStatus::Done);
    Ok(())
}

#[test]
fn test_invalid_line_fails() {
    let input = "[ ] Invalid task\n";

    let result = parse_tasks(input);
    assert!(result.is_err());
}
