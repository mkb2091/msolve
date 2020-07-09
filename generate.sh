#! /bin/sh
FILE=$1
PROCESS_AMOUNT=$2
CELLS_TO_REMOVE=$3

while true
do
    cargo run --release -q -- -c generate -d continuous -n 10 >> $FILE
    sort -n $FILE|uniq |tail -n 100000 > temp
    mv temp $FILE
    tail $FILE -n $PROCESS_AMOUNT | rg "[0-9.]{81}" -o | cargo run --release -q -- -c generate -d once $CELLS_TO_REMOVE | rg "^[0-9][0-9]+;"|sort -n|tee -a $FILE|tee temp
    wc -l temp
    wc -l $FILE
done