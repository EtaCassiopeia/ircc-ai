include ./k8s/.env

SHELL := /bin/bash

# Variables
SERVICE_NAMES = bot oracle embed

# Default: build, tag, push all services
publish: $(SERVICE_NAMES)

# Build, tag, and push a specific service
$(SERVICE_NAMES):
	docker buildx build --platform='linux/amd64' -t $@ -f docker/Dockerfile.$@.prod --load .
	docker tag $@ gcr.io/$(PROJECT_ID)/$@:latest
	docker push gcr.io/$(PROJECT_ID)/$@:latest

# Cluster
cluster:
	gcloud container clusters create $(CLUSTER_NAME)
	gcloud container clusters get-credentials $(CLUSTER_NAME)

# Deploy to Kubernetes
deploy:
	. ./k8s/.env
	@echo "Deploying to $(CLUSTER_NAME)@$(PROJECT_ID)..."
	# kubectl apply -f k8s/qdrant/deployment.yaml
	# kubectl apply -f k8s/qdrant/service.yaml
	# @read -p "qdrant has been deployed successfully. Press enter to continue..."
	# env $$(cat ./k8s/.env | xargs) envsubst < k8s/embed/job.yaml | kubectl apply -f -
	# @read -p "embed has been deployed successfully. Press enter to continue..."
	env $$(cat ./k8s/.env | xargs) envsubst < k8s/oracle/deployment.yaml | kubectl apply -f -
	@read -p "oracle has been deployed successfully. Press enter to continue..."
	env $$(cat ./k8s/.env | xargs) envsubst < k8s/oracle/service.yaml | kubectl apply -f -
	# env $$(cat ./k8s/.env | xargs) envsubst < k8s/bot/deployment.yaml | kubectl apply -f -
	@echo "All services have been deployed successfully."
