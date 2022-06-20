#!/bin/bash
set -e
set -x

REF_BRANCH=master
NEW_ALGORITHM_BRANCH=new-algorithm
BENCHES_DIR=`pwd`
RAYON_PATH=$BENCHES_DIR/../rayon-fork
RUNS=50

# build_benches branch-name
build_benches () {  
  cd $RAYON_PATH && git checkout $1
  cd $BENCHES_DIR && cargo clean

  cargo build --release
  cp ./target/release/benchmarks $BENCHES_DIR/$1
}

# prepare_binaries branch1 branch2
prepare_binaries () {
  build_benches $1
  build_benches $2
}

bench () {  
  for i in {{1..9},{10..99..10},{100..1000..100},{2000..10000..1000}};
  do
    run $i $NEW_ALGORITHM_BRANCH $NEW_ALGORITHM_BRANCH-$i-us.csv
    if [[ $i -eq 50 ]]
    then
      run -1 $REF_BRANCH $REF_BRANCH.csv
    fi
  done
}

# run sleeping_threshold_us binary_name output_filename
run () {
  echo "Running $2 with SLEEPING_THRESHOLD_US=$1 [$3]"
  sudo SLEEPING_THRESHOLD_US=$1 ./$2 --runs $RUNS --output-filename $3
}

cleanup() {
  cd $BENCHES_DIR
  rm -rf $REF_BRANCH $NEW_ALGORITHM_BRANCH
}

prepare_binaries $REF_BRANCH $NEW_ALGORITHM_BRANCH
bench
cleanup

