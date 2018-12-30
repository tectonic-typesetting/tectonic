# We use a phony stamp file here to force Make to always run the `cargo build`
# command, since Cargo won't do any work if the release executable is
# up-to-date. Due to the weaknesses of Make, we will always regenerate the
# AppImage file even if the executable was up-to-date, but that's OK.

MAKEFILEDIR:=$(dir $(lastword $(MAKEFILE_LIST)))
PROJECTDIR:=$(MAKEFILEDIR)/../..
TARGETDIR:=$(PROJECTDIR)/target
RELEASEDIR:=$(TARGETDIR)/release
EXECUTABLE:=$(RELEASEDIR)/tectonic
APPDIR:=$(MAKEFILEDIR)/appdir

all: Tectonic-x86_64.AppImage

%.AppImage: linuxdeploy-x86_64.AppImage $(EXECUTABLE).always
	mkdir -p $(APPDIR)
	rm -rf $(APPDIR)/usr/bin # pre-existing executables don't get updated!
	$(MAKEFILEDIR)/linuxdeploy-x86_64.AppImage \
		--icon-file=$(MAKEFILEDIR)/tectonic.svg \
		--desktop-file=$(MAKEFILEDIR)/tectonic.desktop \
		--appdir=$(APPDIR) \
		--executable=$(RELEASEDIR)/tectonic \
		--output=appimage

linuxdeploy-x86_64.AppImage:
	wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/$@ -O $(MAKEFILEDIR)/$@
	chmod +x $(MAKEFILEDIR)/$@

$(EXECUTABLE).always:
	cargo build \
		--manifest-path=$(PROJECTDIR)/Cargo.toml \
		--target-dir=$(TARGETDIR) \
		--release
