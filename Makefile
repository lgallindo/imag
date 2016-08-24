toml = $@/Cargo.toml
bin = $@/target/debug/$@

doc-crate-toml=./.imag-documentation/Cargo.toml

ECHO=$(shell which echo) -e
CARGO=$(shell which cargo)

BINS=$(shell find -maxdepth 1 -name "imag-*" -type d)
LIBS=$(shell find -maxdepth 1 -name "libimag*" -type d)

BIN_TARGETS=$(patsubst imag-%,,$(BINS))
LIB_TARGETS=$(LIBS)
LIB_TARGETS_TEST=$(foreach x,$(subst ./,,$(LIBS)),test-$(x))
TARGETS=$(BIN_TARGETS) $(LIB_TARGETS)
CLEAN_TARGETS=$(foreach x,$(TARGETS),$(x)-clean)

all: $(TARGETS)
	@$(ECHO) "\t[ALL   ]"

bin: $(BIN_TARGETS)
	@$(ECHO) "\t[ALLBIN]"

lib: $(LIB_TARGETS)
	@$(ECHO) "\t[ALLLIB]"

lib-test: $(LIB_TARGETS_TEST)

clean: $(CLEAN_TARGETS)
	@$(ECHO) "\t[CLEAN ]"

$(TARGETS): %: .FORCE
	@$(ECHO) "\t[CARGO ]:\t$@"
	@$(CARGO) build --manifest-path ./$@/Cargo.toml

$(LIB_TARGETS_TEST): %: .FORCE
	@$(ECHO) "\t[TEST  ]:\t$@"
	@$(CARGO) test --manifest-path ./$(subst test-,,$@)/Cargo.toml

$(CLEAN_TARGETS): %: .FORCE
	@$(ECHO) "\t[CLEAN]:\t$(subst -clean,,$@)"
	@$(CARGO) clean --manifest-path ./$(subst -clean,,$@)/Cargo.toml

.FORCE:

