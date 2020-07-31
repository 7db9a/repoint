#!/bin/bash

# privkey 5JZ4RXH4MoXpaUQMcJHo8DxhZtkf5U5VnYd9zZH8BRKZuAbxZEw
# 1 = opcode
# 2 = msg

docker run --rm repoint_opreturn:0.1.0 node ./lib/opreturn.js \
"5JZ4RXH4MoXpaUQMcJHo8DxhZtkf5U5VnYd9zZH8BRKZuAbxZEw" \
$2 \
"$3" \
400 \
https://api.mattercloud.net \
true
