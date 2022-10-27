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

    #[test]
    fn start_1() {
        check_file_ok("examples/tests/verif/start_1.rl");
    }

    #[test]
    fn invariant_1() {
        check_file_ok("examples/tests/verif/invariant_1.rl");
    }

    #[test]
    fn terminate_1() {
        check_file_ok("examples/tests/verif/terminate_1.rl");
    }

    #[test]
    fn postcondition_1() {
        check_file_ok("examples/tests/verif/postcondition_1.rl");
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
        assert!(v.len() >= 1);
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

    #[test]
    fn start_err_1() {
        let result = check_file_err("examples/tests/verif/start_err_1.rl");
        assert!(matches!(result, VError::SkillStartEffectCanFail(_, _)))
    }

    #[test]
    fn invariant_err_1() {
        let result = check_file_err("examples/tests/verif/invariant_err_1.rl");
        assert!(matches!(result, VError::SkillInvariantCantSucceed(_)));
    }
    #[test]
    fn invariant_err_2() {
        let result = check_file_err("examples/tests/verif/invariant_err_2.rl");
        assert!(matches!(result, VError::SkillInvariantCantFail(_)));
    }
    #[test]
    fn invariant_err_3() {
        let result = check_file_err("examples/tests/verif/invariant_err_3.rl");
        assert!(matches!(result, VError::SkillInvariantEffectCanFail(_, _)));
    }
    #[test]
    fn invariant_err_4() {
        let result = check_file_err("examples/tests/verif/invariant_err_4.rl");
        assert!(matches!(result, VError::SkillStartInvariantCanFail(_, _)));
    }

    #[test]
    fn terminate_err_1() {
        let result = check_file_err("examples/tests/verif/terminate_err_1.rl");
        assert!(matches!(result, VError::SkillInterruptEffectCanFail(_, _)));
    }
    #[test]
    fn terminate_err_2() {
        let result = check_file_err("examples/tests/verif/terminate_err_2.rl");
        assert!(matches!(result, VError::SkillSuccessEffectCanFail(_, _)));
    }
    #[test]
    fn terminate_err_3() {
        let result = check_file_err("examples/tests/verif/terminate_err_3.rl");
        assert!(matches!(result, VError::SkillFailureEffectCanFail(_, _)));
    }

    #[test]
    fn postcondition_err_1() {
        let result = check_file_err("examples/tests/verif/postcondition_err_1.rl");
        assert!(matches!(
            result,
            VError::SkillInterruptPostconditionCanFail(_, _)
        ));
    }
    #[test]
    fn postcondition_err_2() {
        let result = check_file_err("examples/tests/verif/postcondition_err_2.rl");
        assert!(matches!(
            result,
            VError::SkillSuccessPostconditionCanFail(_, _)
        ));
    }
    #[test]
    fn postcondition_err_3() {
        let result = check_file_err("examples/tests/verif/postcondition_err_3.rl");
        assert!(matches!(
            result,
            VError::SkillFailurePostconditionCanFail(_, _)
        ));
    }
}
