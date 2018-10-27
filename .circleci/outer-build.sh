#! /bin/bash
# Copyright 2018 the Tectonic Project
# Licensed under the MIT License

set -e -x

buildroot=$HOME

# This link points to a tarball of a pre-assembled Ubuntu PowerPC chroot
# that can be run via QEMU. This tarball was generated according to the
# procedure describe here:
# https://github.com/tectonic-typesetting/tectonic-ci-support/blob/master/bigendian/README.md .
# The current version embeds Rust 1.27.0.

tarball='https://dataverse.harvard.edu/api/access/datafile/:persistentId?persistentId=doi:10.7910/DVN/VM4ZZ3/BM5RRE'

# Validate that our little scheme is going to work. Annoyingly
# $CIRCLE_WORKING_DIRECTORY (passed in as $1 since we're sudo -- it's stripped
# out of the environment) is defined with a literal `~` in its value.

work=$(eval echo $1)

case $work in
    $buildroot/*) ;;
    *) echo >&2 "error: working directory \"$work\" does not reside within \"$buildroot\""
       exit 1 ;;
esac

chroot_work=${work#$buildroot}

# Install the stuff we need to run in the QEMU chroot.

apt-get -qq update
apt-get install -qy binfmt-support qemu-user-static

# Xenial's QEMU has a bug in its PPC "binfmt_misc" definition that means that
# it fails to identify certain valid PPC binaries (namely, our
# /usr/lib/gcc/powerpc-linux-gnu/5/cc1). See
# https://aur.archlinux.org/packages/qemu-user-static/?comments=all . Fix it:

update-binfmts --disable qemu-ppc
sed -e 's/\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff/\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xfc/' \
    </var/lib/binfmts/qemu-ppc >binfmt-bugfix
tee /var/lib/binfmts/qemu-ppc <binfmt-bugfix
update-binfmts --enable qemu-ppc

# Now we can set up the build chroot. Generic setup steps from
# https://en.wikipedia.org/wiki/Chroot.

mkdir -p $buildroot
cd $buildroot
curl -fsSL "$tarball" |tar xz
mkdir -p dev/pts dev/shm proc sys tmp
mount -t proc proc proc
mount -t sysfs sys sys
mount -t tmpfs tmpfs dev/shm
mount -t devpts devpts dev/pts
chmod ugoa+rwx,+t tmp
for file in etc/hosts etc/resolv.conf ; do
    rm -f $file # blow away if existing symlink
    cp -f /$file $file
done

# Ready to hand off to the chroot!

exec chroot $buildroot $chroot_work/.circleci/inner-build.sh $chroot_work
