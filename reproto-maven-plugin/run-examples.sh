#!/bin/bash

set -e

root=$(realpath $(dirname $0))

(cd .. && cargo build --release)

exe=$(realpath $root/../target/release/reproto)
examples=$(realpath $root/../examples)

if [[ ! -x $exe ]]; then
    echo "Not an executable: $exe"
    exit 1
fi

if [[ ! -d $examples ]]; then
    echo "Not a directory: $examples"
    exit 1
fi

echo "Installing Plugin"
mvn -q install

for dir in $PWD/examples/*; do
    pushd $dir

    target=$dir/src/main/reproto

    if [[ ! -x $target ]]; then
        echo "Linking $examples -> $target"
        ln -s $examples $target
    fi

    echo "Building: $dir"
    mvn -q clean package -D reproto.executable=$exe
    mvn -q dependency:build-classpath -D mdep.outputFile=target/classpath

    classpath=$(cat target/classpath):target/classes

    echo "Running: $dir"
    java -cp $classpath se.tedro.tests.App

    popd
done

exit 0
