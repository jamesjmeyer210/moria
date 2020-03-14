#!/bin/bash
# memcheck.sh

debug='target/debug/moria'
release='target/release/moria'

debug() {
  valgrind --tool=memcheck \
	--leak-check=yes \
	--show-reachable=yes \
	--num-callers=20 \
	--track-fds=yes \
	"./$1"
}

if [ "$#" -eq "1" ] && [ "$1" == "$release" ] ; then
  printf "[%s]: Debugging release...\n" "$release"
  debug "$release"
else
  printf "[%s]: Debugging default...\n" "$debug"
  debug "$release"
fi