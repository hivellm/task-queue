<template>
  <div class="workflows-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-text">
      <h1><i class="fas fa-project-diagram"></i> Workflows</h1>
      <p>Manage task workflows and automation</p>
    </div>
        <div class="header-actions">
          <button class="btn btn-outline" @click="refreshWorkflows">
            <i class="fas fa-refresh"></i> Refresh
          </button>
          <button class="btn btn-primary" @click="showCreateModal = true">
            <i class="fas fa-plus"></i> New Workflow
          </button>
        </div>
      </div>
    </div>

    <div class="workflows-content">
      <div class="workflows-filters">
        <div class="filter-group">
          <div class="filter-item">
            <label>Status:</label>
            <select v-model="filters.status" class="form-select">
              <option value="">All Status</option>
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
              <option value="completed">Completed</option>
              <option value="failed">Failed</option>
            </select>
          </div>
          <div class="filter-item">
            <label>Type:</label>
            <select v-model="filters.type" class="form-select">
              <option value="">All Types</option>
              <option value="cicd">CI/CD</option>
              <option value="data">Data Processing</option>
              <option value="notification">Notification</option>
              <option value="deployment">Deployment</option>
            </select>
          </div>
          <div class="filter-item">
            <label>Search:</label>
            <input 
              v-model="filters.search" 
              type="text" 
              class="form-input" 
              placeholder="Search workflows..."
            >
          </div>
        </div>
      </div>

      <div class="workflows-grid">
        <div 
          class="workflow-card" 
          v-for="workflow in filteredWorkflows" 
          :key="workflow.id"
          @click="selectWorkflow(workflow)"
        >
          <div class="workflow-header">
            <div class="workflow-icon">
              <i :class="getWorkflowIcon(workflow.type)"></i>
            </div>
            <div class="workflow-info">
              <h3 class="workflow-name">{{ workflow.name }}</h3>
              <p class="workflow-description">{{ workflow.description }}</p>
            </div>
            <div class="workflow-status" :class="workflow.status">
              <i :class="getStatusIcon(workflow.status)"></i>
              {{ workflow.status }}
            </div>
          </div>

          <div class="workflow-metrics">
            <div class="metric">
              <div class="metric-label">Executions</div>
              <div class="metric-value">{{ workflow.executions }}</div>
            </div>
            <div class="metric">
              <div class="metric-label">Success Rate</div>
              <div class="metric-value">{{ workflow.successRate }}%</div>
            </div>
            <div class="metric">
              <div class="metric-label">Avg Duration</div>
              <div class="metric-value">{{ workflow.avgDuration }}</div>
            </div>
          </div>

          <div class="workflow-steps">
            <div class="steps-header">
              <span class="steps-title">Workflow Steps</span>
              <span class="steps-count">{{ workflow.steps.length }} steps</span>
            </div>
            <div class="steps-list">
              <div 
                class="step-item" 
                v-for="(step, index) in workflow.steps.slice(0, 4)" 
                :key="index"
                :class="step.status"
              >
                <div class="step-icon">
                  <i :class="getStepIcon(step.status)"></i>
                </div>
                <div class="step-content">
                  <div class="step-name">{{ step.name }}</div>
                  <div class="step-description">{{ step.description }}</div>
                </div>
              </div>
              <div v-if="workflow.steps.length > 4" class="step-more">
                +{{ workflow.steps.length - 4 }} more steps
              </div>
            </div>
          </div>

          <div class="workflow-details">
            <div class="detail-item">
              <i class="fas fa-calendar"></i>
              <span>Last Run: {{ formatDate(workflow.lastRun) }}</span>
            </div>
            <div class="detail-item">
              <i class="fas fa-clock"></i>
              <span>Next Run: {{ workflow.nextRun || 'Manual' }}</span>
            </div>
          </div>

          <div class="workflow-actions">
            <button class="btn btn-sm btn-outline" @click.stop="viewWorkflow(workflow)">
              <i class="fas fa-eye"></i> View
            </button>
            <button class="btn btn-sm btn-outline" @click.stop="editWorkflow(workflow)">
              <i class="fas fa-edit"></i> Edit
            </button>
            <button 
              class="btn btn-sm btn-success" 
              @click.stop="runWorkflow(workflow)"
              v-if="workflow.status === 'active'"
            >
              <i class="fas fa-play"></i> Run
            </button>
            <button 
              class="btn btn-sm btn-warning" 
              @click.stop="toggleWorkflow(workflow)"
            >
              <i :class="workflow.status === 'active' ? 'fas fa-pause' : 'fas fa-play'"></i>
              {{ workflow.status === 'active' ? 'Pause' : 'Start' }}
            </button>
            <button class="btn btn-sm btn-danger" @click.stop="deleteWorkflow(workflow)">
              <i class="fas fa-trash"></i> Delete
            </button>
          </div>
        </div>
      </div>

      <div v-if="filteredWorkflows.length === 0" class="empty-state">
        <div class="empty-icon">
          <i class="fas fa-project-diagram"></i>
        </div>
        <h3>No workflows found</h3>
        <p>Create your first workflow to automate tasks</p>
        <button class="btn btn-primary" @click="showCreateModal = true">
          <i class="fas fa-plus"></i> Create Workflow
        </button>
      </div>
    </div>

    <!-- Create Workflow Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click="showCreateModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h3>Create New Workflow</h3>
          <button class="btn btn-sm btn-outline" @click="showCreateModal = false">
            <i class="fas fa-times"></i>
          </button>
        </div>
        <div class="modal-content">
          <form @submit.prevent="createWorkflow">
            <div class="form-group">
              <label>Workflow Name</label>
              <input v-model="newWorkflow.name" type="text" class="form-input" required>
            </div>
            <div class="form-group">
              <label>Description</label>
              <textarea v-model="newWorkflow.description" class="form-textarea" rows="3"></textarea>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>Type</label>
                <select v-model="newWorkflow.type" class="form-select" required>
                  <option value="cicd">CI/CD</option>
                  <option value="data">Data Processing</option>
                  <option value="notification">Notification</option>
                  <option value="deployment">Deployment</option>
                </select>
              </div>
              <div class="form-group">
                <label>Status</label>
                <select v-model="newWorkflow.status" class="form-select">
                  <option value="active">Active</option>
                  <option value="inactive">Inactive</option>
                </select>
              </div>
            </div>
            <div class="form-group">
              <label>Schedule (Cron Expression)</label>
              <input v-model="newWorkflow.schedule" type="text" class="form-input" placeholder="0 0 * * * (daily at midnight)">
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <button class="btn btn-outline" @click="showCreateModal = false">Cancel</button>
          <button class="btn btn-primary" @click="createWorkflow">Create Workflow</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

// Reactive data
const workflows = ref([])
const showCreateModal = ref(false)
const loading = ref(true)
const error = ref(null)
const filters = ref({
  status: '',
  type: '',
  search: ''
})

const newWorkflow = ref({
  name: '',
  description: '',
  type: 'cicd',
  status: 'active',
  schedule: ''
})

// Mock data
const mockWorkflows = [
  {
    id: 1,
    name: 'CI/CD Pipeline',
    description: 'Automated build and deployment workflow for the main application',
    type: 'cicd',
    status: 'active',
    executions: 245,
    successRate: 94,
    avgDuration: '8m 32s',
    lastRun: new Date('2024-01-15'),
    nextRun: '2024-01-16 02:00',
    steps: [
      { name: 'Code Checkout', description: 'Pull latest code from repository', status: 'completed' },
      { name: 'Run Tests', description: 'Execute unit and integration tests', status: 'completed' },
      { name: 'Build Application', description: 'Compile and package application', status: 'running' },
      { name: 'Deploy', description: 'Deploy to staging environment', status: 'pending' },
      { name: 'Run E2E Tests', description: 'Execute end-to-end tests', status: 'pending' },
      { name: 'Deploy to Production', description: 'Deploy to production environment', status: 'pending' }
    ]
  },
  {
    id: 2,
    name: 'Data Processing',
    description: 'Automated data validation and processing pipeline',
    type: 'data',
    status: 'active',
    executions: 156,
    successRate: 98,
    avgDuration: '12m 15s',
    lastRun: new Date('2024-01-15'),
    nextRun: '2024-01-15 06:00',
    steps: [
      { name: 'Data Import', description: 'Import data from external sources', status: 'completed' },
      { name: 'Validation', description: 'Validate data integrity and format', status: 'completed' },
      { name: 'Processing', description: 'Transform and process data', status: 'completed' },
      { name: 'Export', description: 'Export processed data', status: 'completed' }
    ]
  },
  {
    id: 3,
    name: 'Notification System',
    description: 'Automated notifications and alerts for system events',
    type: 'notification',
    status: 'inactive',
    executions: 89,
    successRate: 92,
    avgDuration: '2m 45s',
    lastRun: new Date('2024-01-10'),
    nextRun: null,
    steps: [
      { name: 'Monitor Events', description: 'Monitor system events and triggers', status: 'pending' },
      { name: 'Filter Rules', description: 'Apply filtering and routing rules', status: 'pending' },
      { name: 'Send Notifications', description: 'Send emails and push notifications', status: 'pending' }
    ]
  },
  {
    id: 4,
    name: 'Database Backup',
    description: 'Automated database backup and archival process',
    type: 'deployment',
    status: 'active',
    executions: 30,
    successRate: 100,
    avgDuration: '15m 20s',
    lastRun: new Date('2024-01-14'),
    nextRun: '2024-01-16 03:00',
    steps: [
      { name: 'Create Snapshot', description: 'Create database snapshot', status: 'completed' },
      { name: 'Compress Backup', description: 'Compress and encrypt backup', status: 'completed' },
      { name: 'Upload to Storage', description: 'Upload backup to cloud storage', status: 'completed' },
      { name: 'Verify Backup', description: 'Verify backup integrity', status: 'completed' }
    ]
  },
  {
    id: 5,
    name: 'Performance Monitoring',
    description: 'System performance monitoring and reporting',
    type: 'data',
    status: 'active',
    executions: 720,
    successRate: 96,
    avgDuration: '1m 30s',
    lastRun: new Date('2024-01-15'),
    nextRun: '2024-01-15 16:00',
    steps: [
      { name: 'Collect Metrics', description: 'Collect system performance metrics', status: 'completed' },
      { name: 'Analyze Data', description: 'Analyze performance data', status: 'completed' },
      { name: 'Generate Report', description: 'Generate performance report', status: 'completed' }
    ]
  }
]

// Computed properties
const filteredWorkflows = computed(() => {
  return workflows.value.filter(workflow => {
    const matchesStatus = !filters.value.status || workflow.status === filters.value.status
    const matchesType = !filters.value.type || workflow.type === filters.value.type
    const matchesSearch = !filters.value.search || 
      workflow.name.toLowerCase().includes(filters.value.search.toLowerCase()) ||
      workflow.description.toLowerCase().includes(filters.value.search.toLowerCase())
    
    return matchesStatus && matchesType && matchesSearch
  })
})

// Methods
const getWorkflowIcon = (type) => {
  const icons = {
    cicd: 'fas fa-code-branch',
    data: 'fas fa-database',
    notification: 'fas fa-bell',
    deployment: 'fas fa-rocket'
  }
  return icons[type] || 'fas fa-cog'
}

const getStatusIcon = (status) => {
  const icons = {
    active: 'fas fa-play-circle',
    inactive: 'fas fa-pause-circle',
    completed: 'fas fa-check-circle',
    failed: 'fas fa-times-circle'
  }
  return icons[status] || 'fas fa-circle'
}

const getStepIcon = (status) => {
  const icons = {
    completed: 'fas fa-check',
    running: 'fas fa-spinner fa-spin',
    pending: 'fas fa-clock',
    failed: 'fas fa-times'
  }
  return icons[status] || 'fas fa-circle'
}

const formatDate = (date) => {
  return new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  }).format(date)
}

const selectWorkflow = (workflow) => {
  console.log('Selected workflow:', workflow)
}

const viewWorkflow = (workflow) => {
  console.log('View workflow:', workflow)
}

const editWorkflow = (workflow) => {
  console.log('Edit workflow:', workflow)
}

const runWorkflow = async (workflow) => {
  try {
    // For now, just simulate execution since we don't have a direct run endpoint
    console.log('Run workflow:', workflow)
    
    // Simulate workflow execution
    workflow.steps.forEach((step, index) => {
      if (step.status === 'pending') {
        setTimeout(() => {
          step.status = 'running'
          setTimeout(() => {
            step.status = 'completed'
          }, 2000)
        }, index * 1000)
      }
    })
    
    // Update workflow status
    await window.apiClient.updateWorkflowStatus(workflow.id, 'Running', 'Workflow started')
    
  } catch (err) {
    error.value = err.message
    console.error('Failed to run workflow:', err)
  }
}

const toggleWorkflow = (workflow) => {
  workflow.status = workflow.status === 'active' ? 'inactive' : 'active'
}

const deleteWorkflow = (workflow) => {
  if (confirm(`Are you sure you want to delete "${workflow.name}"?`)) {
    workflows.value = workflows.value.filter(w => w.id !== workflow.id)
  }
}

const createWorkflow = async () => {
  if (newWorkflow.value.name) {
    try {
      const workflowData = {
        name: newWorkflow.value.name,
        description: newWorkflow.value.description,
        type: newWorkflow.value.type,
        status: newWorkflow.value.status,
        schedule: newWorkflow.value.schedule
      }
      
      const createdWorkflow = await window.apiClient.createWorkflow(workflowData)
      
      // Add to local list
      workflows.value.unshift({
        id: createdWorkflow.workflow_id || Date.now(),
        ...workflowData,
        executions: 0,
        successRate: 0,
        avgDuration: '0s',
        lastRun: null,
        nextRun: workflowData.schedule ? 'Scheduled' : null,
        steps: []
      })
      
      // Reset form
      newWorkflow.value = {
        name: '',
        description: '',
        type: 'cicd',
        status: 'active',
        schedule: ''
      }
      
      showCreateModal.value = false
      
    } catch (err) {
      error.value = err.message
      console.error('Failed to create workflow:', err)
    }
  }
}

const refreshWorkflows = async () => {
  try {
    loading.value = true
    error.value = null

    const workflowsData = await window.apiClient.listWorkflows()
    workflows.value = workflowsData || []

    // If no workflows, keep empty list (no mock data fallback)

  } catch (err) {
    error.value = err.message
    console.error('Failed to load workflows:', err)
    // Clear data on error
    workflows.value = []
  } finally {
    loading.value = false
  }
}

// Lifecycle
onMounted(() => {
  refreshWorkflows()
})
</script>

<style scoped>
.workflows-page {
  max-width: 1400px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: var(--space-6);
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: var(--space-4);
}

.header-text h1 {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin: 0 0 var(--space-2) 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--text-primary);
}

.header-text p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 16px;
}

.header-actions {
  display: flex;
  gap: var(--space-3);
}

.workflows-filters {
  margin-bottom: var(--space-6);
}

.filter-group {
  display: flex;
  gap: var(--space-4);
  align-items: end;
}

.filter-item {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.filter-item label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.workflows-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
  gap: var(--space-4);
}

.workflow-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: var(--transition);
  cursor: pointer;
}

.workflow-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
  border-color: var(--primary);
}

.workflow-header {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.workflow-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  background: var(--primary);
  border-radius: var(--radius-md);
  color: white;
  font-size: 20px;
  flex-shrink: 0;
}

.workflow-info {
  flex: 1;
}

.workflow-name {
  margin: 0 0 var(--space-1) 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.workflow-description {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
  line-height: 1.4;
}

.workflow-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
  flex-shrink: 0;
}

.workflow-status.active {
  background: var(--success-bg);
  color: var(--success);
}

.workflow-status.inactive {
  background: var(--bg-tertiary);
  color: var(--text-muted);
}

.workflow-status.completed {
  background: var(--info-bg);
  color: var(--info);
}

.workflow-status.failed {
  background: var(--error-bg);
  color: var(--error);
}

.workflow-metrics {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.metric {
  text-align: center;
  padding: var(--space-2);
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.metric-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  margin-bottom: var(--space-1);
}

.metric-value {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.workflow-steps {
  margin-bottom: var(--space-4);
}

.steps-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-3);
}

.steps-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.steps-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.steps-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.step-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.step-item.completed {
  background: var(--success-bg);
  border-color: var(--success);
}

.step-item.running {
  background: var(--info-bg);
  border-color: var(--info);
}

.step-item.pending {
  background: var(--bg-secondary);
  border-color: var(--border);
}

.step-item.failed {
  background: var(--error-bg);
  border-color: var(--error);
}

.step-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: var(--radius-sm);
  font-size: 10px;
  flex-shrink: 0;
}

.step-item.completed .step-icon {
  background: var(--success);
  color: white;
}

.step-item.running .step-icon {
  background: var(--info);
  color: white;
}

.step-item.pending .step-icon {
  background: var(--bg-tertiary);
  color: var(--text-muted);
}

.step-item.failed .step-icon {
  background: var(--error);
  color: white;
}

.step-content {
  flex: 1;
}

.step-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
}

.step-description {
  font-size: 11px;
  color: var(--text-secondary);
}

.step-more {
  text-align: center;
  padding: var(--space-2);
  font-size: 12px;
  color: var(--text-muted);
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.workflow-details {
  margin-bottom: var(--space-4);
}

.detail-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: var(--space-1);
}

.detail-item i {
  width: 12px;
  text-align: center;
}

.workflow-actions {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}

.empty-state {
  text-align: center;
  padding: var(--space-8);
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 48px;
  color: var(--text-muted);
  margin-bottom: var(--space-4);
}

.empty-state h3 {
  margin: 0 0 var(--space-2) 0;
  color: var(--text-primary);
}

.empty-state p {
  margin: 0 0 var(--space-4) 0;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  width: 90%;
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-4);
  border-bottom: 1px solid var(--border);
}

.modal-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.modal-content {
  padding: var(--space-4);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-3);
  padding: var(--space-4);
  border-top: 1px solid var(--border);
}

.form-group {
  margin-bottom: var(--space-4);
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-4);
}

.form-group label {
  display: block;
  margin-bottom: var(--space-2);
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.form-input,
.form-select,
.form-textarea {
  width: 100%;
  padding: var(--space-3);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 14px;
  transition: var(--transition);
}

.form-input:focus,
.form-select:focus,
.form-textarea:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 3px var(--primary-bg);
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
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

.btn-success {
  background: var(--success);
  border-color: var(--success);
  color: white;
}

.btn-success:hover {
  background: var(--success-hover);
  border-color: var(--success-hover);
}

.btn-warning {
  background: var(--warning);
  border-color: var(--warning);
  color: white;
}

.btn-warning:hover {
  background: var(--warning-hover);
  border-color: var(--warning-hover);
}

.btn-danger {
  background: var(--error);
  border-color: var(--error);
  color: white;
}

.btn-danger:hover {
  background: var(--error-hover);
  border-color: var(--error-hover);
}
</style>