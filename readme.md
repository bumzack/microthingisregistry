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
