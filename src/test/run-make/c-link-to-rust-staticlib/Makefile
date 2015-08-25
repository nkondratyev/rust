-include ../tools.mk

EXTRAFLAGS := $(EXTRACFLAGS)

ifdef IS_MSVC
# FIXME #27979
all:

else

# FIXME: ignore freebsd
ifneq ($(shell uname),FreeBSD)
all:
	$(RUSTC) foo.rs
	$(CC) bar.c -lfoo -o $(call RUN_BINFILE,bar) $(EXTRAFLAGS) $(EXTRACXXFLAGS)
	$(call RUN,bar)
	rm $(call STATICLIB,foo*)
	$(call RUN,bar)

else
all:

endif

endif
