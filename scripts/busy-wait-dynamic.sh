#!/bin/bash
set -e

# includes
SCRIPTS_DIR="$(dirname "$0")"
source "$SCRIPTS_DIR/performance-scaling.sh"

# variables
RAYON_PATH="/home/jsilvafontena/lig/rayon-fork"
BENCHMARKS_PATH="/home/jsilvafontena/lig/benchmarks-rs"
RAYON_BRANCH="master-busy-wait-dynamic"
RUNS=30

build () {
  cd $RAYON_PATH && git checkout $RAYON_BRANCH
  cd $BENCHMARKS_PATH && cargo clean && cargo build --release
}

bench () {  
  THREADS=`nproc --all`
  CORES=$(($THREADS/2))

  for threads in $(seq 1 $CORES);
  do
    run $threads target/release/benchmarks output/speedup-$RAYON_BRANCH-$threads-threads.csv
  done
}

# run threads binary_name output_filename
run () {
  local threads=$1
  local binary_name=$2
  local output_filename=$3

  echo "Running $binary_name with $threads threads [$output_filename]"
  sudo RAYON_NUM_THREADS=$threads ./$binary_name --runs $RUNS --output-filename $output_filename
}

disable_performance_scaling 2.1GHz
build
bench
