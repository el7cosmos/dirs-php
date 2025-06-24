.DEFAULT_GOAL = target

target:
	cargo build $(CARGO_MODE_FLAGS)
	cp ./target/$(CARGO_MODE_DIR)/libdirs.$(SHLIB_SUFFIX_NAME) ./modules/dirs.$(SHLIB_DL_SUFFIX_NAME)

test: target

clean: cargo_clean

cargo_clean:
	cargo clean
