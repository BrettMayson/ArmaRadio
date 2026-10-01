#!/usr/bin/env bash
# Author: Joncantplay
# Run in MSYS2 with the x64 MSVC environment inherited from the workflow.
set -euo pipefail
RADIO_FFMPEG_VERSION=8.1.3
RADIO_ROOT=$(pwd)
mkdir -p .codec-build resources/codecs output/ffmpeg-source output/codec
curl --fail --location --retry 3 "https://ffmpeg.org/releases/ffmpeg-${RADIO_FFMPEG_VERSION}.tar.xz" -o .codec-build/ffmpeg-source.tar.xz
tar -xf .codec-build/ffmpeg-source.tar.xz -C .codec-build
# MSYS link.exe conflicts with the Microsoft linker.
if [ -f /usr/bin/link.exe ]; then mv /usr/bin/link.exe /usr/bin/link-msys.exe; fi
command -v cl.exe
cd ".codec-build/ffmpeg-${RADIO_FFMPEG_VERSION}"
RADIO_CONFIGURE=(
    --toolchain=msvc --arch=x86_64 --target-os=win64
    --disable-autodetect --disable-debug --disable-doc --disable-everything
    --disable-programs --enable-ffmpeg --disable-shared --enable-static
    --disable-gpl --disable-nonfree --enable-small --enable-schannel
    --enable-protocol=http,https,tcp,tls,crypto,pipe,file
    --enable-demuxer=aac,hls,mpegts,mov,mp3
    --enable-parser=aac,aac_latm,mpegaudio
    --enable-decoder=aac,aac_latm,mp3
    --enable-muxer=pcm_f32le --enable-encoder=pcm_f32le
    --enable-filter=aresample,aformat,anull,abuffer,abuffersink,atrim
)
printf '%q ' ./configure "${RADIO_CONFIGURE[@]}" > "$RADIO_ROOT/output/ffmpeg-source/FFmpeg-build.txt"
printf '\nUnmodified FFmpeg %s; MSVC x64; LGPL-2.1-or-later.\n' "$RADIO_FFMPEG_VERSION" >> "$RADIO_ROOT/output/ffmpeg-source/FFmpeg-build.txt"
./configure "${RADIO_CONFIGURE[@]}"
make -j2
cp ffmpeg.exe "$RADIO_ROOT/resources/codecs/ffmpeg.exe"
cp ffmpeg.exe "$RADIO_ROOT/output/codec/ffmpeg.exe"
cp COPYING.LGPLv2.1 "$RADIO_ROOT/output/ffmpeg-source/FFmpeg-LICENSE.txt"
cp "$RADIO_ROOT/.codec-build/ffmpeg-source.tar.xz" "$RADIO_ROOT/output/ffmpeg-source/ffmpeg-source.tar.xz"
"$RADIO_ROOT/resources/codecs/ffmpeg.exe" -version
