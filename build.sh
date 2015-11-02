#!/bin/sh

set -x
set -e

if [ "$GTK" = latest ]; then
	BUNDLE="gtk-3.18.1-2"
	FEATURES="gtk_3_16 opengl"
fi

if [ -n "$BUNDLE" ]; then
	WD="$PWD"
	cd "$HOME"
	curl -LO "https://github.com/gkoz/gtk-bootstrap/releases/download/$BUNDLE/deps.txz"
	tar xf deps.txz
	cd "$WD"
	export PKG_CONFIG_PATH="$HOME/local/lib/pkgconfig"
fi

cargo build --features "$FEATURES"
