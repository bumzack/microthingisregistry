#!/bin/zsh

# technologies
curl --header "Content-Type: application/json"  --request POST  --data '{"name":"webflux" }'  localhost:3030/api/technology
curl --header "Content-Type: application/json"  --request POST  --data '{"name":"rust" }'  localhost:3030/api/technology
curl --header "Content-Type: application/json"  --request POST  --data '{"name":"typescript" }'  localhost:3030/api/technology
