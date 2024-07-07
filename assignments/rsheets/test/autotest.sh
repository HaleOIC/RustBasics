#!/bin/bash

# Record six stages
fail_times=0
succ_times=0

### Stage 1
stage_total=9
succ_times1=0
fail_times1=0
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

for i in $(seq 1 "$stage_total"); do
  cargo run -- --mark-mode < "./stage1/$i.in" >  "./stage1/$i.tmp"
  if diff -y "./stage1/$i.out" "./stage1/$i.tmp"; then
    echo -e "${GREEN}Stage 1 Test $i passed.${NC}"
    succ_times=$((succ_times + 1))
    succ_times1=$((succ_times1 + 1))
  else
    echo -e "${RED}Stage 1 Test $i failed.${NC}"
    fail_times=$((fail_times + 1))
    fail_times1=$((fail_times1 + 1))
  fi

  # remove intermediate file
  rm -rf "./stage1/$i.tmp"
done

echo -e "${GREEN}Stage 1: $succ_times passed, ${RED}$fail_times failed${NC}"
