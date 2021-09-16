#!/usr/bin/env bash

_term() {
    kill ${pid_merge}
    kill ${pid_serve}
    kill ${pid_server}
}

trap _term INT TERM

../target/debug/serve &
pid_serve=$!
../target/debug/merge --bind 127.0.0.1:8888 127.0.0.1:8081 &
pid_merge=$!
sleep 1;
socat exec:"python ../serve/dummy_input.py" tcp:127.0.0.1:8888 &
make -C ../ui/ serve-files &
pid_server=$!
sleep 1
xdg-open http://localhost:8080
wait
