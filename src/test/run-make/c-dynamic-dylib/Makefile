-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else

# This hits an assertion in the linker on older versions of osx apparently
ifeq ($(shell uname),Darwin)
all:
	echo ignored
else
all: $(call DYLIB,cfoo)
	$(RUSTC) foo.rs -C prefer-dynamic
	$(RUSTC) bar.rs
	$(call RUN,bar)
	$(call REMOVE_DYLIBS,cfoo)
	$(call FAIL,bar)
endif
endif
