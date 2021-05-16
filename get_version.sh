#! /bin/bash

cargo metadata --no-deps --format-version 1 | python -c 'import sys, json; txt=sys.stdin.read(); data = json.loads(txt); print(data["packages"][0]["version"])'
