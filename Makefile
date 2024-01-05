
default: test

test-lib:
	cargo test -p ext-lib

build:
	cd crates/connector-main && cdk build

deploy: build
	cd crates/connector-main && cdk deploy start --config config.yaml

shutdown: build
	cd crates/connector-main && cdk deploy shutdown --config config.yaml

test:
	cd crates/connector-main && cdk test --config config.yaml
