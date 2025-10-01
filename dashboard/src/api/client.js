// API Client for Task Queue Dashboard
class ApiClient {
  constructor(baseURL = 'http://localhost:16080') {
    this.baseURL = baseURL
    this.defaultHeaders = {
      'Content-Type': 'application/json',
    }
  }

  async request(endpoint, options = {}) {
    const url = `${this.baseURL}${endpoint}`
    const config = {
      headers: { ...this.defaultHeaders, ...options.headers },
      ...options
    }

    try {
      const response = await fetch(url, config)
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`)
      }

      const contentType = response.headers.get('content-type')
      if (contentType && contentType.includes('application/json')) {
        return await response.json()
      }
      
      return await response.text()
    } catch (error) {
      console.error(`API request failed for ${endpoint}:`, error)
      throw error
    }
  }

  // Health check
  async healthCheck() {
    return this.request('/health')
  }

  // Connection test
  async testConnection() {
    try {
      const response = await this.request('/health')
      return { connected: true, ...response }
    } catch (error) {
      return { connected: false, error: error.message }
    }
  }

  // Stats
  async getStats() {
    return this.request('/stats')
  }

  // Tasks
  async listTasks(params = {}) {
    const query = new URLSearchParams(params).toString()
    const endpoint = query ? `/tasks?${query}` : '/tasks'
    return this.request(endpoint)
  }

  async getTask(id) {
    return this.request(`/tasks/${id}`)
  }

  async createTask(task) {
    return this.request('/tasks', {
      method: 'POST',
      body: JSON.stringify(task)
    })
  }

  async updateTask(id, task) {
    return this.request(`/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(task)
    })
  }

  async deleteTask(id) {
    return this.request(`/tasks/${id}`, {
      method: 'DELETE'
    })
  }

  async cancelTask(id) {
    return this.request(`/tasks/${id}/cancel`, {
      method: 'POST'
    })
  }

  async retryTask(id) {
    return this.request(`/tasks/${id}/retry`, {
      method: 'POST'
    })
  }

  async getTaskStatus(id) {
    return this.request(`/tasks/${id}/status`)
  }

  async getTaskResult(id) {
    return this.request(`/tasks/${id}/result`)
  }

  async updateTaskPriority(id, priority) {
    return this.request(`/tasks/${id}/priority`, {
      method: 'PUT',
      body: JSON.stringify({ priority })
    })
  }

  async setTaskStatus(id, status) {
    return this.request(`/tasks/${id}/status`, {
      method: 'PUT',
      body: JSON.stringify({ status })
    })
  }

  async advanceTaskPhase(id) {
    return this.request(`/tasks/${id}/advance-phase`, {
      method: 'POST'
    })
  }

  async addTaskDependency(id, dependency) {
    return this.request(`/tasks/${id}/dependencies`, {
      method: 'POST',
      body: JSON.stringify(dependency)
    })
  }

  async getTaskDependencies(id) {
    return this.request(`/tasks/${id}/dependencies`)
  }

  async getTaskCorrelations(id) {
    return this.request(`/tasks/${id}/correlations`)
  }

  // Projects
  async listProjects() {
    return this.request('/projects')
  }

  async getProject(id) {
    return this.request(`/projects/${id}`)
  }

  async createProject(project) {
    return this.request('/projects', {
      method: 'POST',
      body: JSON.stringify(project)
    })
  }

  async updateProject(id, project) {
    return this.request(`/projects/${id}`, {
      method: 'PUT',
      body: JSON.stringify(project)
    })
  }

  async deleteProject(id) {
    return this.request(`/projects/${id}`, {
      method: 'POST'
    })
  }

  async getProjectTasks(id) {
    return this.request(`/projects/${id}/tasks`)
  }

  // Workflows
  async listWorkflows(params = {}) {
    const query = new URLSearchParams(params).toString()
    const endpoint = query ? `/workflows?${query}` : '/workflows'
    return this.request(endpoint)
  }

  async getWorkflow(id) {
    return this.request(`/workflows/${id}`)
  }

  async createWorkflow(workflow) {
    return this.request('/workflows', {
      method: 'POST',
      body: JSON.stringify(workflow)
    })
  }

  async getWorkflowStatus(id) {
    return this.request(`/workflows/${id}/status`)
  }

  async updateWorkflowStatus(id, status, message = 'Status updated') {
    return this.request(`/workflows/${id}/status`, {
      method: 'PUT',
      body: JSON.stringify({ status, message })
    })
  }

  // Authentication
  async login(credentials) {
    return this.request('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials)
    })
  }

  async logout() {
    return this.request('/api/auth/logout', {
      method: 'POST'
    })
  }

  async verifyToken() {
    return this.request('/api/auth/verify')
  }

  async refreshToken(refreshToken) {
    return this.request('/api/auth/refresh', {
      method: 'POST',
      body: JSON.stringify({ refresh_token: refreshToken })
    })
  }
}

// Create global API client instance
window.apiClient = new ApiClient()
