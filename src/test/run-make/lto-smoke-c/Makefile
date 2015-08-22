-include ../tools.mk

# Apparently older versions of GCC segfault if -g is passed...
CC := $(CC:-g=)

ifdef IS_MSVC
# FIXME #27979
all:

else
all:
	$(RUSTC) foo.rs -C lto
	$(CC) bar.c -lfoo -o $(call RUN_BINFILE,bar) $(EXTRACFLAGS) $(EXTRACXXFLAGS)
	$(call RUN,bar)

endif
