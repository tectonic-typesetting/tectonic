# How To: Build Tectonic: Configure the Default Bundle

Tectonic supports overriding the default bundle configuration at build time through environment variables. This feature is particularly useful for:

- System packagers who need to specify bundle sources during compilation
- Environments requiring reproducible builds
- Users who want to use mirrored bundles for better reliability

## Build-time Environment Variables for the Default Bundle

Two environment variables control the bundle configuration:

### `TECTONIC_BUNDLE_PREFIX`

This variable is **required** at compile time and specifies the base URL prefix for bundles.
A default value is provided in the build script `crates/bundles/build.rs` in the source repository.

The variable is used to construct bundle URLs in the following pattern:
```bash
$TECTONIC_BUNDLE_PREFIX/default_bundle_v$FORMAT_VERSION.tar
```
where `$FORMAT_VERSION` is a suffix from an internal variable, used to distinguish different versions
of the bundle's format for backward compatibility.

For example, the current default is given by:
```bash
# as of January 2025
TECTONIC_BUNDLE_PREFIX=https://relay.fullyjustified.net
```
so the full bundle URL is `https://relay.fullyjustified.net/default_bundle_v33.tar`,
in which `_v33` indicates the version of the bundle format.
Note that this hardcoded default may change, and the default value documented here may be outdated.
Please refer to the source code of Tectonic for the latest default.

### `TECTONIC_BUNDLE_LOCKED`

This variable is **optional** and, when set, provides a fixed URL for the bundle. If this variable contains any non-empty value, it takes precedence over the format-version-specific URL construction using `TECTONIC_BUNDLE_PREFIX`.

## Usage Examples

To build Tectonic with a custom bundle prefix:

```sh
TECTONIC_BUNDLE_PREFIX="https://mirror.example.com/tectonic-bundles" cargo build
```

To build Tectonic with a locked bundle URL:

```sh
TECTONIC_BUNDLE_LOCKED="https://mirror.example.com/tectonic-bundles/my-fixed-bundle.tar" cargo build
```

### Notes

- The bundle URL configuration happens at build time and may be overridden by appropriate `--bundle` flags at runtime.
- If using `TECTONIC_BUNDLE_LOCKED`, ensure the URL points to a compatible bundle version.
