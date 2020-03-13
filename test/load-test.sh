#!/bin/sh
# load-test.sh
# Sends 50k requests to Moria

req(){
	for i in $(seq 0 999) ; do
		curl -L -X GET 'http://127.0.0.1:8000/api/user' -w "%{http_code}\t%{time_total}\n" \
		  -H 'jwt-token:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiXX0.q6lNQ2JaiStSgP7IRmJR8z237GaFD3MmN-ZNtcnpWa8'
	done

	echo "DONE"
}

for i in $(seq 0 50) ; do
	 req "$i" &
done