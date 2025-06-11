## Dockerfile Run
1. `docker build -t marcelaritonang/that-would-be-telling:latest .`
2. `docker push marcelaritonang/that-would-be-telling:latest`
3. `docker run -p 8080:8080 marcelaritonang/that-would-be-telling:latest`

## SurrealDB Docker Compose Example
```
version: '3.8'

networks:
  my_infra:
    driver: bridge

services:
  surrealdb:
    user: root
    env_file:
      - .env
    entrypoint: 
      - /surreal 
      - start 
      - --user
      - $DB_USER
      - --pass
      - $DB_PASSWORD
      - --log
      - debug
      - rocksdb:/mydata/surreal.db
    image: surrealdb/surrealdb:latest
    volumes:
      - ./data:/mydata
    ports:
      - 8000:8000

```