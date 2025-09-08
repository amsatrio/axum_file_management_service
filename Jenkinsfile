pipeline {
    agent any
    stages {
        stage('Build & Push to Local Registry') { 
            steps {
                sh 'docker compose build'
                sh 'docker tag axum-file-management-service localhost:4000/axum-file-management-service'
                sh 'docker push localhost:4000/axum-file-management-service' 
            }
        }
    }
}

