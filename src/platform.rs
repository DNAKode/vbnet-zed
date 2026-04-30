use zed_extension_api as zed;

pub(crate) fn language_server_binary_name() -> &'static str {
    match zed::current_platform().0 {
        zed::Os::Windows => "vbnet-ls.exe",
        _ => "vbnet-ls",
    }
}

pub(crate) fn language_server_fallback_binary_name() -> &'static str {
    match zed::current_platform().0 {
        zed::Os::Windows => "VbNet.LanguageServer.exe",
        _ => "VbNet.LanguageServer",
    }
}

pub(crate) fn debug_adapter_binary_name() -> &'static str {
    match zed::current_platform().0 {
        zed::Os::Windows => "netcoredbg.exe",
        _ => "netcoredbg",
    }
}

pub(crate) fn release_asset_name() -> Option<&'static str> {
    let (os, arch) = zed::current_platform();
    release_asset_name_for(os, arch)
}

pub(crate) fn release_asset_name_for(os: zed::Os, arch: zed::Architecture) -> Option<&'static str> {
    match (os, arch) {
        (zed::Os::Windows, zed::Architecture::X8664) => {
            Some("vbnet-language-server-win-x64.zip")
        }
        (zed::Os::Linux, zed::Architecture::X8664) => {
            Some("vbnet-language-server-linux-x64.tar.gz")
        }
        (zed::Os::Mac, zed::Architecture::X8664) => {
            Some("vbnet-language-server-osx-x64.tar.gz")
        }
        (zed::Os::Mac, zed::Architecture::Aarch64) => {
            Some("vbnet-language-server-osx-arm64.tar.gz")
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_assets_cover_published_targets() {
        assert_eq!(
            release_asset_name_for(zed::Os::Windows, zed::Architecture::X8664),
            Some("vbnet-language-server-win-x64.zip")
        );
        assert_eq!(
            release_asset_name_for(zed::Os::Linux, zed::Architecture::X8664),
            Some("vbnet-language-server-linux-x64.tar.gz")
        );
        assert_eq!(
            release_asset_name_for(zed::Os::Mac, zed::Architecture::Aarch64),
            Some("vbnet-language-server-osx-arm64.tar.gz")
        );
    }

    #[test]
    fn release_assets_reject_unpublished_targets() {
        assert_eq!(
            release_asset_name_for(zed::Os::Linux, zed::Architecture::Aarch64),
            None
        );
        assert_eq!(
            release_asset_name_for(zed::Os::Windows, zed::Architecture::Aarch64),
            None
        );
    }
}
