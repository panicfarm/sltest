use rand::Rng;
use sled::*;

fn main() {
    let config = sled::Config::default()
        .cache_capacity(256)
        .path("/tmp/sltest.0");
    println!("{:?}", config);
    let db = config.open().unwrap();
    if !db.is_empty() {
        panic!("Empty DB!!!")
    }
    let mut key;
    let mut val;
    let mut n = 0;
    loop {
        key = rand::thread_rng().gen::<[u8; 32]>();
        val = rand::thread_rng().gen::<[u8; 32]>();
        db.insert(key.to_vec(), val.to_vec());
        n += 1;
        println!("{}", n);
    }
}
