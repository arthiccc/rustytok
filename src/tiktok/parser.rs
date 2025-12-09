use regex::Regex;
use serde_json::Value;

use crate::error::AppError;
use super::types::{UserInfo, VideoInfo, TagInfo};

/// Extract SIGI_STATE JSON from TikTok HTML pages
fn extract_sigi_state(html: &str) -> Option<Value> {
    // TikTok embeds data in a script tag with id="SIGI_STATE" or __UNIVERSAL_DATA_FOR_REHYDRATION__
    let patterns = [
        r#"<script id="SIGI_STATE"[^>]*>([^<]+)</script>"#,
        r#"<script id="__UNIVERSAL_DATA_FOR_REHYDRATION__"[^>]*>([^<]+)</script>"#,
    ];
    
    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(html) {
                if let Some(json_str) = caps.get(1) {
                    if let Ok(json) = serde_json::from_str::<Value>(json_str.as_str()) {
                        return Some(json);
                    }
                }
            }
        }
    }
    
    // Try finding JSON in script tags
    let script_re = Regex::new(r#"<script[^>]*>(.*?)</script>"#).ok()?;
    for caps in script_re.captures_iter(html) {
        if let Some(content) = caps.get(1) {
            let content = content.as_str();
            if content.contains("\"UserModule\"") || content.contains("\"ItemModule\"") {
                if let Ok(json) = serde_json::from_str::<Value>(content) {
                    return Some(json);
                }
            }
        }
    }
    
    None
}

pub fn parse_user_page(html: &str, username: &str) -> Result<UserInfo, AppError> {
    // Try to extract JSON data
    if let Some(json) = extract_sigi_state(html) {
        // Try different JSON structures TikTok uses
        if let Some(user_info) = parse_user_from_json(&json, username) {
            return Ok(user_info);
        }
    }
    
    // Fallback: create placeholder user info
    tracing::warn!("Could not parse TikTok JSON, using fallback for user: {}", username);
    
    Ok(UserInfo {
        id: "unknown".to_string(),
        username: username.to_string(),
        nickname: username.to_string(),
        bio: "Profile information could not be loaded. TikTok may have changed their page structure.".to_string(),
        avatar_url: String::new(),
        follower_count: 0,
        following_count: 0,
        like_count: 0,
        video_count: 0,
        videos: vec![],
    })
}

fn parse_user_from_json(json: &Value, username: &str) -> Option<UserInfo> {
    // Try __DEFAULT_SCOPE__ structure (newer)
    if let Some(scope) = json.get("__DEFAULT_SCOPE__") {
        if let Some(user_detail) = scope.get("webapp.user-detail") {
            if let Some(user_info) = user_detail.get("userInfo") {
                return parse_user_info_object(user_info, username);
            }
        }
    }
    
    // Try UserModule structure (older)
    if let Some(user_module) = json.get("UserModule") {
        if let Some(users) = user_module.get("users") {
            if let Some(user) = users.get(username) {
                return parse_legacy_user_object(user, username, json);
            }
        }
    }
    
    None
}

fn parse_user_info_object(user_info: &Value, username: &str) -> Option<UserInfo> {
    let user = user_info.get("user")?;
    let stats = user_info.get("stats").unwrap_or(&Value::Null);
    
    Some(UserInfo {
        id: user.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        username: user.get("uniqueId").and_then(|v| v.as_str()).unwrap_or(username).to_string(),
        nickname: user.get("nickname").and_then(|v| v.as_str()).unwrap_or(username).to_string(),
        bio: user.get("signature").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        avatar_url: user.get("avatarLarger").and_then(|v| v.as_str())
            .or_else(|| user.get("avatarMedium").and_then(|v| v.as_str()))
            .unwrap_or("").to_string(),
        follower_count: stats.get("followerCount").and_then(|v| v.as_u64()).unwrap_or(0),
        following_count: stats.get("followingCount").and_then(|v| v.as_u64()).unwrap_or(0),
        like_count: stats.get("heartCount").and_then(|v| v.as_u64())
            .or_else(|| stats.get("heart").and_then(|v| v.as_u64())).unwrap_or(0),
        video_count: stats.get("videoCount").and_then(|v| v.as_u64()).unwrap_or(0),
        videos: vec![], // Videos would need separate parsing
    })
}

fn parse_legacy_user_object(user: &Value, username: &str, json: &Value) -> Option<UserInfo> {
    let stats = json.get("UserModule")?.get("stats")?.get(username)?;
    
    Some(UserInfo {
        id: user.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        username: user.get("uniqueId").and_then(|v| v.as_str()).unwrap_or(username).to_string(),
        nickname: user.get("nickname").and_then(|v| v.as_str()).unwrap_or(username).to_string(),
        bio: user.get("signature").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        avatar_url: user.get("avatarLarger").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        follower_count: stats.get("followerCount").and_then(|v| v.as_u64()).unwrap_or(0),
        following_count: stats.get("followingCount").and_then(|v| v.as_u64()).unwrap_or(0),
        like_count: stats.get("heartCount").and_then(|v| v.as_u64()).unwrap_or(0),
        video_count: stats.get("videoCount").and_then(|v| v.as_u64()).unwrap_or(0),
        videos: vec![],
    })
}

pub fn parse_video_page(html: &str, video_id: &str) -> Result<VideoInfo, AppError> {
    if let Some(json) = extract_sigi_state(html) {
        if let Some(video) = parse_video_from_json(&json, video_id) {
            return Ok(video);
        }
    }
    
    tracing::warn!("Could not parse TikTok JSON, using fallback for video: {}", video_id);
    
    Ok(VideoInfo {
        id: video_id.to_string(),
        description: "Video information could not be loaded.".to_string(),
        author_username: "unknown".to_string(),
        author_nickname: "Unknown".to_string(),
        author_avatar: String::new(),
        video_url: String::new(),
        thumbnail_url: String::new(),
        like_count: 0,
        comment_count: 0,
        share_count: 0,
        view_count: 0,
        create_time: 0,
        music_title: None,
        music_author: None,
    })
}

fn parse_video_from_json(json: &Value, video_id: &str) -> Option<VideoInfo> {
    // Try __DEFAULT_SCOPE__ structure
    if let Some(scope) = json.get("__DEFAULT_SCOPE__") {
        if let Some(video_detail) = scope.get("webapp.video-detail") {
            if let Some(item_info) = video_detail.get("itemInfo") {
                if let Some(item_struct) = item_info.get("itemStruct") {
                    return parse_video_item(item_struct);
                }
            }
        }
    }
    
    // Try ItemModule structure
    if let Some(item_module) = json.get("ItemModule") {
        if let Some(item) = item_module.get(video_id) {
            return parse_video_item(item);
        }
        // Sometimes there's only one item
        if let Some(items) = item_module.as_object() {
            if let Some((_, item)) = items.iter().next() {
                return parse_video_item(item);
            }
        }
    }
    
    None
}

fn parse_video_item(item: &Value) -> Option<VideoInfo> {
    let author = item.get("author").unwrap_or(&Value::Null);
    let stats = item.get("stats").unwrap_or(&Value::Null);
    let video = item.get("video").unwrap_or(&Value::Null);
    let music = item.get("music");
    
    // Get video URL - try multiple possible locations
    let video_url = video.get("playAddr")
        .or_else(|| video.get("downloadAddr"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    let thumbnail_url = video.get("cover")
        .or_else(|| video.get("originCover"))
        .or_else(|| video.get("dynamicCover"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    Some(VideoInfo {
        id: item.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        description: item.get("desc").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        author_username: author.get("uniqueId").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
        author_nickname: author.get("nickname").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
        author_avatar: author.get("avatarMedium").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        video_url: video_url.to_string(),
        thumbnail_url: thumbnail_url.to_string(),
        like_count: stats.get("diggCount").and_then(|v| v.as_u64()).unwrap_or(0),
        comment_count: stats.get("commentCount").and_then(|v| v.as_u64()).unwrap_or(0),
        share_count: stats.get("shareCount").and_then(|v| v.as_u64()).unwrap_or(0),
        view_count: stats.get("playCount").and_then(|v| v.as_u64()).unwrap_or(0),
        create_time: item.get("createTime").and_then(|v| v.as_i64()).unwrap_or(0),
        music_title: music.and_then(|m| m.get("title")).and_then(|v| v.as_str()).map(String::from),
        music_author: music.and_then(|m| m.get("authorName")).and_then(|v| v.as_str()).map(String::from),
    })
}

pub fn parse_tag_page(html: &str, tag_name: &str) -> Result<TagInfo, AppError> {
    if let Some(json) = extract_sigi_state(html) {
        if let Some(tag) = parse_tag_from_json(&json, tag_name) {
            return Ok(tag);
        }
    }
    
    tracing::warn!("Could not parse TikTok JSON, using fallback for tag: {}", tag_name);
    
    Ok(TagInfo {
        name: tag_name.to_string(),
        view_count: 0,
        videos: vec![],
    })
}

fn parse_tag_from_json(json: &Value, tag_name: &str) -> Option<TagInfo> {
    // Try __DEFAULT_SCOPE__ structure
    if let Some(scope) = json.get("__DEFAULT_SCOPE__") {
        if let Some(challenge_detail) = scope.get("webapp.challenge-detail") {
            let challenge_info = challenge_detail.get("challengeInfo")?;
            let challenge = challenge_info.get("challenge")?;
            let stats = challenge_info.get("stats").unwrap_or(&Value::Null);
            
            return Some(TagInfo {
                name: challenge.get("title").and_then(|v| v.as_str()).unwrap_or(tag_name).to_string(),
                view_count: stats.get("viewCount").and_then(|v| v.as_u64()).unwrap_or(0),
                videos: vec![], // Would need separate parsing
            });
        }
    }
    
    // Try ChallengePage structure
    if let Some(challenge_page) = json.get("ChallengePage") {
        if let Some(challenge_info) = challenge_page.get("challengeInfo") {
            let challenge = challenge_info.get("challenge")?;
            let stats = challenge_info.get("stats").unwrap_or(&Value::Null);
            
            return Some(TagInfo {
                name: challenge.get("title").and_then(|v| v.as_str()).unwrap_or(tag_name).to_string(),
                view_count: stats.get("viewCount").and_then(|v| v.as_u64()).unwrap_or(0),
                videos: vec![],
            });
        }
    }
    
    None
}
