#!/bin/bash

while read -ra line;
do
	for word in "${line[@]}";
	do
		printf "%d " "0x$word";
	done;
	printf "\n";
done < dump.txt
