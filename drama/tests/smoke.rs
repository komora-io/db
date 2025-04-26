use std::time::Duration;

use drama::Executor;
use komora_sync::{oneshot, ReceiveOne};

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

    let mut receivers = vec![];

    for _ in 0..128 {
        let recv = executor.execute(async {
            let timer = bad_timer(Duration::from_millis(100));

            timer.await.unwrap();

            1_usize
        });
        receivers.push(recv);
    }

    for _ in 0..128 {
        let recv = receivers.pop().unwrap();

        let res: usize = recv.recv().expect("executor thread died unexpectedly");

        assert_eq!(res, 1);
    }
}

#[test]
fn executor_test_02() {
    let executor = Executor::new(1);

    let mut receivers = vec![];

    for _ in 0..128 {
        let recv = executor.execute(async {
            let timer = bad_timer(Duration::from_millis(100));

            timer.await.unwrap();

            1_usize
        });
        receivers.push(recv);
    }

    for _ in 0..128 {
        let recv = receivers.pop().unwrap();

        let res: usize = recv.recv().expect("executor thread died unexpectedly");

        assert_eq!(res, 1);
    }
}
