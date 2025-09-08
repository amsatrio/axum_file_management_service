pipeline {
    agent any
    stages {
        stage('Build & Push to Local Registry') { 
            steps {
                sh 'docker compose build'
                sh 'docker tag axum_file_management_service localhost:4000/axum_file_management_service'
                sh 'docker push localhost:4000/axum_file_management_service' 
            }
        }
    }
}

