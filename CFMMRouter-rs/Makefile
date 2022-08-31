ROOT_DIR := $(abspath $(dir $(lastword $(MAKEFILE_LIST))))

JULIA ?= julia
DLEXT := $(shell $(JULIA) --startup-file=no -e 'using Libdl; print(Libdl.dlext)')

PREFIX := $(ROOT_DIR)/target
LIBDIR := $(PREFIX)/lib
BINDIR := $(PREFIX)/bin

ifeq ($(OS), Windows)
  LIBDIR := $(BINDIR)
endif

CFMMROUTER := $(LIBDIR)/cfmmrouter.$(DLEXT)
SUBDIRS := CFMMRouter.jl 

.PHONY: all $(SUBDIRS)
all: $(CFMMROUTER) $(MAIN_RS)
router: $(CFMMROUTER)

$(CFMMROUTER):
	$(MAKE) -C CFMMRouter.jl
	PREFIX=$(PREFIX) $(MAKE) -C CFMMRouter.jl install

.PHONY: clean
clean:
	$(MAKE) -C CFMMRouter.jl clean

.PHONY: distclean
distclean: clean
	$(MAKE) -C CFMMRouter.jl distclean
	$(RM) -Rf $(ROOT_DIR)/target

