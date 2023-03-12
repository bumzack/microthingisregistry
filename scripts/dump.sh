#!/bin/zsh

# dump all tables
echo ""
echo "HOST"
echo "====================================="
curl -s localhost:3030/api/host | jq
echo ""

echo ""
echo "TECHNOLOGY"
echo "====================================="
curl -s localhost:3030/api/technology | jq
echo ""

echo ""
echo "MICROSERVICE"
echo "====================================="
curl -s localhost:3030/api/microservice | jq
echo ""

echo ""
echo "FRONTEND"
echo "====================================="
curl -s localhost:3030/api/frontend | jq
echo ""


echo ""
echo "BACKEND"
echo "====================================="
curl -s localhost:3030/api/backend | jq
echo ""