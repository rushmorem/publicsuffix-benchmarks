use addr::parser::DomainName;
use psl::List;
use std::str;
use std::time::Instant;

fn run(domain: &str, expected: &str) {
    let domain = List.parse_domain_name(domain).unwrap();
    assert_eq!(domain.root().unwrap(), expected);
}

fn main() {
    {
        let iterations = 1000;
        let now = Instant::now();

        for _i in 0..iterations {
            run("abc.www.example.com", "example.com");
            run("abc.golang.org", "golang.org");
            run("www.食狮.中国", "食狮.中国");
            run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn");
            run("a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }

    {
        let iterations = 10_000;
        let now = Instant::now();

        for _i in 0..iterations {
            run("abc.www.example.com", "example.com");
            run("abc.golang.org", "golang.org");
            run("www.食狮.中国", "食狮.中国");
            run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn");
            run("a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }

    {
        let iterations = 100_000;
        let now = Instant::now();

        for _i in 0..iterations {
            run("abc.www.example.com", "example.com");
            run("abc.golang.org", "golang.org");
            run("www.食狮.中国", "食狮.中国");
            run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn");
            run("a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }

    {
        let iterations = 10_000_000;
        let now = Instant::now();

        for _i in 0..iterations {
            run("abc.www.example.com", "example.com");
            run("abc.golang.org", "golang.org");
            run("www.食狮.中国", "食狮.中国");
            run("www.xn--85x722f.xn--55qx5d.cn", "xn--85x722f.xn--55qx5d.cn");
            run("a.b.example.uk.com", "example.uk.com");
        }

        let elapsed = now.elapsed();

        println!(
            "{:8} iterations = {:3}.{:03}s",
            iterations,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    }
}
