#[cfg(test)]
mod tests {
    use todo_rs::cmd::*;

    #[test]

    fn test_add_target() {
        for i in 1..10 {
            let result = command_add(&[
                "add",
                &format!("test_task_{}", i),
                "2025-8-25",
                "A test task",
                "TestWork",
            ]);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_del_target() {
        for i in 1..10 {
            let result = command_del(&["del", &format!("{}", i)]);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_edit_target() {
        for i in 1..10 {
            let result = command_edit(&["edit", &format!("{}", i), "description", "test_edit"]);
            assert!(result.is_ok());
        }
    }
}
