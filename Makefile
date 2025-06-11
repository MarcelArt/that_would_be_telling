docker-build:
	@docker build -t marcelaritonang/that-would-be-telling:latest .

docker-push: docker-build
	@docker push marcelaritonang/that-would-be-telling:latest
	