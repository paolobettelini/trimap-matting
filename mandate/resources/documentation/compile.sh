#!/bin/bash

MAIN_FILE="documentation"

if [ "$1" = "--help" ] || [ "$1" == "-h" ]; then
    echo "usage: ${0}"

    exit 0
fi

WORKING_DIR=`pwd`
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"

cd $SCRIPT_DIR

tectonic "${MAIN_FILE}.tex"

mv "${MAIN_FILE}.pdf" ../../

cd $WORKING_DIR