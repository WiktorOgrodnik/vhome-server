docker run  --rm -v datadir:/var/lib/postgresql/data -d -p 5432:5432 -e POSTGRES_PASSWORD=secret_pass  grouplist-postgres
