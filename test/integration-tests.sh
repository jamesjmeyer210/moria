#!/bin/sh
# integration-tests.sh

# Send a request to /api/user with a valid token and receive a 200
curl -L -X GET "http://127.0.0.1:8000/api/user" \
  --header 'jwt-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjMyNTAzNjgwMDAwLCJncm91cHMiOlsidXNlcnMiLCJhZG1pbnMiXX0.jSoEF07QCc6Si0MQRbThx3pSEQgtsAT74ucCh_fbAmI' \
  -w "\t%{http_code}\t%{time_total}\n"
