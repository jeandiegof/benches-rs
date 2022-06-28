#!/bin/bash
set -e

# includes
SCRIPTS_DIR="$(dirname "$0")"
source "$SCRIPTS_DIR/performance-scaling.sh"

# variables
RAYON_PATH="$SCRIPTS_DIR/../../rayon-fork"
RAYON_BRANCH="new-algorithm"

SLEEPING_THRESHOLDS=(500)
WAITING_TIME_MULTIPLIER=(2)
RUNS=50

build () {
  cd $RAYON_PATH && git checkout $RAYON_BRANCH
  cd $SCRIPTS_DIR/../ && cargo clean && cargo build --release
}

bench () {  
  THREADS=`nproc --all`
  CORES=$(($THREADS/2))
  for threads in $(seq 1 $CORES);
  do
    for st in ${SLEEPING_THRESHOLDS[@]}; do
      for wt in ${WAITING_TIME_MULTIPLIER[@]}; do
        run $threads $st $wt target/release/benchmarks output/speedup-$threads-threads-$st-us-$wt.csv
      done
    done
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

disable_performance_scaling 2.1GHz
build
bench

