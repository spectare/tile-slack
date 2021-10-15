default: build

build:
	go test ./main.go ./main_test.go
	go build -o tile-slack ./main.go

docker:
	docker build -f Dockerfile -t spectare/tile-slack:latest .

dockerrun:
	docker run -p5000:5000 spectare/tile-slack:latest
