VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
LINUX_TARGET := x86_64-unknown-linux-gnu
WINDOWS_TARGET := x86_64-pc-windows-gnu
LINUX_BIN := target/$(LINUX_TARGET)/release/resize
WINDOWS_BIN := target/$(WINDOWS_TARGET)/release/resize.exe
MANPAGE := man/resize.1

.PHONY: all build linux windows install man release clean

all: build man

build: linux windows

linux:
	cargo build --release --target $(LINUX_TARGET)
	strip $(LINUX_BIN)

windows:
	cargo build --release --target $(WINDOWS_TARGET)
	x86_64-w64-mingw32-strip $(WINDOWS_BIN)

man: $(MANPAGE)
	@sed -i 's/^\.TH RESIZE 1 .*/\.TH RESIZE 1 "$(shell date +"%B %Y")" "resize $(VERSION)" "User Commands"/' $(MANPAGE)
	@echo "Man page updated to version $(VERSION)"

install: linux man
	sudo cp $(LINUX_BIN) /usr/local/bin/resize
	sudo mkdir -p /usr/local/share/man/man1
	sudo cp $(MANPAGE) /usr/local/share/man/man1/resize.1
	sudo mandb -q
	@echo "Installed resize $(VERSION) to /usr/local/bin and man page to /usr/local/share/man/man1"

release: build man
	@echo "Creating GitHub release v$(VERSION)..."
	-gh release delete v$(VERSION) --yes 2>/dev/null
	gh release create v$(VERSION) \
		$(LINUX_BIN)#resize-linux-x86_64 \
		$(WINDOWS_BIN)#resize-windows-x86_64.exe \
		--title "v$(VERSION)" \
		--notes "$$(cat <<'EOF'\n## resize v$(VERSION)\n\nSee [README](https://github.com/nestorwheelock/resize#readme) for full documentation.\n\n### Downloads\n- **Linux (x86_64):** resize-linux-x86_64\n- **Windows (x86_64):** resize-windows-x86_64.exe\n\nLicense: GPL-3.0-or-later\nEOF\n)"
	@echo "Released v$(VERSION)"

clean:
	cargo clean
