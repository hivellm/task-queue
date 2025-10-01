export default {
  name: 'Dependencies',
  template: `
    <div class="dependencies-page">
      <div class="page-header">
        <h1><i class="fas fa-sitemap"></i> Dependencies</h1>
        <p>Manage task dependencies and relationships</p>
        <button class="btn btn-primary">
          <i class="fas fa-plus"></i> New Dependency
        </button>
      </div>
      
      <div class="dependencies-content">
        <div class="dependencies-grid">
          <div class="dependency-card">
            <div class="dependency-header">
              <div class="dependency-info">
                <h3>Database Migration</h3>
                <p>Task Queue API</p>
              </div>
              <div class="dependency-status completed">
                <i class="fas fa-check-circle"></i>
                Completed
              </div>
            </div>
            <div class="dependency-details">
              <div class="dependency-item">
                <span class="dependency-label">Depends on:</span>
                <span class="dependency-value">Schema Design</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Blocks:</span>
                <span class="dependency-value">API Development</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Priority:</span>
                <span class="dependency-value critical">Critical</span>
              </div>
            </div>
            <div class="dependency-actions">
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
          
          <div class="dependency-card">
            <div class="dependency-header">
              <div class="dependency-info">
                <h3>API Development</h3>
                <p>Task Queue API</p>
              </div>
              <div class="dependency-status running">
                <i class="fas fa-play-circle"></i>
                Running
              </div>
            </div>
            <div class="dependency-details">
              <div class="dependency-item">
                <span class="dependency-label">Depends on:</span>
                <span class="dependency-value">Database Migration</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Blocks:</span>
                <span class="dependency-value">Frontend Integration</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Priority:</span>
                <span class="dependency-value high">High</span>
              </div>
            </div>
            <div class="dependency-actions">
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
          
          <div class="dependency-card">
            <div class="dependency-header">
              <div class="dependency-info">
                <h3>Frontend Integration</h3>
                <p>Dashboard UI</p>
              </div>
              <div class="dependency-status pending">
                <i class="fas fa-clock"></i>
                Pending
              </div>
            </div>
            <div class="dependency-details">
              <div class="dependency-item">
                <span class="dependency-label">Depends on:</span>
                <span class="dependency-value">API Development</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Blocks:</span>
                <span class="dependency-value">Testing</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Priority:</span>
                <span class="dependency-value medium">Medium</span>
              </div>
            </div>
            <div class="dependency-actions">
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
          
          <div class="dependency-card">
            <div class="dependency-header">
              <div class="dependency-info">
                <h3>Testing</h3>
                <p>Task Queue API</p>
              </div>
              <div class="dependency-status pending">
                <i class="fas fa-clock"></i>
                Pending
              </div>
            </div>
            <div class="dependency-details">
              <div class="dependency-item">
                <span class="dependency-label">Depends on:</span>
                <span class="dependency-value">Frontend Integration</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Blocks:</span>
                <span class="dependency-value">Deployment</span>
              </div>
              <div class="dependency-item">
                <span class="dependency-label">Priority:</span>
                <span class="dependency-value high">High</span>
              </div>
            </div>
            <div class="dependency-actions">
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
        </div>
        
        <div class="dependencies-visualization">
          <div class="card">
            <div class="card-header">
              <h3><i class="fas fa-project-diagram"></i> Dependency Graph</h3>
            </div>
            <div class="card-content">
              <div class="dependency-graph">
                <div class="graph-node completed">
                  <div class="node-icon">
                    <i class="fas fa-database"></i>
                  </div>
                  <div class="node-label">Schema Design</div>
                </div>
                <div class="graph-arrow">→</div>
                <div class="graph-node completed">
                  <div class="node-icon">
                    <i class="fas fa-sync"></i>
                  </div>
                  <div class="node-label">Database Migration</div>
                </div>
                <div class="graph-arrow">→</div>
                <div class="graph-node running">
                  <div class="node-icon">
                    <i class="fas fa-code"></i>
                  </div>
                  <div class="node-label">API Development</div>
                </div>
                <div class="graph-arrow">→</div>
                <div class="graph-node pending">
                  <div class="node-icon">
                    <i class="fas fa-palette"></i>
                  </div>
                  <div class="node-label">Frontend Integration</div>
                </div>
                <div class="graph-arrow">→</div>
                <div class="graph-node pending">
                  <div class="node-icon">
                    <i class="fas fa-vial"></i>
                  </div>
                  <div class="node-label">Testing</div>
                </div>
                <div class="graph-arrow">→</div>
                <div class="graph-node pending">
                  <div class="node-icon">
                    <i class="fas fa-rocket"></i>
                  </div>
                  <div class="node-label">Deployment</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  `
}
