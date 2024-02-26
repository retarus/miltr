#!/bin/bash
set -ex 

cargo login

cd utils
cargo publish
sleep 20

cd common
cargo publish
sleep 20

cd server
cargo publish
sleep 20

cd client
cargo publish
sleep 20
