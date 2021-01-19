fn main() {
    let _guard = sentry::init(("http://ca2e9cb6ec93494686092721d95c5b70@localhost:9000/3", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    test();

    // Sentry will capture this
    panic!("李卿SB!");
}

fn test() {
    panic!("李卿SBBBBBB!");
}