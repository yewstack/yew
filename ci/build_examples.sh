#!/usr/bin/env bash

# Script to build all examples in yew/examples 
#
# Not building yew-router nor yewtil examples

# src: https://gist.github.com/fbucek/f986da3cc3a9bbbd1573bdcb23fed2e1
set -e # error -> trap -> exit
function info() { echo -e "[\033[0;34m $@ \033[0m]"; } # blue: [ info message ]
function fail() { FAIL="true"; echo -e "[\033[0;31mFAIL\033[0m] $@"; } # red: [FAIL]
trap 'LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ "$FAIL" == "true"  ]; then fail finished with error; else echo -e "[\033[0;32m Finished $@ \033[0m]";fi' EXIT
SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir

cd $SRCDIR/../examples # switch to examples folder 

for EXAMPLE in *
do  
    if [[ $EXAMPLE ==  static ]] || [[ $EXAMPLE ==  server ]] || [[ $EXAMPLE ==  target ]]; then
        echo -e "Skipping folder: $EXAMPLE"
    elif [ -d ${EXAMPLE} ]; then
        ./build.sh ${EXAMPLE} $@
    fi
done
