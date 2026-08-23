#!/bin/bash

# Make new iconset
echo "Making iconset..."
mkdir assets/pausemouse.iconset
sips -z 16 16     assets/app.png --out assets/pausemouse.iconset/icon_16x16.png
sips -z 32 32     assets/app.png --out assets/pausemouse.iconset/icon_16x16@2x.png
sips -z 32 32     assets/app.png --out assets/pausemouse.iconset/icon_32x32.png
sips -z 64 64     assets/app.png --out assets/pausemouse.iconset/icon_32x32@2x.png
sips -z 128 128   assets/app.png --out assets/pausemouse.iconset/icon_128x128.png
sips -z 256 256   assets/app.png --out assets/pausemouse.iconset/icon_128x128@2x.png
sips -z 256 256   assets/app.png --out assets/pausemouse.iconset/icon_256x256.png
sips -z 512 512   assets/app.png --out assets/pausemouse.iconset/icon_256x256@2x.png
sips -z 512 512   assets/app.png --out assets/pausemouse.iconset/icon_512x512.png
sips -z 1024 1024 assets/app.png --out assets/pausemouse.iconset/icon_512x512@2x.png
iconutil -c icns assets/pausemouse.iconset -o assets/AppIcon.icns
rm -rf assets/pausemouse.iconset

# Exit instantly if any individual command fails
set -e

echo "Building Pausemouse in release mode..."
cargo build --release

echo "Preparing Bundle Directories inside target/bundle/..."
BUNDLE_DIR="target/bundle/osx/Pausemouse.app"
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR/Contents/MacOS"
mkdir -p "$BUNDLE_DIR/Contents/Resources"

echo "Copying binary and metadata into position..."
cp target/release/pausemouse "$BUNDLE_DIR/Contents/MacOS/"
cp Info.plist "$BUNDLE_DIR/Contents/"

if [ -f "assets/AppIcon.icns" ]; then
    cp assets/AppIcon.icns "$BUNDLE_DIR/Contents/Resources/"
    echo "AppIcon attached successfully."
fi

touch "$BUNDLE_DIR"

echo "Success! Application is ready at: $BUNDLE_DIR"
