#!/usr/bin/env bash

FIFO=my_fifo

_term() {
    kill ${pid_merge}
    kill ${pid_serve}
    kill ${pid_server}
    rm -rf $FIFO
}

trap _term INT TERM

rm -rf $FIFO
mkfifo $FIFO

../target/debug/merge 127.0.0.1:8888 > $FIFO &
pid_merge=$!
cat $FIFO | ../target/debug/serve &
pid_serve=$!
sleep 1;
socat exec:"python ../serve/dummy_input.py" tcp:127.0.0.1:8888 &
make -C ../ui/ serve-files &
pid_server=$!
xdg-open http://localhost:8080
wait
