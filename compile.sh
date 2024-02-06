#!/bin/bash

ARCH=$(uname -m)-linux
echo "---------------------------------------------------------------------------------"
echo Getting release version.
RELEASE_VERSION=$(git describe --tag | sed s/v// | sed -E s/[\.]/\\\\\./g | awk -F \- '{print $1}')
echo "---------------------------------------------------------------------------------"
echo Updating version in toml file.
sed -i -E "0,/^version.*$/{s/^version.*$/version = '${RELEASE_VERSION}'/}" Cargo.toml
FILE_VERSION=$(git describe --tag | awk -F \- '{print $1}')
echo $FILE_VERSION > quit-version.txt
echo "---------------------------------------------------------------------------------"
echo Building portal quit software for host architecture.
echo "---------------------------------------------------------------------------------"
cargo build --release
if [ $? -eq 0 ]; then
    echo "---------------------------------------------------------------------------------"
    echo Portal quit software successfully compiled for host architecture.
    echo "---------------------------------------------------------------------------------"
else
    echo "---------------------------------------------------------------------------------"
    echo Unable to compile portal quit software for host architecture.
    echo "---------------------------------------------------------------------------------"
    exit 1
fi;
git stash
git stash drop
echo Creating tar
echo "---------------------------------------------------------------------------------"
tar -cvf ${ARCH}-${FILE_VERSION}.tar -C target/release/ chronokeep-portal-quit ../../quit-version.txt
rm quit-version.txt
gzip ${ARCH}-${FILE_VERSION}.tar
cp ${ARCH}-${FILE_VERSION}.tar.gz ~/