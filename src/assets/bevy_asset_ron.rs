//! MIT License
//! Copyright (c) 2021 Jasen Borisov
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//!
//!
//! Easily register custom data to be loaded as bevy assets from RON files
//!
//! Caveat: you need a different file name extension for each asset type.
//!
//! Create your custom asset types as follows:
//!

use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::BoxedFuture;
use serde::Deserialize;
use std::marker::PhantomData;

struct RonLoader<T> {
    extensions: Vec<&'static str>,
    _t: PhantomData<fn() -> T>,
}

impl<T> AssetLoader for RonLoader<T>
where
    for<'de> T: TypeUuid + Deserialize<'de> + Send + Sync + 'static,
{
    fn extensions(&self) -> &[&str] {
        &self.extensions
    }

    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let loaded = ron::de::from_bytes::<T>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(loaded));
            Ok(())
        })
    }
}

/// Plugin to register a single asset type into your App.
///
/// Create and register as many instances as you need.
pub struct RonAssetPlugin<T> {
    extensions: Vec<&'static str>,
    _t: PhantomData<fn() -> T>,
}

impl<T> Plugin for RonAssetPlugin<T>
where
    for<'de> T: TypeUuid + Deserialize<'de> + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        let loader = RonLoader::<T> {
            extensions: self.extensions.clone(),
            _t: PhantomData,
        };
        app.add_asset::<T>().add_asset_loader(loader);
    }
}

impl<T> RonAssetPlugin<T> {
    /// Create a new plugin instance for a custom RON asset type.
    ///
    /// Files with the provided `extensions` will be loaded by bevy as an asset
    /// of the provided type `T`, using serde with the RON deserializer.
    ///
    /// `T` must implement `serde::Deserialize` and `bevy::reflect::TypeUuid`.
    /// Both of these traits can be derived.
    pub fn new(extensions: &[&'static str]) -> Self {
        Self {
            extensions: extensions.to_owned(),
            _t: PhantomData,
        }
    }
}
