-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else
all: $(call STATICLIB,cfoo)
	$(RUSTC) foo.rs
	$(RUSTC) bar.rs
	$(call REMOVE_RLIBS,foo)
	rm $(TMPDIR)/$(call STATICLIB_GLOB,cfoo)
	$(call RUN,bar)

endif
