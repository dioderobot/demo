#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BOARD="$SCRIPT_DIR/../layout/layout.kicad_pcb"
PRESET="$SCRIPT_DIR/dm0002-panel-4x5-mousebites.json"
OUTPUT="$SCRIPT_DIR/dm0002-panel-4x5-mousebites.kicad_pcb"

kikit panelize \
  --preset "$PRESET" \
  "$BOARD" \
  "$OUTPUT"
