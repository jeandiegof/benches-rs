#!/bin/bash
set -e
# set -x

REF_BRANCH=master
NEW_ALGORITHM_BRANCH=new-algorithm
BENCHES_DIR=`pwd`
RAYON_PATH=$BENCHES_DIR/../rayon-fork
RUNS=30

# build_benches branch-name
build_benches () {  
  cd $RAYON_PATH && git checkout $1
  cd $BENCHES_DIR && cargo clean

  cargo build --release
  cp ./target/release/benches $BENCHES_DIR/$1
}

# prepare_binaries branch1 branch2
prepare_binaries () {
  build_benches $1
  build_benches $2
}

bench () {  
  CORES=`nproc --all`
  for i in $(seq 1 $CORES);
  do
    run $i $REF_BRANCH $REF_BRANCH-$i-threads.csv
    run $i $NEW_ALGORITHM_BRANCH $NEW_ALGORITHM_BRANCH-$i-threads-10us.csv
  done
}

# run threads binary_name output_filename
run () {
  echo "Running $2 with $1 threads [$3]"
  sudo RAYON_NUM_THREADS=$1 SLEEPING_THRESHOLD_US=10 ./$2 --runs $RUNS --output-filename $3
}

cleanup() {
  cd $BENCHES_DIR
  rm -rf $REF_BRANCH $NEW_ALGORITHM_BRANCH
}

prepare_binaries $REF_BRANCH $NEW_ALGORITHM_BRANCH
bench
cleanup

