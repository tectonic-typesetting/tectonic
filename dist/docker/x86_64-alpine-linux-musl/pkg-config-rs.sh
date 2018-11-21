#!/bin/sh
# Copyright 2018 The Tectonic Project
# Licensed under the MIT License.

set -ex

git clone --branch 0.3.14 https://github.com/alexcrichton/pkg-config-rs /pkg-config-rs

# make pkg-config-rs allows static linking with system libraries
cd /pkg-config-rs
patch -p1 <<'EOF'
diff --git a/src/lib.rs b/src/lib.rs
index 88dd310..ffcd7ae 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -547,7 +547,7 @@ fn is_static_available(name: &str, dirs: &[PathBuf]) -> bool {
     };
 
     dirs.iter().any(|dir| {
-        !system_roots.iter().any(|sys| dir.starts_with(sys)) &&
+        // !system_roots.iter().any(|sys| dir.starts_with(sys)) &&
         dir.join(&libname).exists()
     })
 }
EOF
