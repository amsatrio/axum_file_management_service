pipeline {
    agent any
    stages {
        stage('Docker Build') { 
            agent any
            steps {
                sh 'docker rmi -f localhost:4000/axum-file-management-service:latest'
                sh 'docker compose -f ./container/docker/compose.yaml build'
            }
        }
        stage('Push the Image to Local Registry') { 
            agent any
            steps {
                sh 'docker tag axum-file-management-service localhost:4000/axum-file-management-service'
                sh 'docker push localhost:4000/axum-file-management-service' 
            }
        }
        stage('Kubernates Deployment') { 
            agent any
            steps {
 				  sh '''
				  kubectl apply -f container/kubernates/storage.yaml --validate=false
                  kubectl apply -f container/kubernates/service.yaml --validate=false
                  kubectl apply -f container/kubernates/deployment.yaml --validate=false
				  '''
            }
        }
    }
}

