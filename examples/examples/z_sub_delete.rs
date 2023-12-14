use zenoh::prelude::sync::*;

#[async_std::main]
async fn main() {
    let session = zenoh::open(Config::default()).res().unwrap();
    let sub = session.declare_subscriber("deleteme").res().unwrap();
    loop {
        let sample = sub.recv().unwrap();
        eprintln!("Got sample of kind: {:?}", sample.kind);
    }
}
