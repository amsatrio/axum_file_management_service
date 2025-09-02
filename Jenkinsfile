pipeline {
    agent any
    stages {
        stage('Build') { 
            steps {
                sh 'docker compose build --no-cache'
                sh 'docker compose up -d' 
            }
        }
    }
}

