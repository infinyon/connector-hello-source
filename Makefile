
default: test

test-lib:
	cargo test -p ext-lib

build:
	cd crates/connector-main && cdk build

deploy: build
	cd crates/connector-main && cdk deploy start --config config.yaml

shutdown: build
	cd crates/connector-main && cdk deploy shutdown --config config.yaml

# test connector without optional secret parameter
test:
	cd crates/connector-main && cdk test --config config.yaml

# test connector with optional secret parameter
test-secret:
	cd crates/connector-main && cdk test --config config-with-secret.yaml -s secret.txt
