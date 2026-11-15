PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man
MAN1DIR ?= $(MANDIR)/man1
CARGO ?= cargo
TARGET_DIR ?= target
PROFILE ?= release

BIN := rustfetch

.PHONY: all build release debug test lint fmt check clean install uninstall dist

all: build

build:
	$(CARGO) build --$(PROFILE)

release:
	$(CARGO) build --release

debug:
	$(CARGO) build

test:
	$(CARGO) test --all

lint:
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --all-targets -- -D warnings

fmt:
	$(CARGO) fmt --all

check:
	$(CARGO) fmt --all -- --check
	$(CARGO) clippy --all-targets -- -D warnings
	$(CARGO) test --all

clean:
	$(CARGO) clean

install: build
	install -Dm755 $(TARGET_DIR)/$(PROFILE)/$(BIN) $(DESTDIR)$(BINDIR)/$(BIN)
	install -Dm644 docs/rustfetch.1 $(DESTDIR)$(MAN1DIR)/rustfetch.1
	install -Dm644 LICENSE $(DESTDIR)$(PREFIX)/share/licenses/$(BIN)/LICENSE
	install -Dm644 README.md $(DESTDIR)$(PREFIX)/share/doc/$(BIN)/README.md

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/$(BIN)
	rm -f $(DESTDIR)$(MAN1DIR)/rustfetch.1
	rm -rf $(DESTDIR)$(PREFIX)/share/licenses/$(BIN)
	rm -rf $(DESTDIR)$(PREFIX)/share/doc/$(BIN)

dist:
	@echo "Create a release tarball with: git archive --format=tar.gz --prefix=$(BIN)-$$(grep '^version' Cargo.toml | head -n1 | cut -d'\"' -f2)/ -o $(BIN)-$$(grep '^version' Cargo.toml | head -n1 | cut -d'\"' -f2).tar.gz v$$(grep '^version' Cargo.toml | head -n1 | cut -d'\"' -f2)"
