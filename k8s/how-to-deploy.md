# How to Deploy

## Prerequisites
Make sure you have the following installed:
- Docker
- gcloud
- kubectl

Please refer to the [Installation](#set-up-gcp) section for instructions.

Make sure the cluster is up and running. Refer to the [Cluster](#cluster) section for instructions.

Make sute Kubectl is configured to use the cluster. Refer to the [Kubectl](#kubectl) section for instructions.

Update [k8s/.env](.env) and [k8s/.ev.secret](.env.secret) files if necessary. Create the files if they don't exist as they are not tracked by git.

Use the following template for the `k8s/.env` file:
```toml
RUST_LOG=debug
QDRANT_URL=http://qdrant:6334
WEBSERVER_PORT=3000
ORACLE_QUERY_URL=http://oracle:3000/query
PROJECT_ID=ask-ircc-ai
CLUSTER_NAME=ask-ircc-cluster
``````

Use the following template for the `k8s/.env.secret` file:

```toml
TELOXIDE_TOKEN=YOUR_TELOXIDE_TOKEN
OPENAI_API_KEY=YOUR_OPENAI_API_KEY
```

### Build and Push Docker Images

Build the docker images for the bot, oracle, and qdrant:

```bash
make publish
```

### Update ConfigMap

Update the ConfigMap with the latest environment variables:

```bash
./k8s/update-configmap.sh
```

### Deploy to Kubernetes

Deploy the bot, oracle, and qdrant to the cluster:

```bash
make deploy
```

-----------------

## Set up GCP

Here's a step-by-step guide on how to use `gcloud`, the command-line tool for Google Cloud Platform (GCP):


**On macOS**:
You can use Homebrew:
```bash
brew install --cask google-cloud-sdk
```

### 1. Initial Setup:

Once installed, initialize `gcloud`:

```bash
gcloud init
```

This will:
- Let you log in to your Google account.
- Choose a default project.
- Choose a default Compute Engine zone.
- Configure some basic settings.

Use the following command to set up the default region and zone:

```bash
gcloud config set compute/zone northamerica-northeast2
```
- Note: northamerica-northeast2 is the zone for Toronto, Canada. You can choose a different zone if you wish.

### 2. Logging In:

You can log in to your Google account with:

```bash
gcloud auth login
```

This will open a new browser window (or give you a URL to visit) where you'll provide your credentials and authorize `gcloud`.

### 3. Creating a New Project:

To create a new project:

```bash
gcloud projects create YOUR_PROJECT_ID --name="YOUR_PROJECT_NAME"
```

Replace `YOUR_PROJECT_ID` with a unique ID for your project and `YOUR_PROJECT_NAME` with a friendly name.

### 4. Setting a Default Project:

When you have multiple projects, you can set a default project for your current session:

```bash
gcloud config set project YOUR_PROJECT_ID
```

### Tips:

- **Getting Help**: Almost all `gcloud` commands have a `--help` flag that can provide more detailed information on command usage.

- **Listing Projects**: To list all projects you have access to:
  ```bash
  gcloud projects list
  ```

- **Updating `gcloud`**: Keep the SDK updated for the latest features and security updates:
  ```bash
  gcloud components update
  ```

- **CLI Autocompletion**: `gcloud` includes autocompletion for commands. If you're using Bash or Zsh, the completion scripts are usually installed by default.

Remember that `gcloud` is a powerful tool, and commands can have effects on your GCP resources and billing. Always ensure you understand a command before executing it, especially when modifying or deleting resources.

### Container Registry

Run the following command to enable the Container Registry Service:

```bash
gcloud services enable containerregistry.googleapis.com
```

Configure Docker with the following command:

    ```bash
    gcloud auth configure-docker
    ```

    Your credentials are saved in your user home directory.

    -   Linux: `$HOME/.docker/config.json`
    -   Windows: `%USERPROFILE%/.docker/config.json`

Note: To authenticate your request, follow the steps in: https://cloud.google.com/container-registry/docs/advanced-authentication


### Cluster
Before creating a cluster, you'll need to set up a few things:

1. Enable the billing account for your project. While you can perform many billing-related operations using the `gcloud` command-line tool and the Cloud Billing API, you cannot create a new billing account or directly enable a billing account via the command line. These actions typically involve entering sensitive payment information, which is better suited for a secure web interface rather than the command line.

However, once you have a billing account set up through the Google Cloud Console, you can do the following via the command line:

   1. **Link a Project to a Billing Account**:
       ```bash
       gcloud beta billing projects link YOUR_PROJECT_ID --billing-account=YOUR_BILLING_ACCOUNT_ID
       ```

   2. **List billing accounts**:
       ```bash
       gcloud beta billing accounts list
       ```

   3. **Get details of a specific billing account**:
       ```bash
       gcloud beta billing accounts describe YOUR_BILLING_ACCOUNT_ID
       ```

If you need to set up a new billing account, you will need to go to the [Google Cloud Console](https://console.cloud.google.com/), navigate to the "Billing" section, and create or set up your billing account there. After that, you can manage the associations of billing accounts to projects using the command line as shown above.

2. Enable the Kubernetes Engine API:
    ```bash
    gcloud services enable container.googleapis.com
    ```

3. Create a cluster:
    ```bash
    gcloud container clusters create YOUR_CLUSTER_NAME
    ```

### Kubectl

`kubectl` is the command-line tool for Kubernetes. It allows you to run commands against Kubernetes clusters. You can use it to deploy applications, inspect and manage cluster resources, and view logs.

To install `kubectl`, run the following command:

```bash
brew install kubectl
```


#### Wipe existing config and set up for GCP

To change your `kubectl` configuration from Azure (or any other environment) to Google Cloud Platform (GCP), you'll want to:

1. **Backup your existing configuration**: Before making changes, it's a good practice to backup your current configuration:
    ```bash
    cp ~/.kube/config ~/.kube/config.backup
    ```

2. **Clear your existing configuration**: If you're sure you want to remove the existing `kubectl` configuration, you can simply overwrite the config file:
    ```bash
    > ~/.kube/config
    ```
    This will create an empty `kubectl` configuration file. If you wish to completely remove it instead of emptying it, you can use `rm ~/.kube/config`.

3. **Set up `kubectl` for GCP**:

   - Ensure that you've already set up `gcloud` as previously described.
   - Install the required plugin:
        ```bash
        gcloud components install gke-gcloud-auth-plugin
        ```

   - Obtain credentials for your GKE (Google Kubernetes Engine) cluster:
     ```bash
     gcloud container clusters get-credentials YOUR_CLUSTER_NAME --zone YOUR_ZONE --project YOUR_PROJECT_ID
     ```
     This command fetches credentials for the specified GKE cluster and automatically configures `kubectl` to use them. Replace `YOUR_CLUSTER_NAME`, `YOUR_ZONE`, and `YOUR_PROJECT_ID` with appropriate values for your GCP setup.

After completing these steps, `kubectl` should be configured to interact with your GKE cluster on GCP. You can verify by running:

```bash
kubectl cluster-info
```

If at any point you need to revert back to your old configuration, you can restore from the backup:

```bash
cp ~/.kube/config.backup ~/.kube/config
```