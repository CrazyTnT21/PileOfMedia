FROM postgres:17-alpine

WORKDIR /app
COPY . .

RUN echo "psql -c \"create database collectiondb;\"" > /docker-entrypoint-initdb.d/init.sh
RUN echo "psql -f /app/setup.sql collectiondb" >> /docker-entrypoint-initdb.d/init.sh

EXPOSE 5432
CMD ["postgres"]
