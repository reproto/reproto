#!/usr/bin/env bash

set -e

BASE_URL=${BASE_URL:-"https://storage.googleapis.com/reproto-releases"}
DATA_HOME=${XDG_DATA_HOME:-"$HOME/.local/share"}
BIN_HOME="$HOME/.local/bin"

releases=$BASE_URL/releases
noninteractive=n
force=n

while [[ $# > 0 ]]; do
    case "$1" in
    -f)
        force=y
        shift 1
        ;;
    --noninteractive)
        noninteractive=y
        shift 1
        ;;
    --help|-h)
        echo "Usage: install.sh [-f] [--noninteractive]"
        echo ""
        echo "This is the reproto installer, downloaded from:"
        echo "https://github.com/reproto/reproto"
        echo ""
        echo "  -f               : overwrite existing installation if it exists"
        echo "  --noninteractive : do not prompt before installing"
        echo "  -h (--help)      : show this help"
        echo ""
        echo "Environment variables:"
        echo "  REPROTO_HOME: points to where reproto should be installed (default: $HOME/.reproto)"
        exit 0
        ;;
    *)
        echo "unrecognized option: $1"
        exit 1
        ;;
    esac
done

test_cmd() {
    command -v "$1" > /dev/null 2>&1
    return $?
}

detect_platform() {
    case "$(uname -s)" in
    Linux) echo linux ;;
    Darwin) echo osx ;;
    *) return 1 ;;
    esac
}

detect_arch() {
    case $(uname -m) in
    x86_64 | x86-64 | x64 | amd64) echo x86_64 ;;
    *) return 1 ;;
    esac
}

get_version() {
    if test_cmd curl; then
        version=$(curl $releases | head -n 1)
        echo $version
        return 0
    fi

    if test_cmd wget; then
        version=$(wget -q -O - $releases | head -n 1)
        echo $version
        return 0
    fi

    return 1
}

download() {
    if test_cmd curl; then
        curl -sSfL "$1" -o "$2"
        return 0
    fi

    if test_cmd wget; then
        wget "$1" -O "$2"
        return 0
    fi

    return 1
}

if ! platform=$(detect_platform); then
    echo "failed to detect platform"
    exit 1
fi

if ! arch=$(detect_arch); then
    echo "failed to detect arch"
    exit 1
fi

if ! version=$(get_version); then
    echo "failed to get latest version"
    exit 1
fi

echo "NOTICE: This will install reproto into $BIN_HOME"

while true; do
    read -p "Are you sure? [Yn]" -n 1 -r

    if [[ $REPLY =~ ^[Nn]$ ]]; then
        echo "Aborting"
        exit 0
    fi

    if [[ -z $REPLY || $REPLY =~ ^[Yy]$ ]]; then
        break
    fi
done

mkdir -p $BIN_HOME
mkdir -p $DATA_HOME/releases

versioned=reproto-$version-$platform-$arch
archive="$BASE_URL/$versioned.tar.gz"

# archive
a="$DATA_HOME/releases/$versioned.tar.gz"
# unpacked binary
b="$DATA_HOME/releases/$versioned"
# link
l="$BIN_HOME/reproto"
# link destination
d="$DATA_HOME/releases/$versioned"

do_download=y
do_unpack=y
do_copy=y

if [[ -f $a ]]; then
    if [[ $force == "y" ]]; then
        echo "Removing old archive (-f): $a"
        rm -f $a
    else
        do_download=n
    fi
fi

if [[ -f $b ]]; then
    if [[ $force == "y" ]]; then
        echo "Removing old binary (-f): $b"
        rm -f $b
    else
        # no need to download since binary already exists
        do_unpack=n
    fi
fi

if [[ -L $l ]]; then
    if [[ $force == "y" ]]; then
        echo "Removing old link (-f): $l"
        rm -f $l
    else
        do_copy=n
    fi
fi

if [[ $do_download == "y" ]]; then
    echo "Downloading archive: $archive"
    download "$archive" "$a"
fi

if [[ $do_unpack == "y" ]]; then
    echo "Unpacking archive: $archive"
    tar -C "$DATA_HOME/releases" -xf $a
    mv "$DATA_HOME/releases/reproto" "$b"
    chmod +x $b
fi

if [[ $do_copy == "y" ]]; then
    echo "Creating link: $l -> $d"
    cp -f $d $l
fi

echo ""
echo "All done!"
echo ""
echo "Please make sure that $BIN_HOME is in your PATH, or that REPROTO_HOME is set and points to $BIN_HOME"
