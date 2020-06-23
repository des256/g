// G
// Desmond Germans, 2020

mod video;
pub use video::*;

mod pixel;
pub use pixel::*;

mod image;
pub use image::*;

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
pub use linux::*;
#[cfg(target_os="linux")]
mod opengl45;
#[cfg(target_os="linux")]
pub use opengl45::*;
#[cfg(target_os="linux")]
mod alsa;
#[cfg(target_os="linux")]
pub use ::alsa::*;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::*;
#[cfg(target_os="windows")]
mod opengl45;
#[cfg(target_os="windows")]
pub use opengl45::*;
#[cfg(target_os="windows")]
mod directsound;
#[cfg(target_os="windows")]
pub use directsound::*;

#[cfg(target_os="macos")]
mod macos;
#[cfg(target_os="macos")]
pub use macos::*;
#[cfg(target_os="macos")]
mod opengl45;
#[cfg(target_os="macos")]
pub use opengl45::*;
// TODO: audio for MacOS

#[cfg(target_os="android")]
mod android;
#[cfg(target_os="android")]
pub use android::*;
#[cfg(target_os="android")]
mod gles20;
#[cfg(target_os="android")]
pub use gles20::*;
// TODO: audio for Android

#[cfg(target_os="ios")]
mod ios;
#[cfg(target_os="ios")]
pub use ios::*;
#[cfg(target_os="ios")]
mod gles20;
#[cfg(target_os="ios")]
pub use gles20::*;
// TODO: audio for iOS

#[cfg(target_arch="wasm32")]
mod web;
#[cfg(target_arch="wasm32")]
pub use web::*;
#[cfg(target_arch="wasm32")]
mod webgl1;
#[cfg(target_arch="wasm32")]
pub use webgl1::*;
#[cfg(target_arch="wasm32")]
mod webaudio;
#[cfg(target_arch="wasm32")]
pub use webaudio::*;
