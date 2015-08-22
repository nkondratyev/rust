-include ../tools.mk

all:
	$(RUSTC) foo.rs
	rm $(TMPDIR)/$(call BIN,foo)
	$(RUSTC) foo.rs --crate-name bar
	rm $(TMPDIR)/$(call BIN,bar)
	$(RUSTC) foo1.rs
	rm $(TMPDIR)/$(call BIN,foo)
	$(BARE_RUSTC) foo1.rs -o $(TMPDIR)/bar1 -L $(TMPDIR)
	rm $(TMPDIR)/$(call BIN,bar1)
