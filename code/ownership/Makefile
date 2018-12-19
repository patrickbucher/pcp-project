%:
	$(eval FILE := $(wildcard src/$@-*.rs))
	@bat $(FILE)
	@rustc $(FILE) --out-dir bin
	@./$(subst src,bin,$(subst .rs,,$(FILE)))

clean:
	rm -rf bin
