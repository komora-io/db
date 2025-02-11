use db::{open, Config, Db};

#[test]
fn smoke_00() {
    let db = open("smoke_00").unwrap();

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
