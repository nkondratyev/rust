-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else
all: $(call STATICLIB,cfoo)
	$(RUSTC) foo.rs -C prefer-dynamic
	$(RUSTC) bar.rs
	rm $(TMPDIR)/$(call STATICLIB_GLOB,cfoo)
	$(call RUN,bar)
	$(call REMOVE_DYLIBS,foo)
	$(call FAIL,bar)
endif
