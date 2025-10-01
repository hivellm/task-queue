export default {
  name: 'Tasks',
  template: `
    <div class="tasks-page">
      <div class="page-header">
        <h1><i class="fas fa-tasks"></i> Tasks</h1>
        <p>View and manage all tasks</p>
        <button class="btn btn-primary">
          <i class="fas fa-plus"></i> New Task
        </button>
      </div>
      
      <div class="tasks-content">
        <div class="tasks-filters">
          <div class="filter-group">
            <select class="form-select">
              <option value="">All Status</option>
              <option value="pending">Pending</option>
              <option value="running">Running</option>
              <option value="completed">Completed</option>
              <option value="failed">Failed</option>
            </select>
            <select class="form-select">
              <option value="">All Projects</option>
              <option value="api">Task Queue API</option>
              <option value="ui">Dashboard UI</option>
              <option value="db">Database Migration</option>
            </select>
            <input type="text" class="form-input" placeholder="Search tasks...">
          </div>
        </div>
        
        <div class="tasks-grid">
          <div class="task-card">
            <div class="task-header">
              <div class="task-title">
                <h3>Update Documentation</h3>
                <span class="task-id">#TASK-001</span>
              </div>
              <div class="task-status completed">
                <i class="fas fa-check-circle"></i>
                Completed
              </div>
            </div>
            <div class="task-meta">
              <span class="task-project">
                <i class="fas fa-folder"></i>
                Task Queue API
              </span>
              <span class="task-priority high">
                <i class="fas fa-exclamation-triangle"></i>
                High
              </span>
              <span class="task-created">
                <i class="fas fa-clock"></i>
                2 hours ago
              </span>
            </div>
            <div class="task-description">
              Update API documentation with new endpoints and examples
            </div>
            <div class="task-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-eye"></i>
                View
              </button>
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-edit"></i>
                Edit
              </button>
            </div>
          </div>
          
          <div class="task-card">
            <div class="task-header">
              <div class="task-title">
                <h3>API Testing</h3>
                <span class="task-id">#TASK-002</span>
              </div>
              <div class="task-status running">
                <i class="fas fa-play-circle"></i>
                Running
              </div>
            </div>
            <div class="task-meta">
              <span class="task-project">
                <i class="fas fa-folder"></i>
                Task Queue API
              </span>
              <span class="task-priority medium">
                <i class="fas fa-minus"></i>
                Medium
              </span>
              <span class="task-created">
                <i class="fas fa-clock"></i>
                4 hours ago
              </span>
            </div>
            <div class="task-description">
              Implement comprehensive API testing suite
            </div>
            <div class="task-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-eye"></i>
                View
              </button>
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-pause"></i>
                Pause
              </button>
            </div>
          </div>
          
          <div class="task-card">
            <div class="task-header">
              <div class="task-title">
                <h3>Database Migration</h3>
                <span class="task-id">#TASK-003</span>
              </div>
              <div class="task-status failed">
                <i class="fas fa-times-circle"></i>
                Failed
              </div>
            </div>
            <div class="task-meta">
              <span class="task-project">
                <i class="fas fa-folder"></i>
                Database Migration
              </span>
              <span class="task-priority critical">
                <i class="fas fa-exclamation-circle"></i>
                Critical
              </span>
              <span class="task-created">
                <i class="fas fa-clock"></i>
                6 hours ago
              </span>
            </div>
            <div class="task-description">
              Migrate database schema to new version
            </div>
            <div class="task-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-eye"></i>
                View
              </button>
              <button class="btn btn-sm btn-primary">
                <i class="fas fa-redo"></i>
                Retry
              </button>
            </div>
          </div>
          
          <div class="task-card">
            <div class="task-header">
              <div class="task-title">
                <h3>UI Components</h3>
                <span class="task-id">#TASK-004</span>
              </div>
              <div class="task-status pending">
                <i class="fas fa-clock"></i>
                Pending
              </div>
            </div>
            <div class="task-meta">
              <span class="task-project">
                <i class="fas fa-folder"></i>
                Dashboard UI
              </span>
              <span class="task-priority low">
                <i class="fas fa-arrow-down"></i>
                Low
              </span>
              <span class="task-created">
                <i class="fas fa-clock"></i>
                1 day ago
              </span>
            </div>
            <div class="task-description">
              Create reusable UI components for dashboard
            </div>
            <div class="task-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-eye"></i>
                View
              </button>
              <button class="btn btn-sm btn-primary">
                <i class="fas fa-play"></i>
                Start
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  `
}
