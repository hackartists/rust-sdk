PACKAGES=by-types by-macros by-axum rest-api

.PHONY: publish
publish: $(patsubst %,publish.%,$(PACKAGES))

publish.%:
	./publish.sh $*
