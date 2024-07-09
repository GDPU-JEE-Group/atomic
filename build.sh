#!/bin/bash

run_cmd() {
    echo "Executing: $*"
    "$@"
    local status=$?
    if [ $status -ne 0 ]; then
        echo "Error: Command failed with status $status"
        exit $status
    fi
}

package_project(){
    # clean
    rm -rf target/atomic-amd64-linux ./atomic-amd64-linux.tar ./atomic-amd64-linux.tar.sha256

    # compile
    # run_cmd export RUSTFLAGS="-C target-feature=+crt-static"
    run_cmd cargo build --release --verbose

    # tar and package
    run_cmd mkdir -p target/atomic-amd64-linux
    run_cmd mv target/release/server target/atomic-amd64-linux/
    run_cmd mv target/release/client target/atomic-amd64-linux/
    run_cmd cp config/app_config.toml target/atomic-amd64-linux/
    run_cmd cp ./README.md target/atomic-amd64-linux/
    run_cmd pwd
    run_cmd tree target/atomic-amd64-linux
    run_cmd tar cvf ./atomic-amd64-linux.tar target/atomic-amd64-linux
    
    # sha256
    echo "shasum -a 256 ./atomic-amd64-linux.tar | cut -d ' ' -f 1 > ./atomic-amd64-linux.tar.sha256"
    shasum -a 256 ./atomic-amd64-linux.tar | cut -d ' ' -f 1 > ./atomic-amd64-linux.tar.sha256

}

main(){
    package_project
}

main

