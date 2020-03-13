#!/bin/sh
# load-test.sh
# Sends x * y requests for moria every z seconds

thread_count="$1"
request_cout="$2"
wait="$3"

req(){
	for i in $(seq 0 "$request_cout") ; do
		curl -L -X GET 'http://127.0.0.1:8000/api/user' -w "%{http_code}\t%{time_total}\n" \
		  -H 'jwt-token:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiXX0.q6lNQ2JaiStSgP7IRmJR8z237GaFD3MmN-ZNtcnpWa8'
    sleep "$wait"
	done

	echo "DONE"
}

for i in $(seq 0 "$thread_count") ; do
	 req "$i" &
done