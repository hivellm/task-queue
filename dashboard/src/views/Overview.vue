<template>
  <div class="overview-page">
    <div class="page-header">
      <h1><i class="fas fa-tachometer-alt"></i> Overview</h1>
      <p>System overview and quick stats</p>
    </div>
    
    <div class="overview-content">
      <div v-if="error" class="error-message">
        <i class="fas fa-exclamation-triangle"></i>
        <span>{{ error }}</span>
        <button class="btn btn-sm btn-outline" @click="refreshData">
          <i class="fas fa-refresh"></i> Retry
        </button>
      </div>
      
      <div v-if="loading" class="loading-message">
        <i class="fas fa-spinner fa-spin"></i>
        <span>Loading dashboard data...</span>
      </div>

      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon">
            <i class="fas fa-tasks"></i>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ totalTasks }}</div>
            <div class="stat-label">Total Tasks</div>
            <div class="stat-change positive">+12% from last week</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon completed">
            <i class="fas fa-check-circle"></i>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ completedTasks }}</div>
            <div class="stat-label">Completed</div>
            <div class="stat-change positive">67% completion rate</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon running">
            <i class="fas fa-play-circle"></i>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ runningTasks }}</div>
            <div class="stat-label">Running</div>
            <div class="stat-change neutral">21% of total</div>
          </div>
        </div>
        
        <div class="stat-card">
          <div class="stat-icon failed">
            <i class="fas fa-exclamation-triangle"></i>
          </div>
          <div class="stat-content">
            <div class="stat-value">{{ failedTasks }}</div>
            <div class="stat-label">Failed</div>
            <div class="stat-change negative">12% failure rate</div>
          </div>
        </div>
      </div>
      
      <div class="overview-grid">
        <div class="overview-card">
          <div class="card-header">
            <h3><i class="fas fa-chart-line"></i> Recent Activity</h3>
            <button class="btn btn-sm btn-outline" @click="refreshData">
              <i class="fas fa-refresh"></i> Refresh
            </button>
          </div>
          <div class="card-content">
            <div class="activity-list">
              <div class="activity-item" v-for="activity in recentActivity" :key="activity.id">
                <div class="activity-icon" :class="activity.type">
                  <i :class="activity.icon"></i>
                </div>
                <div class="activity-content">
                  <div class="activity-title">{{ activity.title }}</div>
                  <div class="activity-description">{{ activity.description }}</div>
                  <div class="activity-time">{{ activity.time }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="overview-card">
          <div class="card-header">
            <h3><i class="fas fa-project-diagram"></i> Active Projects</h3>
            <button class="btn btn-sm btn-primary">
              <i class="fas fa-plus"></i> New Project
            </button>
          </div>
          <div class="card-content">
            <div class="project-list">
              <div class="project-item" v-for="project in activeProjects" :key="project.id">
                <div class="project-info">
                  <div class="project-name">{{ project.name }}</div>
                  <div class="project-description">{{ project.description }}</div>
                </div>
                <div class="project-stats">
                  <div class="project-metrics">
                    <span class="project-tasks">{{ project.taskCount }} tasks</span>
                    <div class="progress-bar">
                      <div class="progress-fill" :style="{ width: project.progress + '%' }"></div>
                    </div>
                  </div>
                  <span class="project-status" :class="project.status">{{ project.status }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="overview-grid">
        <div class="overview-card">
          <div class="card-header">
            <h3><i class="fas fa-clock"></i> Task Queue Status</h3>
          </div>
          <div class="card-content">
            <div class="queue-stats">
              <div class="queue-item">
                <div class="queue-label">Queue Size</div>
                <div class="queue-value">{{ queueSize }}</div>
              </div>
              <div class="queue-item">
                <div class="queue-label">Processing Rate</div>
                <div class="queue-value">{{ processingRate }}/min</div>
              </div>
              <div class="queue-item">
                <div class="queue-label">Average Wait Time</div>
                <div class="queue-value">{{ averageWaitTime }}</div>
              </div>
              <div class="queue-item">
                <div class="queue-label">Worker Status</div>
                <div class="queue-value status-online">
                  <i class="fas fa-circle"></i>
                  {{ activeWorkers }} active
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="overview-card">
          <div class="card-header">
            <h3><i class="fas fa-server"></i> System Health</h3>
          </div>
          <div class="card-content">
            <div class="health-metrics">
              <div class="health-item">
                <div class="health-label">CPU Usage</div>
                <div class="health-bar">
                  <div class="health-fill" :style="{ width: systemHealth.cpu + '%' }"></div>
                </div>
                <div class="health-value">{{ systemHealth.cpu }}%</div>
              </div>
              
              <div class="health-item">
                <div class="health-label">Memory Usage</div>
                <div class="health-bar">
                  <div class="health-fill" :style="{ width: systemHealth.memory + '%' }"></div>
                </div>
                <div class="health-value">{{ systemHealth.memory }}%</div>
              </div>
              
              <div class="health-item">
                <div class="health-label">Disk Usage</div>
                <div class="health-bar">
                  <div class="health-fill" :style="{ width: systemHealth.disk + '%' }"></div>
                </div>
                <div class="health-value">{{ systemHealth.disk }}%</div>
              </div>
              
              <div class="health-item">
                <div class="health-label">Network I/O</div>
                <div class="health-bar">
                  <div class="health-fill" :style="{ width: systemHealth.network + '%' }"></div>
                </div>
                <div class="health-value">{{ systemHealth.network }}%</div>
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

// Reactive data
const totalTasks = ref(0)
const completedTasks = ref(0)
const runningTasks = ref(0)
const failedTasks = ref(0)

const queueSize = ref(0)
const processingRate = ref(0)
const averageWaitTime = ref('0s')
const activeWorkers = ref(0)

const systemHealth = ref({
  cpu: 0,
  memory: 0,
  disk: 0,
  network: 0
})

const loading = ref(true)
const error = ref(null)

const recentActivity = ref([])
const activeProjects = ref([])

// Methods
const loadStats = async () => {
  try {
    loading.value = true
    error.value = null

    // Load stats
    const stats = await window.apiClient.getStats()

    totalTasks.value = stats.total_tasks || 0
    completedTasks.value = stats.completed_tasks || 0
    runningTasks.value = stats.active_tasks || 0
    failedTasks.value = stats.failed_tasks || 0

    queueSize.value = stats.pending_tasks || 0
    processingRate.value = Math.floor((runningTasks.value * 60) / 100) || 0
    averageWaitTime.value = queueSize.value > 0 ? `${Math.floor(queueSize.value * 0.3)}s` : '0s'
    activeWorkers.value = runningTasks.value || 0

    systemHealth.value = {
      cpu: stats.cpu_usage_percent || 0,
      memory: Math.floor((stats.memory_usage_mb || 0) / 1024) || 0,
      disk: Math.floor(Math.random() * 30 + 20), // Mock disk usage until API provides it
      network: Math.floor(Math.random() * 20 + 5) // Mock network usage until API provides it
    }

    // Load projects
    const projects = await window.apiClient.listProjects()
    activeProjects.value = projects.map(project => ({
      id: project.id,
      name: project.name,
      description: project.description || 'No description',
      taskCount: 0, // Will be calculated when we have task-project relations
      progress: calculateProjectProgress(project.status),
      status: mapProjectStatus(project.status)
    }))

    // Load recent tasks for activity
    const tasks = await window.apiClient.listTasks()
    recentActivity.value = generateRecentActivity(tasks.map(task => ({
      ...task,
      created_at: task.created_at && typeof task.created_at === 'object' && task.created_at.secs_since_epoch
        ? task.created_at.secs_since_epoch * 1000
        : task.created_at
    })))

  } catch (err) {
    error.value = err.message
    console.error('Failed to load stats:', err)
  } finally {
    loading.value = false
  }
}

const calculateProjectProgress = (status) => {
  const progressMap = {
    'Planning': 0,
    'InProgress': 50,
    'Testing': 75,
    'Completed': 100,
    'Cancelled': 0,
    'Failed': 25
  }
  return progressMap[status] || 0
}

const mapProjectStatus = (status) => {
  const statusMap = {
    'Planning': 'pending',
    'InProgress': 'running',
    'Testing': 'running',
    'Completed': 'completed',
    'Cancelled': 'cancelled',
    'Failed': 'failed'
  }
  return statusMap[status] || 'pending'
}

const generateRecentActivity = (tasks) => {
  if (!tasks || tasks.length === 0) return []

  // Sort tasks by created_at (most recent first) and take the last 5
  const sortedTasks = tasks
    .sort((a, b) => new Date(b.created_at || 0) - new Date(a.created_at || 0))
    .slice(0, 5)

  return sortedTasks.map(task => ({
    id: task.id,
    type: getActivityType(task.status),
    icon: getActivityIcon(task.status),
    title: `Task "${task.name}" ${getActivityAction(task.status)}`,
    description: task.description || 'No description',
    time: formatRelativeTime(task.created_at)
  }))
}

const getActivityType = (status) => {
  const typeMap = {
    'Completed': 'success',
    'Running': 'info',
    'Failed': 'error',
    'Cancelled': 'warning',
    'Pending': 'info'
  }
  return typeMap[status] || 'info'
}

const getActivityIcon = (status) => {
  const iconMap = {
    'Completed': 'fas fa-check',
    'Running': 'fas fa-play',
    'Failed': 'fas fa-times',
    'Cancelled': 'fas fa-ban',
    'Pending': 'fas fa-clock'
  }
  return iconMap[status] || 'fas fa-circle'
}

const getActivityAction = (status) => {
  const actionMap = {
    'Completed': 'completed',
    'Running': 'started',
    'Failed': 'failed',
    'Cancelled': 'cancelled',
    'Pending': 'created'
  }
  return actionMap[status] || 'updated'
}

const formatRelativeTime = (dateString) => {
  if (!dateString) return 'Unknown time'

  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now - date
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  const diffDays = Math.floor(diffHours / 24)

  if (diffDays > 0) {
    return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`
  } else if (diffHours > 0) {
    return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`
  } else {
    const diffMinutes = Math.floor(diffMs / (1000 * 60))
    return diffMinutes > 0 ? `${diffMinutes} minute${diffMinutes > 1 ? 's' : ''} ago` : 'Just now'
  }
}

const refreshData = () => {
  loadStats()
}

// Lifecycle
onMounted(() => {
  loadStats()
  
  // Update data every 30 seconds
  setInterval(() => {
    loadStats()
  }, 30000)
})
</script>

<style scoped>
.overview-page {
  max-width: 1200px;
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

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.stat-card {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-4);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  transition: var(--transition);
}

.stat-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.stat-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 48px;
  height: 48px;
  background: var(--primary);
  border-radius: var(--radius-md);
  color: white;
  font-size: 20px;
}

.stat-icon.completed {
  background: var(--success);
}

.stat-icon.running {
  background: var(--info);
}

.stat-icon.failed {
  background: var(--error);
}

.stat-content {
  flex: 1;
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
}

.stat-label {
  color: var(--text-secondary);
  font-size: 14px;
  font-weight: 500;
  margin-bottom: var(--space-1);
}

.stat-change {
  font-size: 12px;
  font-weight: 500;
}

.stat-change.positive {
  color: var(--success);
}

.stat-change.negative {
  color: var(--error);
}

.stat-change.neutral {
  color: var(--text-muted);
}

.overview-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: var(--space-6);
  margin-bottom: var(--space-6);
}

.overview-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
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

.activity-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.activity-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
}

.activity-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-sm);
  font-size: 12px;
  flex-shrink: 0;
}

.activity-icon.success {
  background: var(--success-bg);
  color: var(--success);
}

.activity-icon.info {
  background: var(--info-bg);
  color: var(--info);
}

.activity-icon.error {
  background: var(--error-bg);
  color: var(--error);
}

.activity-icon.warning {
  background: var(--warning-bg);
  color: var(--warning);
}

.activity-content {
  flex: 1;
}

.activity-title {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
}

.activity-description {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: var(--space-1);
}

.activity-time {
  font-size: 11px;
  color: var(--text-muted);
}

.project-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.project-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-3);
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.project-info {
  flex: 1;
}

.project-name {
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
}

.project-description {
  font-size: 12px;
  color: var(--text-secondary);
}

.project-stats {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: var(--space-2);
}

.project-metrics {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: var(--space-1);
}

.project-tasks {
  font-size: 12px;
  color: var(--text-secondary);
}

.progress-bar {
  width: 80px;
  height: 4px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--primary);
  transition: width 0.3s ease;
}

.project-status {
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  font-weight: 500;
  font-size: 11px;
  text-transform: uppercase;
}

.project-status.running {
  background: var(--info-bg);
  color: var(--info);
}

.project-status.completed {
  background: var(--success-bg);
  color: var(--success);
}

.project-status.failed {
  background: var(--error-bg);
  color: var(--error);
}

.queue-stats {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: var(--space-4);
}

.queue-item {
  text-align: center;
  padding: var(--space-3);
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border);
}

.queue-label {
  font-size: 12px;
  color: var(--text-secondary);
  margin-bottom: var(--space-2);
  font-weight: 500;
}

.queue-value {
  font-size: 20px;
  font-weight: 700;
  color: var(--text-primary);
}

.queue-value.status-online {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  color: var(--success);
  font-size: 14px;
}

.queue-value.status-online i {
  font-size: 8px;
}

.health-metrics {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.health-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.health-label {
  width: 100px;
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.health-bar {
  flex: 1;
  height: 8px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.health-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--success) 0%, var(--warning) 70%, var(--error) 100%);
  transition: width 0.3s ease;
}

.health-value {
  width: 40px;
  text-align: right;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
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
  font-size: 12px;
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
  font-size: 11px;
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

.error-message {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-4);
  background: var(--error-bg);
  border: 1px solid var(--error);
  border-radius: var(--radius-md);
  color: var(--error);
  margin-bottom: var(--space-4);
}

.error-message i {
  font-size: 18px;
}

.loading-message {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-4);
  background: var(--info-bg);
  border: 1px solid var(--info);
  border-radius: var(--radius-md);
  color: var(--info);
  margin-bottom: var(--space-4);
}

.loading-message i {
  font-size: 18px;
}
</style>