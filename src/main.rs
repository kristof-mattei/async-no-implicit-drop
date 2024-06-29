#[tokio::main]
async fn main() {
    println!("Start: async_test");
    async_test().await;
    println!("End: async_test");
    println!();
    println!();
    println!("Start: non_async_test");
    non_async_test();
    println!("End: non_async_test");
}

#[allow(clippy::unused_async)]
async fn async_test() {
    {
        let drop_test = DropTest::new("async_test".into());

        drop_test.do_something();
    }
}

fn non_async_test() {
    {
        let drop_test = DropTest::new("non_async_test".into());

        drop_test.do_something();
    }
}

struct DropTest {
    from: String,
}

impl DropTest {
    fn new(from: String) -> Self {
        Self { from }
    }

    #[allow(clippy::unused_self)]
    fn do_something(&self) {
        println!("From: {}", self.from);
    }
}

impl Drop for DropTest {
    fn drop(&mut self) {
        println!("Dropped: {}", self.from);
    }
}
