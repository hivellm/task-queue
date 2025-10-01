export default {
  name: 'Console',
  template: `
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
                  <i class="fas fa-save"></i>
                  Save
                </button>
                <button class="btn btn-sm btn-primary">
                  <i class="fas fa-play"></i>
                  Send
                </button>
              </div>
            </div>
            <div class="panel-content">
              <div class="request-config">
                <div class="config-row">
                  <label>Method:</label>
                  <select class="form-select">
                    <option value="GET">GET</option>
                    <option value="POST">POST</option>
                    <option value="PUT">PUT</option>
                    <option value="DELETE">DELETE</option>
                  </select>
                </div>
                <div class="config-row">
                  <label>URL:</label>
                  <input type="text" class="form-input" value="/api/tasks" placeholder="Enter API endpoint">
                </div>
                <div class="config-row">
                  <label>Headers:</label>
                  <textarea class="form-textarea" placeholder='{"Authorization": "Bearer token", "Content-Type": "application/json"}'></textarea>
                </div>
                <div class="config-row">
                  <label>Body:</label>
                  <textarea class="form-textarea" placeholder='{"name": "New Task", "description": "Task description"}'></textarea>
                </div>
              </div>
            </div>
          </div>
          
          <div class="console-panel">
            <div class="panel-header">
              <h3><i class="fas fa-reply"></i> Response</h3>
              <div class="panel-actions">
                <button class="btn btn-sm btn-outline">
                  <i class="fas fa-copy"></i>
                  Copy
                </button>
                <button class="btn btn-sm btn-outline">
                  <i class="fas fa-download"></i>
                  Export
                </button>
              </div>
            </div>
            <div class="panel-content">
              <div class="response-info">
                <div class="response-status success">
                  <i class="fas fa-check-circle"></i>
                  200 OK
                </div>
                <div class="response-time">
                  <i class="fas fa-clock"></i>
                  245ms
                </div>
                <div class="response-size">
                  <i class="fas fa-file"></i>
                  1.2 KB
                </div>
              </div>
              <div class="response-body">
                <pre><code>{
  "success": true,
  "data": {
    "id": "task_123",
    "name": "New Task",
    "description": "Task description",
    "status": "pending",
    "created_at": "2024-01-15T10:30:00Z"
  }
}</code></pre>
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
                <div class="history-item">
                  <div class="history-method GET">GET</div>
                  <div class="history-url">/api/tasks</div>
                  <div class="history-status success">200</div>
                  <div class="history-time">10:30 AM</div>
                  <div class="history-actions">
                    <button class="btn btn-sm btn-outline">
                      <i class="fas fa-redo"></i>
                    </button>
                  </div>
                </div>
                <div class="history-item">
                  <div class="history-method POST">POST</div>
                  <div class="history-url">/api/tasks</div>
                  <div class="history-status success">201</div>
                  <div class="history-time">10:25 AM</div>
                  <div class="history-actions">
                    <button class="btn btn-sm btn-outline">
                      <i class="fas fa-redo"></i>
                    </button>
                  </div>
                </div>
                <div class="history-item">
                  <div class="history-method PUT">PUT</div>
                  <div class="history-url">/api/tasks/task_123</div>
                  <div class="history-status success">200</div>
                  <div class="history-time">10:20 AM</div>
                  <div class="history-actions">
                    <button class="btn btn-sm btn-outline">
                      <i class="fas fa-redo"></i>
                    </button>
                  </div>
                </div>
                <div class="history-item">
                  <div class="history-method DELETE">DELETE</div>
                  <div class="history-url">/api/tasks/task_456</div>
                  <div class="history-status error">404</div>
                  <div class="history-time">10:15 AM</div>
                  <div class="history-actions">
                    <button class="btn btn-sm btn-outline">
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
  `
}
