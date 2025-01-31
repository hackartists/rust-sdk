PACKAGES=by-types by-macros rest-api dioxus-aws easy-dynamodb by-axum by-components dioxus-oauth dioxus-popup dioxus-translate-types dioxus-translate-macro dioxus-translate google-wallet

.PHONY: publish
publish: $(patsubst %,publish.%,$(PACKAGES))

publish.%:
	./publish.sh $*

.PHONY: build
build: $(patsubst %,build.%,$(PACKAGES))

build.%:
	cargo build -p $*
