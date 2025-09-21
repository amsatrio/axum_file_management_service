pipeline {
    agent any
    stages {
        stage('Docker Build') { 
            agent any
            steps {
                sh 'docker compose -f ./container/docker/compose.yaml build'
            }
        }
        stage('Push the Image to Local Registry') { 
            agent any
            steps {
                sh 'docker rmi localhost:4000/axum-file-management-service:latest'
                sh 'docker tag axum-file-management-service localhost:4000/axum-file-management-service'
                sh 'docker push localhost:4000/axum-file-management-service' 
            }
        }
        stage('Kubernates Deployment') { 
            agent any
            steps {
 				  sh '''
				  microk8s.kubectl apply -f container/kubernates/storage.yaml
                  microk8s.kubectl apply -f container/kubernates/deployment.yaml
                  microk8s.kubectl apply -f container/kubernates/service.yaml
                  microk8s.kubectl rollout restart deployment/axum-file-management
				  '''
            }
        }
    }
}

