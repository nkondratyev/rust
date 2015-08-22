-include ../tools.mk

ifdef IS_MSVC
# FIXME #27979
all:

else
all:
	$(CC) -std=c99 test.c -c -o $(TMPDIR)/test.o
	$(AR) rcs $(TMPDIR)/libtest.a $(TMPDIR)/test.o
	$(RUSTC) test.rs -L $(TMPDIR)
	$(call RUN,test) || exit 1

endif
