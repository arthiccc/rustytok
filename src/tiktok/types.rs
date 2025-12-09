use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub bio: String,
    pub avatar_url: String,
    pub follower_count: u64,
    pub following_count: u64,
    pub like_count: u64,
    pub video_count: u64,
    pub videos: Vec<VideoInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub description: String,
    pub author_username: String,
    pub author_nickname: String,
    pub author_avatar: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub like_count: u64,
    pub comment_count: u64,
    pub share_count: u64,
    pub view_count: u64,
    pub create_time: i64,
    pub music_title: Option<String>,
    pub music_author: Option<String>,
}

impl VideoInfo {
    /// Get proxied video URL
    pub fn proxied_video_url(&self) -> String {
        format!("/proxy?url={}", urlencoding::encode(&self.video_url))
    }
    
    /// Get proxied thumbnail URL  
    pub fn proxied_thumbnail_url(&self) -> String {
        format!("/proxy?url={}", urlencoding::encode(&self.thumbnail_url))
    }
}

impl UserInfo {
    /// Get proxied avatar URL
    pub fn proxied_avatar_url(&self) -> String {
        format!("/proxy?url={}", urlencoding::encode(&self.avatar_url))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    pub name: String,
    pub view_count: u64,
    pub videos: Vec<VideoInfo>,
}
