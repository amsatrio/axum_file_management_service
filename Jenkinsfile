pipeline {
    agent any
    stages {
        stage('Build') { 
            steps {
                sh '. "/home/user0/.cargo/env"'
                sh 'make build_release' 
            }
        }
    }
}

