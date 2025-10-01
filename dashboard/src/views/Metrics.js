export default {
  name: 'Metrics',
  template: `
    <div class="metrics-page">
      <div class="page-header">
        <h1><i class="fas fa-chart-line"></i> Metrics</h1>
        <p>System performance and analytics</p>
      </div>
      
      <div class="metrics-content">
        <div class="metrics-grid">
          <div class="metric-card">
            <div class="metric-header">
              <h3><i class="fas fa-tachometer-alt"></i> Performance</h3>
            </div>
            <div class="metric-chart">
              <div class="chart-placeholder">
                <i class="fas fa-chart-area"></i>
                <p>CPU Usage: 45%</p>
                <p>Memory Usage: 67%</p>
                <p>Disk Usage: 23%</p>
              </div>
            </div>
          </div>
          
          <div class="metric-card">
            <div class="metric-header">
              <h3><i class="fas fa-tasks"></i> Task Statistics</h3>
            </div>
            <div class="metric-chart">
              <div class="chart-placeholder">
                <i class="fas fa-chart-pie"></i>
                <p>Total Tasks: 24</p>
                <p>Completed: 16 (67%)</p>
                <p>Failed: 3 (12%)</p>
                <p>Running: 5 (21%)</p>
              </div>
            </div>
          </div>
          
          <div class="metric-card">
            <div class="metric-header">
              <h3><i class="fas fa-clock"></i> Response Times</h3>
            </div>
            <div class="metric-chart">
              <div class="chart-placeholder">
                <i class="fas fa-chart-line"></i>
                <p>Average: 245ms</p>
                <p>P95: 890ms</p>
                <p>P99: 1.2s</p>
              </div>
            </div>
          </div>
          
          <div class="metric-card">
            <div class="metric-header">
              <h3><i class="fas fa-users"></i> Usage</h3>
            </div>
            <div class="metric-chart">
              <div class="chart-placeholder">
                <i class="fas fa-chart-bar"></i>
                <p>Active Users: 12</p>
                <p>API Calls: 1,234</p>
                <p>Requests/min: 45</p>
              </div>
            </div>
          </div>
        </div>
        
        <div class="metrics-details">
          <div class="card">
            <div class="card-header">
              <h3><i class="fas fa-chart-line"></i> System Performance</h3>
            </div>
            <div class="card-content">
              <div class="performance-metrics">
                <div class="metric-item">
                  <div class="metric-label">CPU Usage</div>
                  <div class="metric-bar">
                    <div class="metric-fill" style="width: 45%"></div>
                  </div>
                  <div class="metric-value">45%</div>
                </div>
                <div class="metric-item">
                  <div class="metric-label">Memory Usage</div>
                  <div class="metric-bar">
                    <div class="metric-fill" style="width: 67%"></div>
                  </div>
                  <div class="metric-value">67%</div>
                </div>
                <div class="metric-item">
                  <div class="metric-label">Disk Usage</div>
                  <div class="metric-bar">
                    <div class="metric-fill" style="width: 23%"></div>
                  </div>
                  <div class="metric-value">23%</div>
                </div>
                <div class="metric-item">
                  <div class="metric-label">Network I/O</div>
                  <div class="metric-bar">
                    <div class="metric-fill" style="width: 12%"></div>
                  </div>
                  <div class="metric-value">12%</div>
                </div>
              </div>
            </div>
          </div>
          
          <div class="card">
            <div class="card-header">
              <h3><i class="fas fa-history"></i> Recent Activity</h3>
            </div>
            <div class="card-content">
              <div class="activity-timeline">
                <div class="timeline-item">
                  <div class="timeline-time">10:30 AM</div>
                  <div class="timeline-content">
                    <div class="timeline-title">Task "Update Documentation" completed</div>
                    <div class="timeline-desc">Execution time: 2.3s</div>
                  </div>
                </div>
                <div class="timeline-item">
                  <div class="timeline-time">10:25 AM</div>
                  <div class="timeline-content">
                    <div class="timeline-title">New task "API Testing" created</div>
                    <div class="timeline-desc">Priority: Medium</div>
                  </div>
                </div>
                <div class="timeline-item">
                  <div class="timeline-time">10:20 AM</div>
                  <div class="timeline-content">
                    <div class="timeline-title">Task "Database Migration" failed</div>
                    <div class="timeline-desc">Error: Connection timeout</div>
                  </div>
                </div>
                <div class="timeline-item">
                  <div class="timeline-time">10:15 AM</div>
                  <div class="timeline-content">
                    <div class="timeline-title">System performance check</div>
                    <div class="timeline-desc">All systems normal</div>
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
