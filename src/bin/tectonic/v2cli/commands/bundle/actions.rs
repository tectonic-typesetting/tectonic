use super::{
    create::{BundleCreateCommand, BundleFormat},
    pack::bundlev1::BundleV1,
    select::{picker::FilePicker, spec::BundleSpec},
};
use anyhow::{Context, Result};
use std::{
    cmp::Ordering,
    fs::{self, File},
    io::Read,
    thread,
    time::Duration,
};
use tracing::{error, info, warn};

pub(super) fn select(cli: &BundleCreateCommand) -> Result<()> {
    let bundle_dir = cli
        .bundle_spec
        .canonicalize()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let mut file = File::open(&cli.bundle_spec)?;
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)?;
    let bundle_config: BundleSpec = match toml::from_str(&file_str) {
        Ok(x) => x,
        Err(e) => {
            error!("failed to load bundle specification",);
            return Err(e.into());
        }
    };

    if let Err(e) = bundle_config.validate() {
        error!("failed to validate bundle specification");
        return Err(e);
    };

    let build_dir = cli.build_dir.join(&bundle_config.bundle.name);

    // Remove build dir if it exists
    if build_dir.exists() {
        warn!("build dir {} aleady exists", build_dir.to_str().unwrap());

        for i in (1..=5).rev() {
            warn!(
                "recursively removing {} in {i} second{}",
                build_dir.to_str().unwrap(),
                if i != 1 { "s" } else { "" }
            );
            thread::sleep(Duration::from_secs(1));
        }
        thread::sleep(Duration::from_secs(2));

        fs::remove_dir_all(&build_dir)?;
    }
    fs::create_dir_all(&build_dir).context("while creating build dir")?;

    let mut picker = FilePicker::new(bundle_config.clone(), build_dir.clone(), bundle_dir.clone())?;

    // Run selector
    let sources: Vec<String> = picker.iter_sources().map(|x| x.to_string()).collect();
    for source in sources {
        picker.add_source(cli, &source)?;
    }
    picker.finish(true)?;

    // Print statistics
    info!("summary is below:\n{}", picker.stats.make_string());

    match picker.stats.compare_patch_found_applied() {
        Ordering::Equal => {}
        Ordering::Greater => {
            warn!("some patches were not applied");
        }
        Ordering::Less => {
            warn!("some patches applied multiple times");
        }
    }

    // Check output hash
    {
        let mut file = File::open(build_dir.join("content/SHA256SUM"))?;
        let mut hash = String::new();
        file.read_to_string(&mut hash)?;
        let hash = hash.trim();
        if hash != bundle_config.bundle.expected_hash {
            warn!("final bundle hash doesn't match bundle configuration:");
            warn!("bundle hash is {hash}");
            warn!("config hash is {}", bundle_config.bundle.expected_hash);
        } else {
            info!("final bundle hash matches configuration");
            info!("hash is {hash}");
        }
    }

    Ok(())
}

pub(super) fn pack(cli: &BundleCreateCommand) -> Result<()> {
    let mut file = File::open(&cli.bundle_spec)?;
    let mut file_str = String::new();
    file.read_to_string(&mut file_str)?;
    let bundle_config: BundleSpec = toml::from_str(&file_str)?;

    let build_dir = cli.build_dir.join(&bundle_config.bundle.name);

    if !build_dir.join("content").is_dir() {
        error!(
            "content directory `{}/content` doesn't exist, can't continue",
            build_dir.to_str().unwrap()
        );
        return Ok(());
    }

    let target_name = format!("{}.ttb", &bundle_config.bundle.name);
    let target = build_dir.join(&target_name);
    if target.exists() {
        if target.is_file() {
            warn!("target bundle `{target_name}` exists, removing");
            fs::remove_file(&target)?;
        } else {
            error!("target bundle `{target_name}` exists and isn't a file, can't continue");
            return Ok(());
        }
    }

    match cli.format {
        BundleFormat::BundleV1 => {
            BundleV1::make(Box::new(File::create(target)?), build_dir.clone())?
        }
    }

    Ok(())
}
