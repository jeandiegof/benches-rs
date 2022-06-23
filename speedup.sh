#!/bin/bash
set -e

REF_BRANCH=master
NEW_ALGORITHM_BRANCH=new-algorithm
SLEEPING_THRESHOLD_US=10
WAITING_TIME_MULTIPLIER=2

BENCHES_DIR=`pwd`
RAYON_PATH=$BENCHES_DIR/../rayon-fork
RUNS=30

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
  CORES=`nproc --all`
  for threads in $(seq 1 $CORES);
  do
    # ref_filename="$REF_BRANCH-$threads-threads.csv"
    # run $threads 0 0 $REF_BRANCH $ref_filename

    new_filename="$NEW_ALGORITHM_BRANCH-$threads-threads-${SLEEPING_THRESHOLD_US}us-$WAITING_TIME_MULTIPLIER.csv"
    run $threads $SLEEPING_THRESHOLD_US $WAITING_TIME_MULTIPLIER $NEW_ALGORITHM_BRANCH $new_filename
  done
}

# run threads sleeping_threshold multiplier binary_name output_filename
run () {
  local threads=$1
  local sleeping_threshold=$2
  local multiplying_factor=$3
  local binary_name=$4
  local output_filename=$5

  echo "Running $binary_name with $threads threads, st = $sleeping_threshold, tm = $multiplying_factor [$output_filename]"
  sudo RAYON_NUM_THREADS=$threads \
       SLEEPING_THRESHOLD_US=$sleeping_threshold \
       WAITING_TIME_MULTIPLIER=$multiplying_factor ./$binary_name --runs $RUNS --output-filename $output_filename
}

cleanup() {
  cd $BENCHES_DIR
  rm -rf $REF_BRANCH $NEW_ALGORITHM_BRANCH
}

prepare_binaries $REF_BRANCH $NEW_ALGORITHM_BRANCH
bench
cleanup

