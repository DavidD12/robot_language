#[cfg(test)]

mod tests_ok {

    use rl::model::Model;
    use rl::*;

    fn check_file_ok(file: &str) {
        let mut model = Model::empty();
        let result = process_file(&mut model, file);
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn type_1() {
        check_file_ok("examples/tests/parsing/type_1.rl");
    }
    #[test]
    fn type_2() {
        check_file_ok("examples/tests/parsing/type_2.rl");
    }
    #[test]
    fn type_3() {
        check_file_ok("examples/tests/parsing/type_3.rl");
    }
    #[test]
    fn skillset_1() {
        check_file_ok("examples/tests/parsing/skillset_1.rl");
    }
    #[test]
    fn data_1() {
        check_file_ok("examples/tests/parsing/data_1.rl");
    }
    #[test]
    fn data_2() {
        check_file_ok("examples/tests/parsing/data_2.rl");
    }
    #[test]
    fn data_3() {
        check_file_ok("examples/tests/parsing/data_3.rl");
    }
    #[test]
    fn resource_1() {
        check_file_ok("examples/tests/parsing/resource_1.rl");
    }
    #[test]
    fn resource_2() {
        check_file_ok("examples/tests/parsing/resource_2.rl");
    }
    #[test]
    fn event_1() {
        check_file_ok("examples/tests/parsing/event_1.rl");
    }
    #[test]
    fn skill_1() {
        check_file_ok("examples/tests/parsing/skill_1.rl");
    }
    #[test]
    fn precondition_1() {
        check_file_ok("examples/tests/parsing/precondition_1.rl");
    }
    #[test]
    fn precondition_2() {
        check_file_ok("examples/tests/parsing/precondition_2.rl");
    }
    #[test]
    fn input_output_1() {
        check_file_ok("examples/tests/parsing/input_output_1.rl");
    }
    #[test]
    fn start_1() {
        check_file_ok("examples/tests/parsing/start_1.rl");
    }
    #[test]
    fn start_2() {
        check_file_ok("examples/tests/parsing/start_2.rl");
    }
    #[test]
    fn invariant_1() {
        check_file_ok("examples/tests/parsing/invariant_1.rl");
    }
    #[test]
    fn invariant_2() {
        check_file_ok("examples/tests/parsing/invariant_2.rl");
    }
    #[test]
    fn interrupt_1() {
        check_file_ok("examples/tests/parsing/interrupt_1.rl");
    }
    #[test]
    fn terminate_1() {
        check_file_ok("examples/tests/parsing/terminate_1.rl");
    }
    #[test]
    fn progress_1() {
        check_file_ok("examples/tests/parsing/progress_1.rl");
    }
}

mod tests_err {

    use rl::model::Model;
    use rl::parser::RlError;
    use rl::*;

    fn check_file(file: &str) -> Result<Model, RlError> {
        let mut model = Model::empty();
        match process_file(&mut model, file) {
            Ok(_) => Ok(model),
            Err(e) => Err(e),
        }
    }

    #[test]
    fn err_type_1() {
        let result = check_file("examples/tests/parsing/err_type_1.rl");
        assert!(matches!(
            result,
            Err(RlError::Parse {
                message: _,
                position: _,
                expected: _
            })
        ));
    }
    #[test]
    fn err_type_2() {
        let result = check_file("examples/tests/parsing/err_type_2.rl");
        assert!(matches!(
            result,
            Err(RlError::Duplicate {
                name: _,
                first: _,
                second: _
            })
        ));
    }
    #[test]
    fn err_data_1() {
        let result = check_file("examples/tests/parsing/err_data_1.rl");
        assert!(matches!(
            result,
            Err(RlError::Duplicate {
                name: _,
                first: _,
                second: _
            })
        ));
    }
    #[test]
    fn err_data_2() {
        let result = check_file("examples/tests/parsing/err_data_2.rl");
        assert!(matches!(
            result,
            Err(RlError::Resolve {
                element: _,
                position: _
            })
        ));
    }
    #[test]
    fn err_resource_1() {
        let result = check_file("examples/tests/parsing/err_resource_1.rl");
        assert!(matches!(
            result,
            Err(RlError::Duplicate {
                name: _,
                first: _,
                second: _
            })
        ));
    }
    #[test]
    fn err_resource_2() {
        let result = check_file("examples/tests/parsing/err_resource_2.rl");
        assert!(matches!(
            result,
            Err(RlError::Duplicate {
                name: _,
                first: _,
                second: _
            })
        ));
    }
    #[test]
    fn err_resource_3() {
        let result = check_file("examples/tests/parsing/err_resource_3.rl");
        assert!(matches!(
            result,
            Err(RlError::Resolve {
                element: _,
                position: _
            })
        ));
    }
    #[test]
    fn err_resource_4() {
        let result = check_file("examples/tests/parsing/err_resource_4.rl");
        assert!(matches!(
            result,
            Err(RlError::Resolve {
                element: _,
                position: _
            })
        ));
    }
    #[test]
    fn err_resource_5() {
        let result = check_file("examples/tests/parsing/err_resource_5.rl");
        assert!(matches!(
            result,
            Err(RlError::Resolve {
                element: _,
                position: _
            })
        ));
    }
    #[test]
    fn err_event_1() {
        let result = check_file("examples/tests/parsing/err_event_1.rl");
        assert!(matches!(
            result,
            Err(RlError::Duplicate {
                name: _,
                first: _,
                second: _
            })
        ));
    }
}
