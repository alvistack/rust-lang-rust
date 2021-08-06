VERSION ?= 1.54.0
DATE ?= $(shell date "+%a %b %d %Y")

.PHONY: all
all: download dist-gzip deb rpm

.PHONY: download
download:
	mkdir -p .cache
	wget --no-check-certificate --relative --quiet --timeout=300 --continue \
		-O .cache/rust-$(VERSION)-x86_64-unknown-linux-gnu.tar.gz \
		https://static.rust-lang.org/dist/rust-$(VERSION)-x86_64-unknown-linux-gnu.tar.gz
	tar -zxv \
		--strip-components=1 \
		-f .cache/rust-$(VERSION)-x86_64-unknown-linux-gnu.tar.gz

.PHONY: dist-gzip
dist-gzip:
	rm -rf .cache
	tar -zcv \
		--exclude='./.git' \
		--exclude='./debian/cargo' \
		--exclude='./debian/libstd-rust-dev' \
		--exclude='./debian/rustc' \
		-f ../rustc_$(VERSION).orig.tar.gz .

.PHONY: deb
deb:
	dch -v $(VERSION)-1 __VERSION__
	sed "/^\s*\* __VERSION__/d" -i debian/changelog
	debuild -us -uc

.PHONY: rpm
rpm:
	cp rustc.spec ../rustc_$(VERSION)-1.spec
	sed "s/__VERSION__/$(VERSION)/g" -i ../rustc_$(VERSION)-1.spec
	sed "s/__DATE__/$(DATE)/g" -i ../rustc_$(VERSION)-1.spec
