#!/usr/bin/env bash

echo "Compiling APF..."
pushd ..

docker run -it --rm -v $(pwd):/workdir \
        -v /tmp:/root/.cargo/git \
        -v /tmp:/root/.cargo/registry \
        anvie/rust-musl-build:latest \
        cargo build --release --target=x86_64-unknown-linux-musl

popd

echo "Build SQL init script..."
# Build init.sql 
python build_init_sql.py

cp ../target/x86_64-unknown-linux-musl/release/apf_server ./

echo "Build docker image..."
docker build . -t apf_server

echo ""
echo ""
echo "Sekarang kamu bisa menjalankan docker compose:"
echo ""
echo "   $ docker-compose up"
echo ""