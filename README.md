# DI

The following repository contains an small Rest API example with Rust language. It creates a CRUD for bikes.

### Dependencies

Diesel ORM:

```
cargo install diesel_cli --no-default-features --features postgres
```
Auto-reload:
```
cargo install systemfd cargo-watch
```

### Running the Application

1. Deploy the Postgres database
```bash
docker-compose up
````

2. Diesel configuration:
```
export DATABASE_URL=postgres://bikes:bikes@localhost/bikes
```

```
diesel setup
diesel migration run
```

3. Start the application:
The following command runs the aplication and allows to reload it when you change something in the code. Please check the `.env` to verified the port in wich it will run the application.  
```bash
systemfd --no-pid -s http::4000 -- cargo watch -x run
```

### Usage
#### Get all bikes
```
curl --request GET \
  --url http://localhost:4000/bikes
```

#### Get a bike by id
```
curl --request GET \
  --url http://localhost:4000/bikes/1fdf0da9-1bfe-410e-b6ac-5a27b3cd1dc8
```
#### Create a bike

```
curl --request POST \
  --url http://localhost:4000/bikes \
  --header 'content-type: application/json' \
  --data '{
	"id": "1fdf0da9-1bfe-410e-b6ac-5a27b3cd1dc8",
	"model": "MBT",
	"description": "This is a montain bike",
	 "created_at": "2020-07-07T02:06:24.028203",
  "updated_at": "2020-07-07T02:06:24.028220"
}
'
```

#### Update a bike
```
curl --request PUT \
  --url http://localhost:4000/bikes/1fdf0da9-1bfe-410e-b6ac-5a27b3cd1dc8 \
  --header 'content-type: application/json' \
  --data '{
  "model": "",
  "description": "",
  "created_at": "2020-07-07T02:06:24.028203",
  "updated_at": "2020-07-07T02:06:24.028220"
}'
```


#### Delete a bike
```
curl --request GET \
  --url http://localhost:4000/bikes/1fdf0da9-1bfe-410e-b6ac-5a27b3cd1dc8
```