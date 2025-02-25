// common contains open_tmp macro
mod common;

#[test]
fn transactor_00() {
    let db = open_tmp!();

    {
        let mut tx = db.tx();
        tx.insert(b"a", b"a");
        tx.commit().unwrap();
    }

    {
        let mut tx = db.tx();
        let read = tx.get(b"a").unwrap().unwrap();
        assert_eq!(read, b"a");
    }
}
