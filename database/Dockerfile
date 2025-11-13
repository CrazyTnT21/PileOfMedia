FROM postgres:18-alpine

WORKDIR /app
COPY . .

RUN echo "psql -c \"create database pileofmedia_db;\"" > /docker-entrypoint-initdb.d/init.sh
RUN echo "psql -f /app/setup.sql pileofmedia_db" >> /docker-entrypoint-initdb.d/init.sh

EXPOSE 5432
CMD ["postgres"]
