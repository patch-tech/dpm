.PHONY: gen

gen:
	for t in static/*; do \
		echo $$t; \
		if [[ -f $$t/Makefile ]]; then \
			make -C $$t; \
		fi; \
	done