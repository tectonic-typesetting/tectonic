MAKEFILEDIR:=$(dir $(lastword $(MAKEFILE_LIST)))
PROJECTDIR:=$(MAKEFILEDIR)/../..
TARGETDIR:=$(PROJECTDIR)/target
RELEASEDIR:=$(TARGETDIR)/release
APPDIR:=$(RELEASEDIR)

all: tectonic-x86_64.AppImage

%.AppImage: linuxdeploy-x86_64.AppImage
	cargo build \
		--manifest-path=$(PROJECTDIR)/Cargo.toml \
		--target-dir=$(TARGETDIR) \
		--release
	$(MAKEFILEDIR)/linuxdeploy-x86_64.AppImage \
		--icon-file=$(MAKEFILEDIR)/tectonic.svg \
		--desktop-file=$(MAKEFILEDIR)/tectonic.desktop \
		--appdir=$(APPDIR) \
		--executable=$(RELEASEDIR)/tectonic \
		--output=appimage

linuxdeploy-x86_64.AppImage:
	wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/$@ -O $(MAKEFILEDIR)/$@
	chmod +x $(MAKEFILEDIR)/$@

