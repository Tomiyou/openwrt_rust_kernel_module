include $(TOPDIR)/rules.mk
include $(INCLUDE_DIR)/kernel.mk

PKG_NAME:=rust-bindings
PKG_VERSION=0.1.0

PKG_SOURCE_PROTO:=git
PKG_SOURCE_URL:=https://github.com/Tomiyou/openwrt_rust_kernel_module
PKG_SOURCE_VERSION:=379577483e4199a1b759641c6b53f0a65d204366
PKG_SOURCE_SUBDIR:=$(PKG_NAME)-$(PKG_VERSION)
PKG_SOURCE:=$(PKG_NAME)-$(PKG_VERSION)-$(PKG_SOURCE_VERSION).tar.gz

include $(INCLUDE_DIR)/local-development.mk
include $(INCLUDE_DIR)/package.mk

define KernelPackage/rust-bindings
  SECTION:=kernel
  CATEGORY:=Kernel modules
  SUBMENU:=Network Support
  TITLE:=Rust bindings for Linux kernel
  FILES:=$(PKG_BUILD_DIR)/rust_bindings.ko
  AUTOLOAD:=$(call AutoLoad,08,rust_bindings)
endef

define KernelPackage/rust-bindings/description
	rust-bindings Rust bindings for Linux kernel.
endef

define Build/Compile
	$(MAKE) -C "$(LINUX_DIR)" \
		CROSS_COMPILE="$(TARGET_CROSS)" \
		ARCH="$(LINUX_KARCH)" \
		M="$(PKG_BUILD_DIR)" \
		EXTRA_CFLAGS="$(EXTRA_CFLAGS)" SoC="$(subtarget)" \
		modules
endef

define Build/InstallDev
	ln -s $(PKG_BUILD_DIR) $(STAGING_DIR)/rust-bindings
endef

$(eval $(call KernelPackage,rust-bindings))
