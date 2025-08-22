# LibroCommerce: A Modern Book Selling Platform

sql prepare

```bash
psql -h localhost -p 5432 -U postgres
create database libro_commerce;
sqlx migrate run --database-url=postgres://postgres:password@127.0.0.1:5432/libro_commerce
```
