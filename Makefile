.PHONY: test test-doc cov cov-html

# Run all integration tests across all feature combinations
test:
	cargo test -p tidos --tests
	cargo test -p tidos --features "i18n,rocket,axum,actix-web,warp" --test i18n

# Run doc tests separately (pre-existing failures may exist)
test-doc:
	cargo test -p tidos --doc

# Generate lcov.info (e.g. for IDE coverage gutters)
cov:
	cargo llvm-cov --no-clean --workspace --tests
	cargo llvm-cov --no-clean --features "i18n,rocket,axum,actix-web,warp" --test i18n
	cargo llvm-cov report --lcov --output-path lcov.info

# Generate HTML coverage report and open it in the browser
cov-html:
	cargo llvm-cov --no-clean --workspace --tests
	cargo llvm-cov --no-clean --features "i18n,rocket,axum,actix-web,warp" --test i18n
	cargo llvm-cov report --html --output-dir target/llvm-cov
	@echo "Report: target/llvm-cov/html/index.html"
