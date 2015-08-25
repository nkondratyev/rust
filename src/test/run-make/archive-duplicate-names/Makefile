-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else
all:
	mkdir $(TMPDIR)/a
	mkdir $(TMPDIR)/b
	$(CC) -c -o $(TMPDIR)/a/foo.o foo.c
	$(CC) -c -o $(TMPDIR)/b/foo.o bar.c
	ar crus $(TMPDIR)/libfoo.a $(TMPDIR)/a/foo.o $(TMPDIR)/b/foo.o
	$(RUSTC) foo.rs
	$(RUSTC) bar.rs
	$(call RUN,bar)
endif
