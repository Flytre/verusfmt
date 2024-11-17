#!/bin/bash

parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

$parent_path/cleanExperimentLogs.sh
$parent_path/cleanLogFiles.sh
$parent_path/cleanTempFiles.sh