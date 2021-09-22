#!/bin/bash
if ! command -v python3 2>&1 >/dev/null;
then
	echo 'error: you must install python3'
	exit
fi

cd static/
python3 -m http.server
