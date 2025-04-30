# FROM mcr.microsoft.com/devcontainers/python:3.12-bullseye
FROM mcr.microsoft.com/devcontainers/rust:bullseye

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Working directory will be set by the Docker Compose configuration
# to /workspaces/${localWorkspaceFolderBasename}

# Update and install dependencies
RUN apt-get update && \
    apt-get install -y \
    zip \
    unzip \
    curl \
    git \
    build-essential \
    tree \
    htop \
    && \
    rm -rf /var/lib/apt/lists/*

# Download and install AWS CLI v2
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && \
    ./aws/install --bin-dir /usr/local/bin --install-dir /usr/local/aws-cli --update && \
    rm -rf awscliv2.zip aws/

# Define default command
CMD ["bash"]