-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else
all: $(TMPDIR)/libbar.a
	$(RUSTC) foo.rs -lstatic=bar
	$(RUSTC) main.rs
	$(call RUN,main)

endif
