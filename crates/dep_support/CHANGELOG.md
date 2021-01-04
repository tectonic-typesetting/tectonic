# rc: minor bump

A new crate to support Tectonic's searching for external libraries
("dependencies"). Notably, this crate supports finding deps using either
pkg-config or vcpkg. It does *not* (yet?) handle the question of deciding
whether to find a dependency externally or vendor it (build it locally).
