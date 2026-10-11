#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
docs_dir="$repo_root/docs"
port="${PLUMB_CODE_TOUR_PORT:-8765}"
url="http://localhost:${port}/code-tour.html"

python3 -m http.server "$port" --bind 127.0.0.1 --directory "$docs_dir" &
server_pid=$!
trap 'kill "$server_pid" 2>/dev/null || true' EXIT INT TERM

for _ in {1..30}; do
  if ! kill -0 "$server_pid" 2>/dev/null; then
    wait "$server_pid"
    exit $?
  fi
  if python3 -c 'import sys, urllib.request; urllib.request.urlopen(sys.argv[1], timeout=0.2)' "$url" >/dev/null 2>&1; then
    break
  fi
  sleep 0.1
done

printf 'Plumb Code Tour: %s\n' "$url"
printf 'Press Ctrl+C to stop the local server.\n'

if command -v wslview >/dev/null 2>&1; then
  wslview "$url" >/dev/null 2>&1 || true
elif [[ -n "${BROWSER:-}" ]] && command -v "$BROWSER" >/dev/null 2>&1; then
  "$BROWSER" "$url" >/dev/null 2>&1 || true
else
  for browser in firefox chromium chromium-browser google-chrome; do
    if command -v "$browser" >/dev/null 2>&1; then
      "$browser" "$url" >/dev/null 2>&1 &
      break
    fi
  done
fi

wait "$server_pid"
