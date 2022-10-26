mod tests_ok {

    use rl::model::Model;
    use rl::verif::check_model;
    use rl::*;

    fn check_file_ok(file: &str) {
        let mut model = Model::empty();
        let res = process_file(&mut model, file);
        assert!(matches!(res, Ok(_)));
        let errors = check_model(&model);
        assert_eq!(errors, vec![]);
    }

    #[test]
    fn event_1() {
        check_file_ok("examples/tests/verif/event_1.rl");
    }

    #[test]
    fn precondition_1() {
        check_file_ok("examples/tests/verif/precondition_1.rl");
    }
}

mod tests_err {
    use rl::model::Model;
    use rl::verif::*;
    use rl::*;

    fn check_file_err(file: &str) -> VError {
        let mut model = Model::empty();
        let res = process_file(&mut model, file);
        assert!(matches!(res, Ok(_)));
        let v = check_model(&model);
        assert!(v.len() == 1);
        v.first().unwrap().clone()
    }

    #[test]
    fn event_err_1() {
        let result = check_file_err("examples/tests/verif/event_err_1.rl");
        assert!(matches!(result, VError::EventGuardCantSucceed(_)));
    }
    #[test]
    fn event_err_2() {
        let result = check_file_err("examples/tests/verif/event_err_2.rl");
        assert!(matches!(result, VError::EventGuardCantFail(_)));
    }
    #[test]
    fn event_err_3() {
        let result = check_file_err("examples/tests/verif/event_err_3.rl");
        assert!(matches!(result, VError::EventEffectCanFail(_, _)));
    }

    #[test]
    fn precondition_err_1() {
        let result = check_file_err("examples/tests/verif/precondition_err_1.rl");
        assert!(matches!(result, VError::SkillPreconditionCantSucceed(_)));
    }
    #[test]
    fn precondition_err_2() {
        let result = check_file_err("examples/tests/verif/precondition_err_2.rl");
        assert!(matches!(result, VError::SkillPreconditionCantFail(_)));
    }
}
