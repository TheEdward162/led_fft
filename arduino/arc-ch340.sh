#!/bin/sh

FQBN="arduino:avr:uno"
PORT="/dev/ttyUSB0"

help() {
	local HELP="arduino-cli (arc) for ch340
Commands:
	c, compile	sketch
	u, upload	sketch
"

	echo "$HELP"
}

compile() {
	if [ "$#" -lt 1 ]; then
		help
		exit 1
	fi

	local sketch="$1"
	local warnings=${2:-default}

	arduino-cli compile --warnings "$warnings" --fqbn "$FQBN" "$sketch"
}

upload() {
	if [ "$#" -lt 1 ]; then
		help
		exit 1
	fi

	local sketch="$1"
	arduino-cli upload --verbose --verify --port "$PORT" --fqbn "$FQBN" "$sketch"
}

case "$1" in
	c|compile)
		shift 1
		compile "$@"
	;;

	u|upload)
		shift 1
		upload "$@"
	;;

	cu|compload)
		shift 1
		"$0" compile "$@" && "$0" upload "$@"
	;;

	*)
		help
	;;
esac

