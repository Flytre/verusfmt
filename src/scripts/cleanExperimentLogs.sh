#!/bin/bash

parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )


echo "removing experiment log files..."
rm -f $parent_path/../../experiment_logs/*
echo "[done]"
