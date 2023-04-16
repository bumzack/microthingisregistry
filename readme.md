# Readme

## read data from service

```
curl -vv http://microthingisregistry.bumzack.at/api/backend
```

```
curl -vv localhost:3030/api/backend
```

## create database user and grant rights

```
CREATE USER 'microthingisregistry'@localhost IDENTIFIED BY 'microthingisregistry';
GRANT ALL PRIVILEGES ON *.* TO 'microthingisregistry'@localhost IDENTIFIED BY 'microthingisregistry';
FLUSH PRIVILEGES;
```

```
SELECT user FROM mysql.user;
DELETE FROM mysql.user where user = 'microthingisregistry';

flush privileges;

CREATE DATABASE microthingisregistry;

CREATE USER 'microthingisregistry'@'localhost' IDENTIFIED BY 'microthingisregistry';
GRANT ALL PRIVILEGES ON microthingisregistry.* TO 'microthingisregistry'@'localhost';

FLUSH PRIVILEGES;
```

## run migrations

```
diesel migration run
```

```
diesel migration redo -n 5
```

```
diesel migration list
```

## maria DB

```SHOW ENGINE INNODB STATUS```

```select * from __diesel_schema_migrations;```

## REST API

### technology

```
curl localhost:3030/api/technology | jq
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"name":"webflux" }' localhost:3030/api/technology
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"name":"rust" }' localhost:3030/api/technology
```

```
curl localhost:3030/api/technology/webflux | jq
```

### host

```
curl localhost:3030/api/host | jq
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"hostname":"localhost", "ip":"127.0.0.1", "port":123 }' localhost:3030/api/host
```

### frontend

```
curl localhost:3030/api/frontend | jq
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"url":"localhost", "service_url":"127.0.0.1", "local_repo_path":"127.0.0.1", "microservice_id":"127.0.0.1", "port":123 , "technology_id": 1}' localhost:3030/api/frontend
```

### backend

```
curl localhost:3030/api/backend | jq
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"openapi_url" : "/swagger.html", "service_url" : "/search/product",  "local_repo_path" : "/Users/bumzack/bla",  "microservice_id" : "solrsearchcategory", "technology_id" : 3, "publish_as_frontend_package" :  false,"api_client_prefix" : "ApiClientPrefix", "api_client_package" : "idontknow_yet" }' localhost:3030/api/backend
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"openapi_url" : "/swagger.html", "service_url" : "/search/movie",  "local_repo_path" : "/Users/bumzack/schalala",  "microservice_id" : "article", "technology_id" : 2, "publish_as_frontend_package" :  true, "api_client_prefix" : "ApiClientPrefix2", "api_client_package" : "idontknow_yet2" }' localhost:3030/api/backend
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"openapi_url" : "/swagger.html", "service_url" : "/search/person",  "local_repo_path" : "/Users/bumzack/person",  "microservice_id" : "person", "technology_id" : 3, "publish_as_frontend_package" :  true, "api_client_prefix" : "ApiClientPrefixperson", "api_client_package" : "idontknow_yetperson" }' localhost:3030/api/backend
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"openapi_url" : "/swagger.html", "service_url" : "/search/customer",  "local_repo_path" : "/Users/bumzack/customer",  "microservice_id" : "customer", "technology_id" : 2, "publish_as_frontend_package" :  true, "api_client_prefix" : "ApiClientPrefixcustomer", "api_client_package" : "idontknow_yet_customer", "url":"customer_url" }' localhost:3030/api/frontend
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"openapi_url" : "/swagger.html", "service_url" : "/search/customerprice",  "local_repo_path" : "/Users/bumzack/customerprice",  "microservice_id" : "customerprice", "technology_id" : 3, "publish_as_frontend_package" :  true, "api_client_prefix" : "ApiClientPrefixcustomerprice", "api_client_package" : "idontknow_yet_customerprice", "url":"customer_customerprice" }'localhost:3030/api/frontend
```

```
curl --header "Content-Type: application/json"  --request POST --data '{"openapi_url" : "/swagger.html", "service_url" : "/search/customer23",  "local_repo_path" : "/Users/bumzack/customer",  "microservice_id" : "customer23", "technology_id" : 3, "publish_as_frontend_package" :  true, "api_client_prefix" : "ApiClientPrefixcustomer23", "api_client_package" : "idontknow_yet_customer23", "url":"customer_url23" }' localhost:3030/api/frontend
```

```
curl --header "Content-Type: application/json"  --request PUT --data '{"openapiclient" : "here comes the code for thejson definition" }' localhost:3030/api/backend/solrsearchcategory
```

```
curl localhost:3030/api/openapiclient/update | jq
```

```
curl localhost:3030/api/backend/searcharticle | jq
```

```
curl localhost:3030/api/backend/openapiclient/searcharticle | jq  > client.json
```

```
curl localhost:3030/api/backend/apiclientprefix/searcharticle
```

```
curl localhost:3030/api/backend/apiclientpackage/searcharticle
```

```
curl localhost:3030/api/backend/asfrontend/all
```


