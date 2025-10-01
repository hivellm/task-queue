export default {
  name: 'Workflows',
  template: `
    <div class="workflows-page">
      <div class="page-header">
        <h1><i class="fas fa-project-diagram"></i> Workflows</h1>
        <p>Manage task workflows and automation</p>
        <button class="btn btn-primary">
          <i class="fas fa-plus"></i> New Workflow
        </button>
      </div>
      
      <div class="workflows-content">
        <div class="workflows-grid">
          <div class="workflow-card">
            <div class="workflow-header">
              <div class="workflow-icon">
                <i class="fas fa-code-branch"></i>
              </div>
              <div class="workflow-info">
                <h3>CI/CD Pipeline</h3>
                <p>Automated build and deployment workflow</p>
              </div>
              <div class="workflow-status active">
                <i class="fas fa-play-circle"></i>
                Active
              </div>
            </div>
            <div class="workflow-steps">
              <div class="step completed">
                <div class="step-icon">
                  <i class="fas fa-check"></i>
                </div>
                <div class="step-content">
                  <h4>Code Checkout</h4>
                  <p>Pull latest code from repository</p>
                </div>
              </div>
              <div class="step completed">
                <div class="step-icon">
                  <i class="fas fa-check"></i>
                </div>
                <div class="step-content">
                  <h4>Run Tests</h4>
                  <p>Execute unit and integration tests</p>
                </div>
              </div>
              <div class="step running">
                <div class="step-icon">
                  <i class="fas fa-spinner fa-spin"></i>
                </div>
                <div class="step-content">
                  <h4>Build Application</h4>
                  <p>Compile and package application</p>
                </div>
              </div>
              <div class="step pending">
                <div class="step-icon">
                  <i class="fas fa-clock"></i>
                </div>
                <div class="step-content">
                  <h4>Deploy</h4>
                  <p>Deploy to staging environment</p>
                </div>
              </div>
            </div>
            <div class="workflow-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-eye"></i>
                View
              </button>
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-edit"></i>
                Edit
              </button>
              <button class="btn btn-sm btn-danger">
                <i class="fas fa-stop"></i>
                Stop
              </button>
            </div>
          </div>
          
          <div class="workflow-card">
            <div class="workflow-header">
              <div class="workflow-icon">
                <i class="fas fa-database"></i>
              </div>
              <div class="workflow-info">
                <h3>Data Processing</h3>
                <p>Automated data validation and processing</p>
              </div>
              <div class="workflow-status completed">
                <i class="fas fa-check-circle"></i>
                Completed
              </div>
            </div>
            <div class="workflow-steps">
              <div class="step completed">
                <div class="step-icon">
                  <i class="fas fa-check"></i>
                </div>
                <div class="step-content">
                  <h4>Data Import</h4>
                  <p>Import data from external sources</p>
                </div>
              </div>
              <div class="step completed">
                <div class="step-icon">
                  <i class="fas fa-check"></i>
                </div>
                <div class="step-content">
                  <h4>Validation</h4>
                  <p>Validate data integrity and format</p>
                </div>
              </div>
              <div class="step completed">
                <div class="step-icon">
                  <i class="fas fa-check"></i>
                </div>
                <div class="step-content">
                  <h4>Processing</h4>
                  <p>Transform and process data</p>
                </div>
              </div>
              <div class="step completed">
                <div class="step-icon">
                  <i class="fas fa-check"></i>
                </div>
                <div class="step-content">
                  <h4>Export</h4>
                  <p>Export processed data</p>
                </div>
              </div>
            </div>
            <div class="workflow-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-eye"></i>
                View
              </button>
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-edit"></i>
                Edit
              </button>
              <button class="btn btn-sm btn-primary">
                <i class="fas fa-play"></i>
                Run
              </button>
            </div>
          </div>
          
          <div class="workflow-card">
            <div class="workflow-header">
              <div class="workflow-icon">
                <i class="fas fa-bell"></i>
              </div>
              <div class="workflow-info">
                <h3>Notification System</h3>
                <p>Automated notifications and alerts</p>
              </div>
              <div class="workflow-status inactive">
                <i class="fas fa-pause-circle"></i>
                Inactive
              </div>
            </div>
            <div class="workflow-steps">
              <div class="step pending">
                <div class="step-icon">
                  <i class="fas fa-clock"></i>
                </div>
                <div class="step-content">
                  <h4>Monitor Events</h4>
                  <p>Monitor system events and triggers</p>
                </div>
              </div>
              <div class="step pending">
                <div class="step-icon">
                  <i class="fas fa-clock"></i>
                </div>
                <div class="step-content">
                  <h4>Filter Rules</h4>
                  <p>Apply filtering and routing rules</p>
                </div>
              </div>
              <div class="step pending">
                <div class="step-icon">
                  <i class="fas fa-clock"></i>
                </div>
                <div class="step-content">
                  <h4>Send Notifications</h4>
                  <p>Send emails and push notifications</p>
                </div>
              </div>
            </div>
            <div class="workflow-actions">
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-eye"></i>
                View
              </button>
              <button class="btn btn-sm btn-outline">
                <i class="fas fa-edit"></i>
                Edit
              </button>
              <button class="btn btn-sm btn-primary">
                <i class="fas fa-play"></i>
                Activate
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  `
}
