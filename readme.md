# Readme

## create database user and grant rights

```
CREATE USER 'microthingisregistry'@localhost IDENTIFIED BY 'microthingisregistry';
GRANT ALL PRIVILEGES ON *.* TO 'microthingisregistry'@localhost IDENTIFIED BY 'microthingisregistry';
FLUSH PRIVILEGES;
```

## run migrations

```diesel migration run```

```diesel migration redo -n 5```

```diesel migration list```


## maria DB

```SHOW ENGINE INNODB STATUS```

```select * from __diesel_schema_migrations;```



## REST API

###  technology 

curl localhost:3030/api/technology | jq

curl --header "Content-Type: application/json"  --request POST  --data '{"name":"xyz" }'  localhost:3030/api/technology 


###  host

curl localhost:3030/api/host | jq

curl --header "Content-Type: application/json"  --request POST  --data '{"hostname":"localhost", "ip":"127.0.0.1", "port":123 }'  localhost:3030/api/host 


###  frontend

curl localhost:3030/api/frontend | jq

curl --header "Content-Type: application/json"  --request POST  --data '{"url":"localhost", "service_url":"127.0.0.1", "local_repo_path":"127.0.0.1", "microservice_id":"127.0.0.1", "port":123 , "technology_id": 1}'  localhost:3030/api/frontend 
 


###  backend

curl localhost:3030/api/backend | jq

curl --header "Content-Type: application/json"  --request POST  --data '{"openapi_url" : "/swagger.html", "service_url" : "/search/product",  "local_repo_path" : "/Users/bumzack/bla",  "microservice_id" : "solrsearchcategory", "technology_id" : 3, "publish_as_frontend_package" :  false }'  localhost:3030/api/backend 
