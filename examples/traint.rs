

fn use_trait_in_struct() {
    trait UrlProvider {
        fn get_url(&self) -> String;
    }

    struct Config {
        name: String,
        url_provider: Box<dyn UrlProvider>
    }

    fn run (config: &Config) {
        println!("name={}, url={}", config.name, config.url_provider.get_url())
    }

    struct MyUrlProvider;

    impl UrlProvider for MyUrlProvider {
        fn get_url(&self) -> String {
            "http://example.com".to_string()
        }
    }

    let config = Config {
        name: "hello".into(),
        url_provider: Box::new(MyUrlProvider),
    };

    run(&config);
    run(&config);
}

fn use_fn_in_struct() {

    struct Config {
        name: String,
        url_provider: Box<dyn Fn() -> String>
    }

    fn run (config: &Config) {
        println!("name={}, url={}", config.name, (config.url_provider)())
    }


    let config = Config {
        name: "hello".into(),
        url_provider: Box::new(|| "my_url".to_string()),
    };

    run(&config);
    run(&config);
}

fn main() {
    use_trait_in_struct();
    use_fn_in_struct();
}
