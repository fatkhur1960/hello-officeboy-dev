#!/usr/bin/env bash
set -e

echo "Compiling APF..."

if [ "$1" = "compile" ]; then

        pushd ..

        docker run -it --rm -v $(pwd):/workdir \
                -v /tmp:/root/.cargo/git \
                -v /tmp:/root/.cargo/registry \
                anvie/rust-musl-build:latest \
                cargo build --release --target=x86_64-unknown-linux-musl

        cp target/x86_64-unknown-linux-musl/release/apf_server docker

        popd

else
        echo "Downloading..."
        wget -O ./apf_server "http://178.128.219.222/linux-x86_64-musl/apf_server-nightly-latest?"$(date +"%s")
fi

echo "Build SQL init script..."
# Build init.sql 
python build_init_sql.py

echo "Build docker image..."
docker build . -t apf_server

echo ""
echo ""
echo "Sekarang kamu bisa menjalankan docker compose:"
echo ""
echo "   $ docker-compose up"
echo ""
