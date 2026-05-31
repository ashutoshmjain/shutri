//! Delivery module for deploying massaged assets to target repositories

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use crate::cli::TargetPlatform;
use crate::config::Config;
use regex::Regex;

/// Deploys the massaged assets for a given Master Key to the target platform repository.
pub fn publish_assets(mk: u32, mk_dir: &Path, target: &TargetPlatform, config: &Config) -> Result<()> {
    match target {
        TargetPlatform::Mdbook => deploy_to_mdbook(mk, mk_dir, &config.mdbook_repo),
        TargetPlatform::Linkedin => deploy_to_social(mk, mk_dir, &config.social_repo, "linkedin"),
        TargetPlatform::Nostr => deploy_to_social(mk, mk_dir, &config.social_repo, "nostr"),
    }
}

fn deploy_to_mdbook(mk: u32, mk_dir: &Path, dest_repo: &Path) -> Result<()> {
    let src_md_name = format!("{}.md", mk);
    let src_md = mk_dir.join(&src_md_name);
    let src_img = mk_dir.join("image.png");
    let src_video = mk_dir.join("video.mp4");
    let src_scroll = mk_dir.join("cinematic_scroll.html");

    let dest_src = dest_repo.join("src");
    let dest_img = dest_src.join("img");

    fs::create_dir_all(&dest_img).context("Failed to create mdBook destination folders")?;

    // 1. Deploy Markdown
    if src_md.exists() {
        fs::copy(&src_md, dest_src.join(&src_md_name))?;

        // Synchronize SUMMARY.md
        sync_summary(&src_md, &dest_src.join("SUMMARY.md"), &src_md_name)?;
    }

    // 2. Deploy Images
    if src_img.exists() {
        fs::copy(&src_img, dest_img.join(format!("{}.png", mk)))?;
    }

    // 3. Deploy Video
    if src_video.exists() {
        fs::copy(&src_video, dest_img.join(format!("video_{}.mp4", mk)))?;
    }

    // 4. Deploy Global Fragments
    if src_scroll.exists() {
        fs::copy(&src_scroll, dest_src.join("cinematic_scroll.html"))?;
    }

    println!("   ✅ mdBook assets deployed and indexed to {:?}", dest_src);
    Ok(())
}

fn sync_summary(source_md: &Path, summary_path: &Path, mk_filename: &str) -> Result<()> {
    if !summary_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(source_md)?;
    let h1_regex = Regex::new(r"(?m)^# (.*)$").unwrap();
    let title = h1_regex.captures(&content)
        .map(|c| c.get(1).unwrap().as_str())
        .unwrap_or("New Episode");

    let mut summary_content = fs::read_to_string(summary_path)?;

    if summary_content.contains(mk_filename) {
        println!("   ℹ️  Episode already indexed in SUMMARY.md");
        return Ok(());
    }

    let new_entry = format!("- [{}](./{})", title, mk_filename);

    let recent_header = "# Recent ..";
    if let Some(pos) = summary_content.find(recent_header) {
        let after_header = &summary_content[pos..];
        if let Some(newline_pos) = after_header.find('\n') {
            let insert_pos = pos + newline_pos + 1;
            summary_content.insert_str(insert_pos, &format!("{}\n", new_entry));
        }
    } else {
        summary_content.push_str(&format!("\n{}", new_entry));
    }

    fs::write(summary_path, summary_content)?;
    println!("   ✅ SUMMARY.md synchronized");
    Ok(())
}

fn deploy_to_social(mk: u32, mk_dir: &Path, dest_repo: &Path, platform: &str) -> Result<()> {
    let (src_filename, dest_extension) = match platform {
        "linkedin" => (format!("{}_linkedin.txt", mk), "txt"),
        "nostr" => (format!("{}_nostr.md", mk), "md"),
        _ => unreachable!(),
    };

    let src_path = mk_dir.join(src_filename);
    let dest_dir = dest_repo.join(platform);
    
    fs::create_dir_all(&dest_dir).context(format!("Failed to create social archive folder for {}", platform))?;

    if src_path.exists() {
        let dest_path = dest_dir.join(format!("{}.{}", mk, dest_extension));
        fs::copy(&src_path, &dest_path)?;
        println!("   ✅ Social buffer deployed to {:?}", dest_path);
    }

    Ok(())
}
