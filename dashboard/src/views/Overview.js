export default {
  name: 'Overview',
  template: `
    <div class="overview-page">
      <div class="page-header">
        <h1><i class="fas fa-tachometer-alt"></i> Overview</h1>
        <p>Dashboard overview and system statistics</p>
      </div>
      
      <div class="dashboard-grid">
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-icon">
              <i class="fas fa-tasks"></i>
            </div>
            <div class="stat-content">
              <h3>Total Tasks</h3>
              <div class="stat-value">24</div>
              <div class="stat-change positive">+12%</div>
            </div>
          </div>
          
          <div class="stat-card">
            <div class="stat-icon">
              <i class="fas fa-play-circle"></i>
            </div>
            <div class="stat-content">
              <h3>Active Tasks</h3>
              <div class="stat-value">8</div>
              <div class="stat-change neutral">0%</div>
            </div>
          </div>
          
          <div class="stat-card">
            <div class="stat-icon">
              <i class="fas fa-check-circle"></i>
            </div>
            <div class="stat-content">
              <h3>Completed</h3>
              <div class="stat-value">16</div>
              <div class="stat-change positive">+25%</div>
            </div>
          </div>
          
          <div class="stat-card">
            <div class="stat-icon">
              <i class="fas fa-folder"></i>
            </div>
            <div class="stat-content">
              <h3>Projects</h3>
              <div class="stat-value">3</div>
              <div class="stat-change positive">+1</div>
            </div>
          </div>
        </div>
        
        <div class="content-grid">
          <div class="card">
            <div class="card-header">
              <h3><i class="fas fa-chart-line"></i> Recent Activity</h3>
            </div>
            <div class="card-content">
              <div class="activity-list">
                <div class="activity-item">
                  <div class="activity-icon success">
                    <i class="fas fa-check"></i>
                  </div>
                  <div class="activity-content">
                    <div class="activity-title">Task "Update Documentation" completed</div>
                    <div class="activity-time">2 hours ago</div>
                  </div>
                </div>
                
                <div class="activity-item">
                  <div class="activity-icon info">
                    <i class="fas fa-plus"></i>
                  </div>
                  <div class="activity-content">
                    <div class="activity-title">New task "API Testing" created</div>
                    <div class="activity-time">4 hours ago</div>
                  </div>
                </div>
                
                <div class="activity-item">
                  <div class="activity-icon warning">
                    <i class="fas fa-exclamation"></i>
                  </div>
                  <div class="activity-content">
                    <div class="activity-title">Task "Database Migration" failed</div>
                    <div class="activity-time">6 hours ago</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="card">
            <div class="card-header">
              <h3><i class="fas fa-list"></i> Quick Actions</h3>
            </div>
            <div class="card-content">
              <div class="quick-actions">
                <button class="btn btn-primary">
                  <i class="fas fa-plus"></i>
                  Create Task
                </button>
                <button class="btn btn-outline">
                  <i class="fas fa-folder"></i>
                  New Project
                </button>
                <button class="btn btn-outline">
                  <i class="fas fa-key"></i>
                  Manage API Keys
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  `
}
