#!/bin/bash

folder_name=$1

if [[ -z $folder_name ]]; then
    read -p 'Name: ' folder_name
fi

if [[ -d $folder_name ]]; then
    echo $folder_name already exists
    exit 1
fi

cargo new --vcs none $folder_name
cd $folder_name
cp ../default_main.rs src/main.rs
touch input.txt

#curl https://adventofcode.com/2022/day/$folder_name/input --cookie "session=53616c7465645f5f9084e26151e17b913537375e6cad8c90312163c3b21ea13760359b903c2a66def37edcf75ee540436edfb5261ae7527374ae23c520c03608" > input.txt

# 31 is to go to the line where the first example input needs to go
nvim -c 31 -p src/main.rs input.txt
