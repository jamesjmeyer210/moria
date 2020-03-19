#!/bin/bash

[ ! -d "ssl" ] && mkdir "ssl"

openssl req -x509 -newkey rsa:4096 -nodes -keyout 'ssl/key.pem' -out 'ssl/cert.pem' -days 365 -subj '/CN=localhost'