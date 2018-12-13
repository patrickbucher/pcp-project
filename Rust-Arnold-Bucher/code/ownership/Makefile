%:
	$(eval FILE := $(wildcard src/$@-*.rs))
	@rustc $(FILE) --out-dir bin
	@./$(subst src,bin,$(subst .rs,,$(FILE)))

clean:
	rm -rf bin