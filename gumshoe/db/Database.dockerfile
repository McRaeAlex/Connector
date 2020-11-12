FROM postgres
ENV POSTGRES_USER dev
ENV POSTGRES_PASSWORD dev
ENV POSTGRES_DB gumshoe
COPY UP.sql /docker-entrypoint-initdb.d/0-UP.sql
COPY INSERT.sql /docker-entrypoint-initdb.d/1-INSERT.sql
