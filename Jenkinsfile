pipeline {
    agent any
    stages {
        stage('Build & Push to Local Registry') { 
            agent any
            steps {
                sh 'docker compose -f ./container/docker/compose.yaml build'
                sh 'docker tag axum-file-management-service localhost:4000/axum-file-management-service'
                sh 'docker push localhost:4000/axum-file-management-service' 
            }
        }
        stage('Kubernates Deployment') { 
            agent any
            steps {
 				  sh '''
				  export KUBECONFIG=/home/jenkins/.kube/config
				  kubectl apply -f container/kubernates/storage.yaml --validate=false
                  kubectl apply -f container/kubernates/service.yaml --validate=false
                  kubectl apply -f container/kubernates/deployment.yaml --validate=false
				  '''
            }
        }
    }
}

