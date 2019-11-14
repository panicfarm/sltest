use rand::Rng;
use tempfile::tempdir;

fn main() {
    let dir = tempdir().unwrap();

    let config = sled::Config::default()
        .cache_capacity(256)
        .snapshot_after_ops(1_000_000_000) //never
        .path(dir.path());
    println!("{:#?}", config);
    let db = config.open().unwrap();
    for n in 0.. {
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let val = rand::thread_rng().gen::<[u8; 32]>();
        db.insert(key.to_vec(), val.to_vec());
        println!("{}", n);
    }
}
