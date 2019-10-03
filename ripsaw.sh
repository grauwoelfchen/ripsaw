#!/bin/env sh

awk -v p="${2}-" -v s=".csv" -v l="${3}" -f ripsaw.awk $1
