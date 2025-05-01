use std::time::Duration;

use komora_sync::{oneshot, ReceiveOne};

use drama::{Classification, Executor, TenantId};

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
    let tenant_id = TenantId::new(0);
    let classification = Classification::Compute;

    let recv = executor.execute(tenant_id, classification, async { 1_usize });

    let res: usize = recv.recv().expect("executor thread died unexpectedly");

    assert_eq!(res, 1);
}

#[test]
fn executor_test_01() {
    let executor = Executor::new(1);
    let tenant_id = TenantId::new(0);
    let classification = Classification::Compute;

    let mut receivers = vec![];

    for _ in 0..128 {
        let recv = executor.execute(tenant_id, classification, async {
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
    let classification = Classification::Compute;

    let mut receivers = vec![];

    for i in 0..128 {
        let tenant_id = TenantId::new(i % 8);
        let recv = executor.execute(tenant_id, classification, async {
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
