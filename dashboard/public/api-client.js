class TaskQueueAPIClient {
    constructor(baseURL = 'http://localhost:16080') {
        this.baseURL = baseURL;
    }

    async request(endpoint, options = {}) {
        const url = `${this.baseURL}${endpoint}`;
        const config = {
            headers: {
                'Content-Type': 'application/json',
                ...options.headers
            },
            ...options
        };

        try {
            const response = await fetch(url, config);
            
            if (!response.ok) {
                const errorText = await response.text();
                throw new Error(`HTTP ${response.status}: ${errorText}`);
            }

            const contentType = response.headers.get('content-type');
            if (contentType && contentType.includes('application/json')) {
                return await response.json();
            } else {
                return await response.text();
            }
        } catch (error) {
            console.error('API Request failed:', error);
            throw error;
        }
    }

    // Health Check
    async healthCheck() {
        return await this.request('/health');
    }

    // Stats
    async getStats() {
        return await this.request('/stats');
    }

    // Tasks
    async listTasks(project = null, status = null) {
        const params = new URLSearchParams();
        if (project) params.append('project', project);
        if (status) params.append('status', status);
        
        const query = params.toString();
        return await this.request(`/tasks${query ? '?' + query : ''}`);
    }

    async getTask(taskId) {
        return await this.request(`/tasks/${taskId}`);
    }

    async getTaskStatus(taskId) {
        return await this.request(`/tasks/${taskId}/status`);
    }

    async getTaskResult(taskId) {
        return await this.request(`/tasks/${taskId}/result`);
    }

    async createTask(taskData) {
        return await this.request('/tasks', {
            method: 'POST',
            body: JSON.stringify(taskData)
        });
    }

    async cancelTask(taskId, reason = 'User requested cancellation') {
        return await this.request(`/tasks/${taskId}/cancel`, {
            method: 'POST',
            body: JSON.stringify({ reason })
        });
    }

    async retryTask(taskId, resetRetryCount = false) {
        return await this.request(`/tasks/${taskId}/retry`, {
            method: 'POST',
            body: JSON.stringify({ reset_retry_count: resetRetryCount })
        });
    }

    async updateTaskPriority(taskId, priority) {
        return await this.request(`/tasks/${taskId}/priority`, {
            method: 'PUT',
            body: JSON.stringify({ priority })
        });
    }

    async addTaskDependency(taskId, dependencyData) {
        return await this.request(`/tasks/${taskId}/dependencies`, {
            method: 'POST',
            body: JSON.stringify(dependencyData)
        });
    }

    async getTaskDependencies(taskId) {
        return await this.request(`/tasks/${taskId}/dependencies`);
    }

    async advanceTaskPhase(taskId) {
        return await this.request(`/tasks/${taskId}/advance-phase`, {
            method: 'POST'
        });
    }

    async getTaskCorrelations(taskId) {
        return await this.request(`/tasks/${taskId}/correlations`);
    }

    // Workflows
    async listWorkflows(project = null, status = null) {
        const params = new URLSearchParams();
        if (project) params.append('project', project);
        if (status) params.append('status', status);
        
        const query = params.toString();
        return await this.request(`/workflows${query ? '?' + query : ''}`);
    }

    async getWorkflow(workflowId) {
        return await this.request(`/workflows/${workflowId}`);
    }

    async getWorkflowStatus(workflowId) {
        return await this.request(`/workflows/${workflowId}/status`);
    }

    async createWorkflow(workflowData) {
        return await this.request('/workflows', {
            method: 'POST',
            body: JSON.stringify(workflowData)
        });
    }

    async cancelWorkflow(workflowId, reason = 'User requested cancellation') {
        return await this.request(`/workflows/${workflowId}/cancel`, {
            method: 'POST',
            body: JSON.stringify({ reason })
        });
    }

    async approveWorkflow(workflowId, message = 'Workflow approved') {
        return await this.request(`/workflows/${workflowId}/approve`, {
            method: 'POST',
            body: JSON.stringify({ message })
        });
    }

    async updateWorkflowStatus(workflowId, status, message = 'Status updated') {
        return await this.request(`/workflows/${workflowId}/status`, {
            method: 'PUT',
            body: JSON.stringify({ status, message })
        });
    }

    // Metrics
    async getMetrics() {
        return await this.request('/metrics');
    }

    // Test Connection
    // Projects
    async listProjects() {
        return await this.request('/projects');
    }

    async getProject(projectId) {
        return await this.request(`/projects/${projectId}`);
    }

    async createProject(projectData) {
        return await this.request('/projects', {
            method: 'POST',
            body: JSON.stringify(projectData)
        });
    }

    async updateProject(projectId, projectData) {
        return await this.request(`/projects/${projectId}`, {
            method: 'PUT',
            body: JSON.stringify(projectData)
        });
    }

    async deleteProject(projectId) {
        return await this.request(`/projects/${projectId}`, {
            method: 'POST'
        });
    }

    async getProjectTasks(projectId) {
        return await this.request(`/projects/${projectId}/tasks`);
    }

    async testConnection() {
        try {
            await this.healthCheck();
            return { connected: true };
        } catch (error) {
            return { connected: false, error: error.message };
        }
    }
}

// Create global instance
window.apiClient = new TaskQueueAPIClient();
