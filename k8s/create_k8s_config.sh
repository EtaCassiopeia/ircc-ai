#!/bin/bash

# Get the directory of the currently executing script
DIR="$(dirname "$0")"

echo "Creating ConfigMap and Secret from .env file..."

# Ensure the .env file exists
if [[ ! -f "${DIR}/.env" ]]; then
    echo ".env file not found in ${DIR}!"
    exit 1
fi

# Check for kubectl
if ! command -v kubectl &> /dev/null; then
    echo "kubectl could not be found. Please install kubectl."
    exit 1
fi

# Create ConfigMap from .env file
kubectl create configmap ircc-config --from-env-file="${DIR}/.env" --dry-run=client -o yaml | kubectl apply -f -

# Create Secret from .env.secret file
kubectl create secret generic ircc-secrets --from-env-file="${DIR}/.env.secret" --dry-run=client -o yaml | kubectl apply -f -

echo "ConfigMap and Secret have been created/updated from .env file."
