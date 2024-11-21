PACKAGES=by-axum

.PHONY: publish
publish: $(patsubst %,publish.%,$(PACKAGES))

publish.%:
	./publish.sh $*
