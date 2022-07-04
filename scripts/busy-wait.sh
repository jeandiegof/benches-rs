#!/bin/bash
set -e

# includes
SCRIPTS_DIR="$(dirname "$0")"
source "$SCRIPTS_DIR/performance-scaling.sh"

# variables
RAYON_PATH="/home/jsilvafontena/lig/rayon-fork"
BENCHMARKS_PATH="/home/jsilvafontena/lig/benchmarks-rs"
RAYON_BRANCH="master-busy-wait"

BUSY_WAIT_CYCLES=(10 100 1000 10000 100000 1000000)
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
    for busy_cycles in ${BUSY_WAIT_CYCLES[@]}; do
      run $threads $busy_cycles target/release/benchmarks output/speedup-$RAYON_BRANCH-$threads-threads-$busy_cycles-cycles.csv
    done
  done
}

# run threads busy_cycles binary_name output_filename
run () {
  local threads=$1
  local busy_cycles=$2
  local binary_name=$3
  local output_filename=$4

  echo "Running $binary_name with $threads threads and $busy_cycles busy wait cycles [$output_filename]"
  sudo RAYON_NUM_THREADS=$threads \
       BUSY_WAIT_CYCLES=$busy_cycles ./$binary_name --runs $RUNS --output-filename $output_filename
}

disable_performance_scaling 2.1GHz
build
bench
