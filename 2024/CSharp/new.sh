#!/bin/bash

folder_name=$1;

if [[ -z $folder_name ]]; then
    read -p 'Name: ' folder_name
fi

if [[ -d $folder_name ]]; then
    echo $folder_name already exists
    exit 1
fi


mkdir $folder_name
cd $folder_name
dotnet new console
cp ../default_Program.cs Program.cs
touch example.txt

#curl https://adventofcode.com/2022/day/$folder_name/input --cookie "session=53616c7465645f5f9084e26151e17b913537375e6cad8c90312163c3b21ea13760359b903c2a66def37edcf75ee540436edfb5261ae7527374ae23c520c03608" > input.txt

nvim Program.cs input.txt example.txt
