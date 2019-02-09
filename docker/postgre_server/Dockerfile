FROM postgres:alpine
ARG database_name=apf

COPY init.sql /docker-entrypoint-initdb.d/10-init.sql

ENV DATABASE_NAME=$database_name

EXPOSE 5432

CMD ["postgres"]

