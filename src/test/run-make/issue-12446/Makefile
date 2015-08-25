-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else
all: $(call STATICLIB,foo)
	$(RUSTC) foo.rs
	$(RUSTC) bar.rs
	$(call RUN,bar)

endif
