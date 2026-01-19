#!/bin/bash
# Download brand icons from Simple Icons CDN

CACHE_DIR="assets/icons/cache/simple-icons"
CDN="https://cdn.simpleicons.org"

echo "📥 Downloading brand icons from Simple Icons..."

# Core browsers
curl -s "$CDN/firefox" -o "$CACHE_DIR/firefox.svg"
curl -s "$CDN/googlechrome" -o "$CACHE_DIR/chrome.svg"
curl -s "$CDN/brave" -o "$CACHE_DIR/brave.svg"

# Media & Entertainment
curl -s "$CDN/spotify" -o "$CACHE_DIR/spotify.svg"
curl -s "$CDN/netflix" -o "$CACHE_DIR/netflix.svg"
curl -s "$CDN/vlcmediaplayer" -o "$CACHE_DIR/vlc.svg"
curl -s "$CDN/obsstudio" -o "$CACHE_DIR/obs.svg"

# Communication
curl -s "$CDN/discord" -o "$CACHE_DIR/discord.svg"
curl -s "$CDN/slack" -o "$CACHE_DIR/slack.svg"
curl -s "$CDN/telegram" -o "$CACHE_DIR/telegram.svg"
curl -s "$CDN/signal" -o "$CACHE_DIR/signal.svg"

# Development
curl -s "$CDN/visualstudiocode" -o "$CACHE_DIR/vscode.svg"
curl -s "$CDN/github" -o "$CACHE_DIR/github.svg"
curl -s "$CDN/docker" -o "$CACHE_DIR/docker.svg"
curl -s "$CDN/git" -o "$CACHE_DIR/git.svg"

# Creative
curl -s "$CDN/gimp" -o "$CACHE_DIR/gimp.svg"
curl -s "$CDN/inkscape" -o "$CACHE_DIR/inkscape.svg"
curl -s "$CDN/blender" -o "$CACHE_DIR/blender.svg"
curl -s "$CDN/audacity" -o "$CACHE_DIR/audacity.svg"

# Gaming
curl -s "$CDN/steam" -o "$CACHE_DIR/steam.svg"
curl -s "$CDN/epicgames" -o "$CACHE_DIR/epicgames.svg"

# Utilities
curl -s "$CDN/bitwarden" -o "$CACHE_DIR/bitwarden.svg"

echo "✅ Downloaded $(ls -1 $CACHE_DIR | wc -l) brand icons"
