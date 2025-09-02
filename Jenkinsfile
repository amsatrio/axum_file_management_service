pipeline {
    agent any
    stages {
        stage('Build') { 
            steps {
                sh '. "$HOME/.cargo/env"'
                sh 'make build_release' 
            }
        }
    }
}

