#!/bin/bash
set -e

# variables
RAYON_PATH="/home/jsilvafontena/lig/rayon-fork"
BENCHMARKS_PATH="/home/jsilvafontena/lig/benchmarks-rs"
RAYON_BRANCH="master"
RUNS=50

build () {
  cd $RAYON_PATH && git checkout $RAYON_BRANCH
  cd $BENCHMARKS_PATH && cargo clean && cargo build --release
}

bench () {  
  mkdir -p output
  THREADS=`nproc --all`

  for nb_threads in $(seq 1 $THREADS);
  do
    run $nb_threads target/release/benchmarks output/speedup-$RAYON_BRANCH-$nb_threads-threads.csv
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

build
bench
