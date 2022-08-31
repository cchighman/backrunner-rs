ROOT_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))

JULIA ?= julia
DLEXT := $(shell $(JULIA) --startup-file=no -e 'using Libdl; print(Libdl.dlext)')

CARGO_TARGET := target
MAIN := backrunner-rs

PREFIX := $(ROOT_DIR)/target
LIBDIR := $(PREFIX)/lib
BINDIR := $(PREFIX)/bin

CFMMROUTER := $(LIBDIR)/cfmmrouter.$(DLEXT)
SUBDIRS := CFMMRouter-rs


ifeq ($(OS), Windows)
  LIBDIR := $(BINDIR)
  MAIN := backrunner-rs.exe
endif

MAIN_DEBUG := $(CARGO_TARGET)/debug/bin/$(MAIN)
MAIN_RELEASE := $(CARGO_TARGET)/release/bin/$(MAIN)

$(MAIN_DEBUG):
	cargo build

$(MAIN_RELEASE):
	cargo build --release

$(CFMMROUTER):
	$(MAKE) -C CFMMRouter-rs
	PREFIX=$(PREFIX) $(MAKE) -C CFMMRouter-rs install

.PHONY: build
build: $(MAIN_DEBUG)

.PHONY: build-release
build-release: $(MAIN_RELEASE)

.PHONY: install
install: $(MAIN_RELEASE)
# NOTE: adjust install location in .cargo/config.toml
	cargo install --path $(ROOT_DIR)

.PHONY: clean
clean:
	cargo clean
	$(MAKE) -C CFMMRouter-rs clean
.PHONY: clean-release
clean-release:
	cargo clean --release

.PHONY: distclean
distclean: 
	$(RM) -Rf $(CARGO_TARGET)
	$(MAKE) -C CFMMRouter-rs distclean
	$(RM) -Rf $(ROOT_DIR)/target

.PHONY: setup
setup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh





