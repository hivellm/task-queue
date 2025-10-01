<template>
  <div class="console-page">
    <div class="page-header">
      <h1><i class="fas fa-terminal"></i> Console</h1>
      <p>API testing and debugging console</p>
    </div>

    <div class="console-content">
      <div class="console-grid">
        <div class="console-panel">
          <div class="panel-header">
            <h3><i class="fas fa-code"></i> API Request</h3>
            <div class="panel-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-save"></i> Save
              </button>
              <button class="btn btn-sm btn-primary" @click="sendRequest">
                <i class="fas fa-play"></i> Send
              </button>
            </div>
          </div>
          <div class="panel-content">
            <div class="request-config">
              <div class="config-row">
                <label>Method:</label>
                <select v-model="request.method" class="form-select">
                  <option value="GET">GET</option>
                  <option value="POST">POST</option>
                  <option value="PUT">PUT</option>
                  <option value="DELETE">DELETE</option>
                </select>
              </div>
              <div class="config-row">
                <label>URL:</label>
                <input v-model="request.url" type="text" class="form-input" placeholder="Enter API endpoint">
              </div>
              <div class="config-row">
                <label>Headers:</label>
                <textarea v-model="request.headers" class="form-textarea" placeholder='{"Authorization": "Bearer token", "Content-Type": "application/json"}'></textarea>
              </div>
              <div class="config-row">
                <label>Body:</label>
                <textarea v-model="request.body" class="form-textarea" placeholder='{"name": "New Task", "description": "Task description"}'></textarea>
              </div>
            </div>
          </div>
        </div>

        <div class="console-panel">
          <div class="panel-header">
            <h3><i class="fas fa-reply"></i> Response</h3>
            <div class="panel-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-copy"></i> Copy
              </button>
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-download"></i> Export
              </button>
            </div>
          </div>
          <div class="panel-content">
            <div class="response-info">
              <div class="response-status" :class="response.statusClass">
                <i :class="response.statusIcon"></i>
                {{ response.status }}
              </div>
              <div class="response-time">
                <i class="fas fa-clock"></i>
                {{ response.time }}
              </div>
              <div class="response-size">
                <i class="fas fa-file"></i>
                {{ response.size }}
              </div>
            </div>
            <div class="response-body">
              <pre><code>{{ response.body }}</code></pre>
            </div>
          </div>
        </div>
      </div>

      <div class="console-history">
        <div class="card">
          <div class="card-header">
            <h3><i class="fas fa-history"></i> Request History</h3>
          </div>
          <div class="card-content">
            <div class="history-list">
              <div class="history-item" v-for="item in requestHistory" :key="item.id">
                <div class="history-method" :class="item.method">{{ item.method }}</div>
                <div class="history-url">{{ item.url }}</div>
                <div class="history-status" :class="item.statusClass">{{ item.status }}</div>
                <div class="history-time">{{ item.time }}</div>
                <div class="history-actions">
                  <button class="btn btn-sm btn-outline" @click="loadHistoryItem(item)">
                    <i class="fas fa-redo"></i>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const request = ref({
  method: 'GET',
  url: '/api/tasks',
  headers: '{\n  "Authorization": "Bearer token",\n  "Content-Type": "application/json"\n}',
  body: ''
})

const response = ref({
  status: '200 OK',
  statusClass: 'success',
  statusIcon: 'fas fa-check-circle',
  time: '245ms',
  size: '1.2 KB',
  body: `{
  "success": true,
  "data": {
    "id": "task_123",
    "name": "New Task",
    "description": "Task description",
    "status": "pending",
    "created_at": "2024-01-15T10:30:00Z"
  }
}`
})

const requestHistory = ref([
  {
    id: 1,
    method: 'GET',
    url: '/api/tasks',
    status: '200',
    statusClass: 'success',
    time: '10:30 AM'
  },
  {
    id: 2,
    method: 'POST',
    url: '/api/tasks',
    status: '201',
    statusClass: 'success',
    time: '10:25 AM'
  },
  {
    id: 3,
    method: 'PUT',
    url: '/api/tasks/task_123',
    status: '200',
    statusClass: 'success',
    time: '10:20 AM'
  },
  {
    id: 4,
    method: 'DELETE',
    url: '/api/tasks/task_456',
    status: '404',
    statusClass: 'error',
    time: '10:15 AM'
  }
])

const sendRequest = async () => {
  const startTime = Date.now()
  
  try {
    // Parse headers
    let headers = {}
    try {
      headers = request.value.headers ? JSON.parse(request.value.headers) : {}
    } catch (e) {
      throw new Error('Invalid JSON in headers')
    }
    
    // Prepare request options
    const options = {
      method: request.value.method,
      headers
    }
    
    // Add body for non-GET requests
    if (request.value.method !== 'GET' && request.value.body) {
      try {
        options.body = request.value.body
      } catch (e) {
        throw new Error('Invalid JSON in body')
      }
    }
    
    // Make the actual API request
    const apiResponse = await window.apiClient.request(request.value.url, options)
    const endTime = Date.now()
    
    // Create history item
    const newHistoryItem = {
      id: Date.now(),
      method: request.value.method,
      url: request.value.url,
      status: '200',
      statusClass: 'success',
      time: new Date().toLocaleTimeString('en-US', { 
        hour: '2-digit', 
        minute: '2-digit',
        hour12: true 
      })
    }
    
    requestHistory.value.unshift(newHistoryItem)
    
    // Update response
    response.value.status = '200 OK'
    response.value.statusClass = 'success'
    response.value.statusIcon = 'fas fa-check-circle'
    response.value.time = `${endTime - startTime}ms`
    response.value.size = `${JSON.stringify(apiResponse).length} bytes`
    response.value.body = JSON.stringify(apiResponse, null, 2)
    
  } catch (error) {
    const endTime = Date.now()
    
    // Create history item for failed request
    const newHistoryItem = {
      id: Date.now(),
      method: request.value.method,
      url: request.value.url,
      status: error.message.includes('404') ? '404' : '500',
      statusClass: 'error',
      time: new Date().toLocaleTimeString('en-US', { 
        hour: '2-digit', 
        minute: '2-digit',
        hour12: true 
      })
    }
    
    requestHistory.value.unshift(newHistoryItem)
    
    // Update response with error
    const statusCode = error.message.includes('404') ? '404' : '500'
    response.value.status = `${statusCode} ${statusCode === '404' ? 'Not Found' : 'Internal Server Error'}`
    response.value.statusClass = 'error'
    response.value.statusIcon = 'fas fa-times-circle'
    response.value.time = `${endTime - startTime}ms`
    response.value.size = '0 bytes'
    response.value.body = JSON.stringify({
      error: error.message,
      timestamp: new Date().toISOString()
    }, null, 2)
  }
}

const loadHistoryItem = (item) => {
  request.value.method = item.method
  request.value.url = item.url
}

onMounted(() => {
  // Initialize with default request
})
</script>

<style scoped>
.console-page {
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: var(--space-6);
}

.page-header h1 {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin: 0 0 var(--space-2) 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
}

.page-header p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 16px;
}

.console-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-6);
  margin-bottom: var(--space-6);
}

.console-panel {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-4);
  border-bottom: 1px solid var(--border);
  background: var(--bg-secondary);
}

.panel-header h3 {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.panel-actions {
  display: flex;
  gap: var(--space-2);
}

.panel-content {
  padding: var(--space-4);
}

.request-config {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.config-row {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.config-row label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.response-info {
  display: flex;
  gap: var(--space-4);
  margin-bottom: var(--space-4);
  padding: var(--space-3);
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.response-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-weight: 600;
}

.response-status.success {
  color: var(--success);
}

.response-status.error {
  color: var(--error);
}

.response-time,
.response-size {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: 14px;
  color: var(--text-secondary);
}

.response-body {
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: var(--space-3);
  overflow-x: auto;
}

.response-body pre {
  margin: 0;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
  line-height: 1.4;
  color: var(--text-primary);
}

.console-history {
  margin-top: var(--space-6);
}

.card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.card-header {
  padding: var(--space-4);
  border-bottom: 1px solid var(--border);
  background: var(--bg-secondary);
}

.card-header h3 {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.card-content {
  padding: var(--space-4);
}

.history-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.history-item {
  display: grid;
  grid-template-columns: 80px 1fr 80px 100px 60px;
  gap: var(--space-3);
  align-items: center;
  padding: var(--space-3);
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  transition: var(--transition);
}

.history-item:hover {
  background: var(--bg-tertiary);
  border-color: var(--primary);
}

.history-method {
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  font-size: 11px;
  font-weight: 600;
  text-align: center;
  text-transform: uppercase;
}

.history-method.GET {
  background: var(--success-bg);
  color: var(--success);
}

.history-method.POST {
  background: var(--info-bg);
  color: var(--info);
}

.history-method.PUT {
  background: var(--warning-bg);
  color: var(--warning);
}

.history-method.DELETE {
  background: var(--error-bg);
  color: var(--error);
}

.history-url {
  font-family: monospace;
  font-size: 12px;
  color: var(--text-primary);
}

.history-status {
  font-size: 12px;
  font-weight: 600;
  text-align: center;
}

.history-status.success {
  color: var(--success);
}

.history-status.error {
  color: var(--error);
}

.history-time {
  font-size: 12px;
  color: var(--text-secondary);
  text-align: center;
}

.history-actions {
  display: flex;
  justify-content: center;
}

.form-select,
.form-input,
.form-textarea {
  padding: var(--space-3);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  transition: var(--transition);
}

.form-select:focus,
.form-input:focus,
.form-textarea:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 3px var(--primary-bg);
}

.form-textarea {
  resize: vertical;
  min-height: 100px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  font-weight: 500;
  text-decoration: none;
  cursor: pointer;
  transition: var(--transition);
}

.btn:hover {
  background: var(--bg-tertiary);
  border-color: var(--primary);
}

.btn-sm {
  padding: var(--space-1) var(--space-2);
  font-size: 12px;
}

.btn-primary {
  background: var(--primary);
  border-color: var(--primary);
  color: white;
}

.btn-primary:hover {
  background: var(--primary-hover);
  border-color: var(--primary-hover);
}

.btn-outline {
  background: transparent;
}

.btn-outline:hover {
  background: var(--bg-tertiary);
}
</style>