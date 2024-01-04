obj-m += rust_bindings.o
clean-files := c_flags

# Some systems for installing kernel headers (e.g. Debian's) happen to
# trigger the out-of-tree build code because the kernel headers directly
# actually just recursively invokes another non-arch-specific one. This
# means that they already generate absolute paths for -I by using the
# flags/addtree make functions.  Some (e.g. Ubuntu's) do not, and
# generate relative paths. We want absolute paths, but we cannot force
# the out-of-tree build code because it won't work on Debian-style
# kernel headers directory (it will look in the mostly-empty kernel
# headers directory instead of the actual one). So we steal the addtree
# and flags functions from scripts/Kbuild.include, and use them _after_
# the build system has generated paths - if any remaining paths are
# relative, we make them absolute with respect to CURDIR. (Unlike the
# upstream addtree function, we prefix -I./foo. We also need to fix
# -include ./include/linux/kconfig.h)
our_addtree = $(if $(patsubst -I%,%,$(1)), \
$(if $(filter-out -I/% -I../%,$(1)),$(patsubst ./%,$(CURDIR)/%,$(patsubst -I%,-I$(CURDIR)/%,$(1))),$(1)),$(1))
our_flags = $(foreach o,$($(1)),$(call our_addtree,$(o)))

# Incompatible Clang flags
bindgen_skip_c_flags := -mno-fp-ret-in-387 -mpreferred-stack-boundary=% \
	-mskip-rax-setup -mgeneral-regs-only -msign-return-address=% \
	-mindirect-branch=thunk-extern -mindirect-branch-register \
	-mfunction-return=thunk-extern -mrecord-mcount -mabi=lp64 \
	-mindirect-branch-cs-prefix -mstack-protector-guard% -mtraceback=no \
	-mno-pointers-to-nested-functions -mno-string \
	-mno-strict-align -mstrict-align -mno-thumb-interwork \
	-fconserve-stack -falign-jumps=% -falign-loops=% \
	-femit-struct-debug-baseonly -fno-ipa-cp-clone -fno-ipa-sra \
	-fno-partial-inlining -fplugin-arg-arm_ssp_per_task_plugin-% \
	-fno-reorder-blocks -fno-allow-store-data-races -fasan-shadow-offset=% \
	-fzero-call-used-regs=% -fno-stack-clash-protection \
	-fno-inline-functions-called-once -fsanitize=bounds-strict \
	-fno-caller-saves -fstrict-flex-arrays=% -fno-var-tracking-assignments \
	--param=% --param asan-%

# Kernel 5.2+: _c_flags, before __c_flags

$(M)/rust_bindings.c:
	$(info Exporting C flags to file)
	$(file > $(M)/c_flags,$(NOSTDINC_FLAGS) $(call our_flags,LINUXINCLUDE) $(filter-out $(bindgen_skip_c_flags), $(__c_flags) $(_c_flags)) $(modkern_cflags))

.PHONY: $(M)/rust_bindings.c

