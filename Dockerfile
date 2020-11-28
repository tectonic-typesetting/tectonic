FROM archlinux

RUN pacman -Syy
# tectonic build required packages only. Install custom packages below to avoid docker recompiling
RUN pacman --noconfirm -S \
        pacman-contrib \
        wget \
        tar \
        sudo \
        binutils \
        fakeroot \
        pkg-config \
        gcc \
        coreutils

RUN useradd -m notroot
RUN echo "notroot ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/notroot

USER notroot
WORKDIR /home/notroot
RUN wget -q https://aur.archlinux.org/cgit/aur.git/snapshot/tectonic.tar.gz 2>/dev/null
RUN tar -zxvf tectonic.tar.gz
WORKDIR /home/notroot/tectonic
RUN makepkg -sric --noconfirm

USER root
# install additional packages here to avoid docker recompiling
RUN pacman -S --noconfirm ttf-liberation
RUN pacman -R --noconfirm wget sudo gcc
RUN paccache -r
RUN pacman -Scc
RUN pacman -Qtdq --noconfirm | pacman -Rns --noconfirm -

WORKDIR /data
VOLUME ["/data"]

