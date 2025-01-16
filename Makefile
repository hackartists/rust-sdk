PACKAGES=by-types by-macros by-axum rest-api

.PHONY: publish
publish: $(patsubst %,publish.%,$(PACKAGES))

publish.%:
	./publish.sh $*

.PHONY: build
build: $(patsubst %,build.%,$(PACKAGES))

build.%:
	cargo build -p $*
