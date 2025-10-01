export default {
  name: 'Projects',
  template: `
    <div class="projects-page">
      <div class="page-header">
        <h1><i class="fas fa-folder"></i> Projects</h1>
        <p>Manage your projects and their tasks</p>
        <button class="btn btn-primary">
          <i class="fas fa-plus"></i> New Project
        </button>
      </div>
      
      <div class="projects-grid">
        <div class="project-card">
          <div class="project-header">
            <div class="project-icon">
              <i class="fas fa-code"></i>
            </div>
            <div class="project-info">
              <h3>Task Queue API</h3>
              <p>Backend API for task management system</p>
            </div>
            <div class="project-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-edit"></i>
              </button>
              <button class="btn btn-sm btn-danger">
                <i class="fas fa-trash"></i>
              </button>
            </div>
          </div>
          <div class="project-stats">
            <div class="stat">
              <span class="stat-label">Tasks</span>
              <span class="stat-value">12</span>
            </div>
            <div class="stat">
              <span class="stat-label">Active</span>
              <span class="stat-value">3</span>
            </div>
            <div class="stat">
              <span class="stat-label">Completed</span>
              <span class="stat-value">9</span>
            </div>
          </div>
          <div class="project-progress">
            <div class="progress-bar">
              <div class="progress-fill" style="width: 75%"></div>
            </div>
            <span class="progress-text">75% Complete</span>
          </div>
        </div>
        
        <div class="project-card">
          <div class="project-header">
            <div class="project-icon">
              <i class="fas fa-palette"></i>
            </div>
            <div class="project-info">
              <h3>Dashboard UI</h3>
              <p>Frontend dashboard for task queue management</p>
            </div>
            <div class="project-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-edit"></i>
              </button>
              <button class="btn btn-sm btn-danger">
                <i class="fas fa-trash"></i>
              </button>
            </div>
          </div>
          <div class="project-stats">
            <div class="stat">
              <span class="stat-label">Tasks</span>
              <span class="stat-value">8</span>
            </div>
            <div class="stat">
              <span class="stat-label">Active</span>
              <span class="stat-value">5</span>
            </div>
            <div class="stat">
              <span class="stat-label">Completed</span>
              <span class="stat-value">3</span>
            </div>
          </div>
          <div class="project-progress">
            <div class="progress-bar">
              <div class="progress-fill" style="width: 37.5%"></div>
            </div>
            <span class="progress-text">37.5% Complete</span>
          </div>
        </div>
        
        <div class="project-card">
          <div class="project-header">
            <div class="project-icon">
              <i class="fas fa-database"></i>
            </div>
            <div class="project-info">
              <h3>Database Migration</h3>
              <p>Migrate to new database schema</p>
            </div>
            <div class="project-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-edit"></i>
              </button>
              <button class="btn btn-sm btn-danger">
                <i class="fas fa-trash"></i>
              </button>
            </div>
          </div>
          <div class="project-stats">
            <div class="stat">
              <span class="stat-label">Tasks</span>
              <span class="stat-value">4</span>
            </div>
            <div class="stat">
              <span class="stat-label">Active</span>
              <span class="stat-value">0</span>
            </div>
            <div class="stat">
              <span class="stat-label">Completed</span>
              <span class="stat-value">4</span>
            </div>
          </div>
          <div class="project-progress">
            <div class="progress-bar">
              <div class="progress-fill" style="width: 100%"></div>
            </div>
            <span class="progress-text">100% Complete</span>
          </div>
        </div>
      </div>
    </div>
  `
}
