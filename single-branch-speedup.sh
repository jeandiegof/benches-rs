#!/bin/bash
set -e

SLEEPING_THRESHOLDS=(10 50 100 500 1000 10000 100000)
WAITING_TIME_MULTIPLIER=2

BENCHES_DIR=`pwd`
RAYON_PATH=$BENCHES_DIR/../rayon-fork
RUNS=50

bench () {  
  CORES=`nproc --all`
  for threads in $(seq 1 $CORES);
  do
    for st in ${SLEEPING_THRESHOLDS[@]}; do
      run $threads $st $WAITING_TIME_MULTIPLIER target/release/benchmarks output/speedup-$threads-threads-$st-us.csv
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

bench

