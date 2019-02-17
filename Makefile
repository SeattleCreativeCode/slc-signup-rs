all:
	cargo build --release
	cp ./target/release/slc-signup ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
