use std::{fs::read_to_string, path::Path};

use anyhow::Result;
use wgpu::{Device, Queue};

use crate::texture;

pub async fn load_string(file_name: &str) -> Result<String> {
    let path = Path::new(env!("OUT_DIR")).join("res").join(file_name);
    let txt = std::fs::read_to_string(path)?;
    Ok(txt)
}

pub async fn load_binary(file_name: &str) -> Result<Vec<u8>> {
    let path = Path::new(env!("OUT_DIR")).join("res").join(file_name);
    let txt = std::fs::read(path)?;
    Ok(txt)
}

pub async fn load_texture(
    file_name: &str,
    device: &Device,
    queue: &Queue,
) -> Result<texture::Texture> {
    let data = load_binary(file_name).await?;
    texture::Texture::from_bytes(device, queue, &data, file_name)
}
