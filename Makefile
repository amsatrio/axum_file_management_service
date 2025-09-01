
.SILENT: run
.SILENT: build
.SILENT: start
.SILENT: build_release
.SILENT: test

ulimit: 
	ulimit -n 65536
	ulimit -a -v infinity
	ulimit -m unlimited 
	ulimit -c unlimited

# dev section
start: ulimit
	RUST_BACKTRACE=1 cargo run
build:
	cargo build &> logs/build.log
dev: # auto-reload
	RUST_BACKTRACE=1 cargo watch -q -c -w src/ -x run

clean:
	cargo clean
	rm -rf target

# release section
build_release:
	cargo build --release &> logs/build_release.log
build_release_start: build_release
	target/debug/axum_file_management_service
	

# test section
test:
	cargo test	

valgrind:
	valgrind --trace-children=yes --track-fds=yes --log-fd=2 --error-limit=no \
         --leak-check=full --show-possibly-lost=yes --track-origins=yes \
         --show-reachable=yes cargo run


# diesel
## setup
diesel_setup:
	diesel setup
## generate up down sql
diesel_generate:
	diesel migration generate m_user
## run migration
diesel_migration:
	diesel migration run
## print schema
diesel_schema:
	diesel print-schema > src/schema.rs