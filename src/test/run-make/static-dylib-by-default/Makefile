-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else
all:
	$(RUSTC) foo.rs
	$(RUSTC) bar.rs
	$(CC) main.c -o $(call RUN_BINFILE,main) -lbar $(EXTRACFLAGS)
	rm $(TMPDIR)/*.rlib
	rm $(call DYLIB,foo)
	$(call RUN,main)

endif
