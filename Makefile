# You will probably be using 3.18 or similar if you aren't sure.

build-3.10:
	cargo build --features gtk_3_10

clippy-3.10:
	cargo clippy --features gtk_3_10

build-3.18:
	cargo build --features gtk_3_18

clippy-3.18:
	cargo clippy --features gtk_3_18

build-3.22:
	cargo build --features gtk_3_22

clippy-3.22:
	cargo clippy --features gtk_3_22
