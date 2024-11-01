#!/bin/bash

parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )


echo "removing temp log files..."
rm -f $parent_path/../logs/*.log
echo "[done]"
