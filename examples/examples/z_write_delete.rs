use zenoh::prelude::r#async::*;

#[async_std::main]
async fn main() {
    let session = zenoh::open(Config::default()).res().await.unwrap();
    let pub_ = session.declare_publisher("deleteme").res().await.unwrap();

    loop {
        eprintln!("Publisher::write(SampleKind::Delete, 42)");
        pub_.write(SampleKind::Delete, 42).res().await.unwrap();
    }
}
