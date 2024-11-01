#!/bin/bash


parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )


echo "removing formatted files..."
rm -f $parent_path/../../tempFiles/*"_formatted_"*.rs
echo "[done]"
