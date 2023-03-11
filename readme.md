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

curl localhost:3030/technology | jq

curl -X POST -d "{\"name\": \"test test\"      }"


curl --header "Content-Type: application/json"  --request POST  --data '{"name":"xyz" }'  localhost:3030/technology 
