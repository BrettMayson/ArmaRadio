#!/usr/bin/env python3
"""Exercise the packaged decoder against local HTTP audio and HLS inputs."""
import functools
import http.server
import math
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import threading
import urllib.request


def main():
    decoder = str(Path(sys.argv[1]).resolve())
    fixture = Path(__file__).resolve().parents[1] / 'tests/fixtures/aac_lc_stereo.adts'
    with tempfile.TemporaryDirectory(prefix='live-radio-codec-test-') as folder:
        root = Path(folder)
        root.joinpath('audio.aac').write_bytes(fixture.read_bytes())
        root.joinpath('playlist.m3u8').write_text(
            '#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:2\n'
            '#EXT-X-MEDIA-SEQUENCE:0\n#EXTINF:1.0,\naudio.aac\n#EXT-X-ENDLIST\n')
        root.joinpath('master.m3u8').write_text(
            '#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=128000,CODECS="mp4a.40.2"\nplaylist.m3u8\n')
        cases = ['audio.aac', 'playlist.m3u8', 'master.m3u8']
        if '--fetch-he' in sys.argv:
            # Official FFmpeg FATE samples; retained only in the temporary test directory.
            for filename in ('al_sbr_cm_48_2.mp4', 'al_sbr_ps_04_new.mp4'):
                with urllib.request.urlopen('https://fate-suite.ffmpeg.org/aac/' + filename, timeout=30) as response:
                    root.joinpath(filename).write_bytes(response.read())
                cases.append(filename)
        class Handler(http.server.SimpleHTTPRequestHandler):
            def log_message(self, *_args):
                pass
        server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), functools.partial(Handler, directory=folder))
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            for case in cases:
                url = f'http://127.0.0.1:{server.server_port}/{case}'
                result = subprocess.run([
                    decoder, '-hide_banner', '-nostdin', '-loglevel', 'error',
                    '-rw_timeout', '15000000', '-protocol_whitelist', 'http,https,tcp,tls,crypto',
                    '-re', '-i', url, '-map', '0:a:0', '-vn', '-sn', '-dn', '-t', '0.25',
                    '-ac', '1', '-ar', '48000', '-acodec', 'pcm_f32le', '-f', 'f32le', 'pipe:1',
                ], capture_output=True, timeout=30)
                if result.returncode:
                    raise RuntimeError(f'{case}: {result.stderr.decode(errors="replace")}')
                assert len(result.stdout) >= 19200, f'{case}: insufficient decoded audio'
                samples = struct.unpack('<' + 'f' * (len(result.stdout) // 4), result.stdout)
                peak = max(abs(x) for x in samples)
                print(
                        f'DEBUG {case}: '
                        f'samples={len(samples)}, '
                        f'min={min(samples):.8f}, '
                        f'max={max(samples):.8f}, '
                        f'peak={peak:.8f}'
                )

                assert all(math.isfinite(x) for x in samples), f'{case}: invalid PCM'
                assert peak > 1e-5, f'{case}: silent output'
                print(f'PASS {case}: {len(samples)} mono PCM samples at 48000 Hz')
        finally:
            server.shutdown()
            server.server_close()
            thread.join()


if __name__ == '__main__':
    main()
