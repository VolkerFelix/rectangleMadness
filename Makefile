RUSTPROFILE := release
BIN_DIR := ./bin
PCK_NAME := rectangle_madness

.PHONY: all
all:
	@mkdir -p $(BIN_DIR)
	cargo build --$(RUSTPROFILE)
	@cp target/$(RUSTPROFILE)/$(PCK_NAME) $(BIN_DIR)/
	@rm -r target

.PHONY: clean
clean:
	cargo clean --target-dir ./target
	rm -f -r $(BIN_DIR)