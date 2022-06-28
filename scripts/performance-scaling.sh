#!/bin/bash
set -e

# set_cpu_scaling_governor [performance, ...]
set_cpu_scaling_governor() {
  sudo cpupower frequency-set -g $1
}

# set_cpu_frequency frequency
set_cpu_frequency() {
  sudo cpupower frequency-set --min $1 --max $1
}

# disable_all_idle_states
disable_all_idle_states() {
  idle_states=`cpupower idle-info | grep 'Number of idle states:' | awk -F': ' '{print $2}'`

  for i in $(seq 0 $(($idle_states-1)));
  do
    sudo cpupower idle-set --disable $i
  done
}

# disable_performance_scaling target_freq
disable_performance_scaling() {
  target_frequency=$1
  
  set_cpu_scaling_governor performance
  set_cpu_frequency $target_frequency
  disable_all_idle_states
}
