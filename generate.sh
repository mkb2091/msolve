#! /bin/sh
FILE=$1
PROCESS_AMOUNT=$2
CELLS_TO_REMOVE=$3

while true
do
    sort -u $FILE | sort -n | tail -n $PROCESS_AMOUNT | rg "[0-9.]{81}" -o | msolve generate $CELLS_TO_REMOVE | msolve difficulty_verify_unique | rg "[0-9][0-9]+;"|sort -n|tee -a $FILE|tee temp && wc -l temp
done