clear-db:
  docker compose down
  sleep 1
  rm -r ./docker-volumes || true
  sleep 1
  docker compose up -d
  sleep 1
  sqlx migrate run
