use super::Executor;

#[test]
fn executor_test_00() {
    let executor = Executor::new(1);

    let recv = executor.execute(async { 1_usize });
    let res: usize = recv.recv().expect("executor thread died unexpectedly");

    assert_eq!(res, 1);
}
