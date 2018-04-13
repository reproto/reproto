#!/usr/bin/env bash

set -e

BASE_URL=${BASE_URL:-"https://storage.googleapis.com/reproto-releases"}
REPROTO_HOME=${REPROTO_HOME:-"$HOME/.reproto"}

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

echo "NOTICE: This will install reproto into $REPROTO_HOME"

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

mkdir -p $REPROTO_HOME/bin
mkdir -p $REPROTO_HOME/releases

versioned=reproto-$version-$platform-$arch
archive="$BASE_URL/$versioned.tar.gz"

# archive
a="$REPROTO_HOME/releases/$versioned.tar.gz"
# unpacked binary
b="$REPROTO_HOME/releases/$versioned"
# link
l="$REPROTO_HOME/bin/reproto"
# link destination
d="../releases/$versioned"

if [[ -f $a ]]; then
    if [[ $force != "y" ]]; then
        echo "Archive already exist: $a"
        exit 1
    fi

    echo "Removing old archive (-f): $a"
    rm -f $a
fi

if [[ -f $b ]]; then
    if [[ $force != "y" ]]; then
        echo "Binary already exist: $b"
        exit 1
    fi

    echo "Removing old binary (-f): $b"
    rm -f $b
fi

if [[ -L $l ]]; then
    if [[ $force != "y" ]]; then
        echo "Link already exist: $l"
        exit 1
    fi

    echo "Removing old link (-f): $l"
    rm -f $l
fi

download "$archive" "$a"
tar -C "$REPROTO_HOME/releases" -xf $a

mv "$REPROTO_HOME/releases/reproto" "$b"
chmod +x $b
ln -s $d $l

echo ""
echo "All done!"
echo ""
echo "Please make sure that $REPROTO_HOME/bin is in your PATH, or that REPROTO_HOME is set and points to $REPROTO_HOME"
