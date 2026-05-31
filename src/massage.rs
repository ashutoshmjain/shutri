//! # Massage Module (Transformation & Sanitization)
//! 
//! This module provides the logic for converting raw assets into platform-specific formats.
//! It handles text sanitization, KaTeX hardening, and snippet injection.

use anyhow::{Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::cli::TargetPlatform;

/// Sanitizes and formats the ingested text asset for the given target platform.
pub fn massage_text(mk: u32, mk_dir: &Path, target: &TargetPlatform) -> Result<PathBuf> {
    let source_path = mk_dir.join("text.rs");
    if !source_path.exists() {
        return Err(anyhow::anyhow!("Raw text asset not found for MK #{}", mk));
    }

    let raw_content = fs::read_to_string(&source_path)
        .context(format!("Failed to read raw text asset from {:?}", source_path))?;

    // 0. Shared Shield Stripping
    let clean_content = strip_shield(raw_content);

    let massaged_content = match target {
        TargetPlatform::Mdbook => massage_for_mdbook(clean_content, mk)?,
        TargetPlatform::Linkedin => massage_for_linkedin(clean_content, mk)?,
        TargetPlatform::Nostr => massage_for_nostr(clean_content, mk)?,
    };

    let dest_filename = match target {
        TargetPlatform::Mdbook => format!("{}.md", mk),
        TargetPlatform::Linkedin => format!("{}_linkedin.txt", mk),
        TargetPlatform::Nostr => format!("{}_nostr.md", mk),
    };

    let dest_path = mk_dir.join(dest_filename);
    fs::write(&dest_path, massaged_content)
        .context(format!("Failed to write massaged content to {:?}", dest_path))?;

    // If target is mdBook, we also need to rebuild the cinematic scroll HTML
    if let TargetPlatform::Mdbook = target {
        let assets_dir = mk_dir.parent().context("Failed to find assets base dir")?;
        let scroll_html = rebuild_cinematic_scroll(assets_dir, mk)?;
        let scroll_path = mk_dir.join("cinematic_scroll.html");
        fs::write(&scroll_path, scroll_html)
            .context(format!("Failed to write cinematic scroll to {:?}", scroll_path))?;
    }

    Ok(dest_path)
}

fn strip_shield(mut content: String) -> String {
    content = content.trim().to_string();
    if content.starts_with("```rust") {
        content = content[7..].trim().to_string();
    }
    if content.starts_with("r#\"") {
        content = content[3..].to_string();
    }
    if content.ends_with("```") {
        content = content[..content.len()-3].trim().to_string();
    }
    if content.ends_with("\"#") {
        content = content[..content.len()-2].trim().to_string();
    }
    content.trim().to_string()
}

fn massage_for_mdbook(mut content: String, mk: u32) -> Result<String> {
    // 1. Multimedia Snippet Injection (Cover Image & Socials)
    let mut header_section = String::new();
    
    // Cover Image
    header_section.push_str(&format!("![Cover Image](img/{}.png)\n\n", mk));
    
    // Podcast Links Snippet
    header_section.push_str("<center><a href=\"https://open.spotify.com/show/7doWf0GON9JsG6r8igc7RE\" target=\"_blank\" style=\"background-color: #2E2E2E; color: white; padding: 10px 20px; text-align: center; text-decoration: none; display: inline-block; border-radius: 5px; margin-top: 10px; margin-right: 10px;\">Spotify</a><a href=\"https://podcasts.apple.com/us/podcast/deep-dive-with-gemini/id1844532251\" target=\"_blank\" style=\"background-color: #2E2E2E; color: white; padding: 10px 20px; text-align: center; text-decoration: none; display: inline-block; border-radius: 5px; margin-top: 10px; margin-right: 10px;\">Apple Podcasts</a><a href=\"https://music.youtube.com/playlist?list=PLIX4sFsmu37qtJMlv-VzMYWM26M1QyXTe&si=o534zFZsc7p5XA9Q\" target=\"_blank\" style=\"background-color: #2E2E2E; color: white; padding: 10px 20px; text-align: center; text-decoration: none; display: inline-block; border-radius: 5px; margin-top: 10px; margin-right: 10px;\">YouTube Music</a><a href=\"https://www.youtube.com/playlist?list=PLIX4sFsmu37qtJMlv-VzMYWM26M1QyXTe\" target=\"_blank\" style=\"background-color: #2E2E2E; color: white; padding: 10px 20px; text-align: center; text-decoration: none; display: inline-block; border-radius: 5px; margin-top: 10px; margin-right: 10px;\">YouTube</a><a href=\"https://fountain.fm/show/7LBvZT6ffpGyubvk8aSF\" target=\"_blank\" style=\"background-color: #2E2E2E; color: white; padding: 10px 20px; text-align: center; text-decoration: none; display: inline-block; border-radius: 5px; margin-top: 10px;\">Fountain.fm</a></center>\n\n");

    // Inject Cinematic Scroll
    header_section.push_str("{{#include cinematic_scroll.html}}\n\n");

    // Replace H1 with the header section + the title
    let h1_regex = Regex::new(r"(?m)^# (.*)$").unwrap();
    if let Some(caps) = h1_regex.captures(&content) {
        let title = caps.get(1).unwrap().as_str();
        content = h1_regex.replace(&content, &format!("# {}\n\n{}", title, header_section)).to_string();
    } else {
        content = format!("{}\n\n{}", header_section, content);
    }

    // 2. KaTeX Hardening
    let math_block_regex = Regex::new(r"(?s)\$\$.*?\$\$").unwrap();
    let inline_math_regex = Regex::new(r"\$.*?\$").unwrap();
    let mut math_blocks = Vec::new();

    let content_with_placeholders = math_block_regex.replace_all(&content, |caps: &regex::Captures| {
        let placeholder = format!("__MATH_BLOCK_{}__", math_blocks.len());
        math_blocks.push(caps.get(0).unwrap().as_str().to_string());
        placeholder
    }).to_string();

    let content_with_all_placeholders = inline_math_regex.replace_all(&content_with_placeholders, |caps: &regex::Captures| {
        let placeholder = format!("__MATH_BLOCK_{}__", math_blocks.len());
        math_blocks.push(caps.get(0).unwrap().as_str().to_string());
        placeholder
    }).to_string();

    let mut temp_content = content_with_all_placeholders;

    // Currency and Dollar Escaping
    let currency_regex = Regex::new(r"(?i)\$([\d\.,]+)\s*(million|billion|trillion|k|m|b|t)?").unwrap();
    temp_content = currency_regex.replace_all(&temp_content, "$1 $2 USD ").to_string();
    temp_content = temp_content.replace("  ", " ");
    temp_content = temp_content.replace("$", r"\$");

    // Restore Math Blocks
    for (idx, block) in math_blocks.iter().enumerate() {
        let placeholder = format!("__MATH_BLOCK_{}__", idx);
        temp_content = temp_content.replace(&placeholder, block);
    }

    // 3. Footnote Fixes
    temp_content = fix_footnotes(temp_content);

    // 4. Lightning Widget Injection (Proper Markdown + HTML)
    temp_content.push_str("\n\n---\n\n<center>Support this research via Lightning: **shutosha@primal.net**</center>\n");

    Ok(temp_content)
}

fn massage_for_linkedin(content: String, _mk: u32) -> Result<String> {
    let h1_regex = Regex::new(r"(?m)^# (.*)$").unwrap();
    let title = h1_regex.captures(&content)
        .map(|c| c.get(1).unwrap().as_str())
        .unwrap_or("New Research Update");

    let no_headers = h1_regex.replace_all(&content, "").to_string();
    let footnote_marker_regex = Regex::new(r"\[\^\d+\]").unwrap();
    let clean_text = footnote_marker_regex.replace_all(&no_headers, "").to_string();
    
    let clean_text = clean_text.replace("$$", "").replace("$", "");

    let pruned_text = if clean_text.len() > 2000 {
        format!("{}...", &clean_text[..2000])
    } else {
        clean_text
    };

    let mut output = String::new();
    output.push_str(&format!("TITLE: {}\n\n", title));
    output.push_str("CONTENT_START\n");
    output.push_str(pruned_text.trim());
    output.push_str("\nCONTENT_END\n\n");
    output.push_str("--- AI_INSTRUCTION ---\n");
    output.push_str("Rewrite the above content as a professional, engaging LinkedIn post. ");
    output.push_str("Use an 'Intriguing & Analytical' tone. Include 3 relevant hashtags. ");
    output.push_str("End with a call to action to read the full paper at shutri.com.");

    Ok(output)
}

fn massage_for_nostr(content: String, mk: u32) -> Result<String> {
    let mut output = String::new();
    output.push_str(&format!("--- NOSTR_METADATA ---\n"));
    output.push_str(&format!("title: Episode #{}\n", mk));
    output.push_str(&format!("tags: research, deepdive, sms\n"));
    output.push_str(&format!("--- END_METADATA ---\n\n"));
    
    let include_regex = Regex::new(r"\{\{#include .*?\}\}").unwrap();
    let clean_content = include_regex.replace_all(&content, "").to_string();
    
    output.push_str(&clean_content);

    Ok(output)
}

pub fn rebuild_cinematic_scroll(assets_dir: &Path, current_mk: u32) -> Result<String> {
    let mut html = String::new();
    
    // Toggle Javascript
    html.push_str("<script>\n  window.oph_toggle = function(btn) {\n    const parent = btn.parentElement;\n    const vid = parent.querySelector('video');\n    const container = btn.closest('.video-carousel-container');\n    if (vid.paused) {\n      container.querySelectorAll('video').forEach(v => { if (v !== vid) { v.pause(); v.muted = true; const otherBtn = v.parentElement.querySelector('.vid-toggle'); if (otherBtn) otherBtn.innerText = '🔇'; } });\n      vid.muted = false; vid.volume = 1.0; vid.play(); btn.innerText = '🔊';\n    } else {\n      vid.pause(); vid.muted = true; btn.innerText = '🔇';\n    }\n  };\n</script>\n");

    // Carousel Container
    html.push_str("<div class=\"video-carousel-container\" style=\"display: flex; overflow-x: auto; scroll-snap-type: x mandatory; gap: 15px; padding: 20px 0; scroll-behavior: smooth;\">\n");

    let mut mks: Vec<u32> = fs::read_dir(assets_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str()?.parse::<u32>().ok())
        .collect();
    
    mks.sort_by(|a, b| b.cmp(a));

    let mut target_vid_id = "vid-0";

    for (i, mk) in mks.iter().enumerate() {
        let mk_video = assets_dir.join(mk.to_string()).join("video.mp4");
        if mk_video.exists() {
            let vid_id = format!("vid-{}", i);
            if *mk == current_mk {
                target_vid_id = Box::leak(vid_id.clone().into_boxed_str());
            }

            html.push_str(&format!(
                "  <div id=\"{}\" style=\"flex: 0 0 60%; scroll-snap-align: center; position: relative; border-radius: 12px; overflow: hidden; background: #000; aspect-ratio: 1/1; display: flex; flex-direction: column;\">\n",
                vid_id
            ));
            html.push_str(&format!(
                "    <video src=\"img/video_{}.mp4\" style=\"width: 100%; height: 85%; object-fit: contain; pointer-events: none;\" playsinline loop preload=\"auto\"></video>\n",
                mk
            ));
            html.push_str(&format!(
                "    <div style=\"height: 15%; background: #1a1a1a; color: #ccc; display: flex; align-items: center; justify-content: center; font-family: monospace; font-size: 12px; border-top: 1px solid #333;\">Episode #{}</div>\n",
                mk
            ));
            html.push_str("    <button class=\"vid-toggle\" onclick=\"oph_toggle(this)\" style=\"position: absolute; top: 10px; right: 10px; background: rgba(0,0,0,0.8); color: white; border: 2px solid white; border-radius: 50%; width: 35px; height: 35px; cursor: pointer; font-size: 18px; z-index: 100;\">🔇</button>\n");
            html.push_str("  </div>\n");
        }
    }

    html.push_str("</div>\n");

    // Focus script
    html.push_str("<script>\n  window.addEventListener('load', () => {\n    const container = document.querySelector('.video-carousel-container');\n    if (container) {\n");
    html.push_str(&format!("      const el = document.getElementById('{}');\n", target_vid_id));
    html.push_str("      if (el) { const offset = el.offsetLeft - (container.offsetWidth / 2) + (el.offsetWidth / 2); container.scrollTo({ left: offset, behavior: 'smooth' }); }\n");
    html.push_str("    }\n  });\n</script>\n");

    Ok(html)
}

fn fix_footnotes(content: String) -> String {
    let header_regex = Regex::new(r"(?i)#### \*\*Works cited\*\*|#### \*\*References\*\*|## Bibliography|## References or Bibliography|## References").unwrap();
    let parts: Vec<&str> = header_regex.split(&content).collect();
    if parts.len() < 2 { return content; }

    let body = parts[0];
    let refs_raw = parts[1];
    let header = "#### **Works cited**";

    let marker_pattern = Regex::new(r"\[(\d+)\]").unwrap();
    let mut old_to_new = HashMap::new();
    let mut count = 1;

    let final_body = marker_pattern.replace_all(body, |caps: &regex::Captures| {
        let old_num = caps.get(1).unwrap().as_str();
        if !old_to_new.contains_key(old_num) {
            old_to_new.insert(old_num.to_string(), count.to_string());
            count += 1;
        }
        format!("[^{}]", old_to_new.get(old_num).unwrap())
    });

    let mut result = final_body.to_string();
    result.push_str("\n\n");
    result.push_str(header);
    result.push_str("\n\n");

    let ref_list_pattern = Regex::new(r"(?m)^\*?\s*(\[?(\d+)\]?)\s*(.*)").unwrap();
    for caps in ref_list_pattern.captures_iter(refs_raw) {
        let old_num = caps.get(2).unwrap().as_str();
        let ref_text = caps.get(3).unwrap().as_str().trim();
        if let Some(new_num) = old_to_new.get(old_num) {
            result.push_str(&format!("[^{}]: {}\n\n", new_num, ref_text));
        }
    }

    result
}
