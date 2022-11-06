#!/bin/bash
nodename=$(LC_TYPE=C < /dev/urandom tr -cd '[:alnum:]' | tr -d '\n' | fold -w 10 | head -n 1)
dtnd -n $nodename -e incoming -C mtcp -r epidemic 2>dtnd.log &
sleep 10
dtntrigger -e incoming -c /asprn/responder
