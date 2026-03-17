pipeline {
    agent any

    environment {
        IMAGE_NAME = "axum-basic"
        IMAGE_TAG = "crimxnhaze/axum-basic-23bcs52:latest"
    }

    stages {
        stage('Checkout Source Code') {
            steps {
                git 'https://github.com/daschinmoy21/axum-basic.git'
            }
        }

        stage('Build Docker Image') {
            steps {
                sh 'docker build -t $IMAGE_NAME .'
            }
        }

        stage('Tag Docker Image') {
            steps {
                sh 'docker tag $IMAGE_NAME $IMAGE_TAG'
            }
        }

        stage('Login to Docker Hub') {
            steps {
                withCredentials([usernamePassword(
                    credentialsId: 'dockerhub-creds',
                    usernameVariable: 'DOCKER_USER',
                    passwordVariable: 'DOCKER_PASS'
                )]) {
                    sh '''
                        set +x
                        echo "$DOCKER_PASS" | docker login -u "$DOCKER_USER" --password-stdin
                    '''
                }
            }
        }

        stage('Push Docker Image') {
            steps {
                sh 'docker push $IMAGE_TAG'
            }
        }
    }
}
