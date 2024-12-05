#!/bin/bash


parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

echo "cleaning experiment output.."
rm -r $parent_path/../../../permutation_experiments/*
echo "[done]"