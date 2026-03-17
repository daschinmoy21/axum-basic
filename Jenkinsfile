pipeline {
    agent any

    environment {
        IMAGE_TAG = "crimxnhaze/bcs52:latest"
    }

    stages {
        stage('Checkout') {
            steps {
                git 'https://github.com/daschinmoy21/axum-basic.git'
            }
        }

        stage('Build') {
            steps {
                sh 'docker build -t $IMAGE_TAG .'
            }
        }

        stage('Login') {
            steps {
                withCredentials([usernamePassword(
                    credentialsId: 'DOCKERHUB',
                    usernameVariable: 'USER',
                    passwordVariable: 'PASS'
                )]) {
                    sh 'echo "$PASS" | docker login -u "$USER" --password-stdin'
                }
            }
        }

        stage('Push') {
            steps {
                sh 'docker push $IMAGE_TAG'
            }
        }
    }
}
