run:
	@echo "\n Testing https://godoc.org/golang.org/x/net/publicsuffix"
	export GOPATH=`pwd` && cd src/go-publicsuffix/ && go get . && go run .

	@echo "\n Testing https://docs.rs/addr/0.10.2/addr/ (psl)"
	cd src/rust-addr-psl/ && cargo run -q --release

	@echo "\n Testing https://docs.rs/addr/0.10.2/addr/ (publicsuffix/anycase)"
	cd src/rust-addr-publicsuffix-anycase/ && cargo run -q --release

	@echo "\n Testing https://docs.rs/publicsuffix/2.0.2/publicsuffix/ (anycase)"
	cd src/rust-publicsuffix-anycase/ && cargo run -q --release

	@echo "\n Testing https://docs.rs/publicsuffix/2.0.2/publicsuffix/"
	cd src/rust-publicsuffix/ && cargo run -q --release

	@echo "\n Testing https://docs.rs/psl/2.0.3/psl/"
	cd src/rust-psl/ && cargo run -q --release
