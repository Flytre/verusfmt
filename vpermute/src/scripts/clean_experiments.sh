#!/bin/bash


parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

echo "cleaning experiment output.."
rm -r $parent_path/../../../permutation_experiments/*

echo "[done]"

echo "cleaning SMarTPeek logs and output.."
$parent_path/../../../src/scripts/clean_all.sh



