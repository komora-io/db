use std::time::Duration;

use super::Executor;
use crate::sync::{oneshot, ReceiveOne};

fn bad_timer(duration: Duration) -> ReceiveOne<()> {
    let (tx, rx) = oneshot();

    std::thread::spawn(move || {
        std::thread::sleep(duration);
        tx.send(());
    });

    rx
}

#[test]
fn executor_test_00() {
    let executor = Executor::new(1);

    let recv = executor.execute(async { 1_usize });

    let res: usize = recv.recv().expect("executor thread died unexpectedly");

    assert_eq!(res, 1);
}

#[test]
fn executor_test_01() {
    let executor = Executor::new(1);

    let recv = executor.execute(async {
        let timer = bad_timer(Duration::from_millis(100));

        timer.await.unwrap();

        1_usize
    });

    let res: usize = recv.recv().expect("executor thread died unexpectedly");

    assert_eq!(res, 1);
}
