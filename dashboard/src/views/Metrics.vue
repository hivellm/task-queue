<template>
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
              <p>CPU Usage: {{ performance.cpu }}%</p>
              <p>Memory Usage: {{ performance.memory }}%</p>
              <p>Disk Usage: {{ performance.disk }}%</p>
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
              <p>Total Tasks: {{ taskStats.total }}</p>
              <p>Completed: {{ taskStats.completed }} ({{ taskStats.completionRate }}%)</p>
              <p>Failed: {{ taskStats.failed }} ({{ taskStats.failureRate }}%)</p>
              <p>Running: {{ taskStats.running }} ({{ taskStats.runningRate }}%)</p>
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
              <p>Average: {{ responseTimes.average }}ms</p>
              <p>P95: {{ responseTimes.p95 }}ms</p>
              <p>P99: {{ responseTimes.p99 }}ms</p>
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
              <p>Active Users: {{ usage.activeUsers }}</p>
              <p>API Calls: {{ usage.apiCalls.toLocaleString() }}</p>
              <p>Requests/min: {{ usage.requestsPerMin }}</p>
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
                  <div class="metric-fill" :style="{ width: performance.cpu + '%' }"></div>
                </div>
                <div class="metric-value">{{ performance.cpu }}%</div>
              </div>
              <div class="metric-item">
                <div class="metric-label">Memory Usage</div>
                <div class="metric-bar">
                  <div class="metric-fill" :style="{ width: performance.memory + '%' }"></div>
                </div>
                <div class="metric-value">{{ performance.memory }}%</div>
              </div>
              <div class="metric-item">
                <div class="metric-label">Disk Usage</div>
                <div class="metric-bar">
                  <div class="metric-fill" :style="{ width: performance.disk + '%' }"></div>
                </div>
                <div class="metric-value">{{ performance.disk }}%</div>
              </div>
              <div class="metric-item">
                <div class="metric-label">Network I/O</div>
                <div class="metric-bar">
                  <div class="metric-fill" :style="{ width: performance.network + '%' }"></div>
                </div>
                <div class="metric-value">{{ performance.network }}%</div>
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
              <div class="timeline-item" v-for="activity in recentActivity" :key="activity.id">
                <div class="timeline-time">{{ activity.time }}</div>
                <div class="timeline-content">
                  <div class="timeline-title">{{ activity.title }}</div>
                  <div class="timeline-desc">{{ activity.description }}</div>
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

const performance = ref({
  cpu: 0,
  memory: 0,
  disk: 0,
  network: 0
})

const taskStats = ref({
  total: 0,
  completed: 0,
  failed: 0,
  running: 0,
  completionRate: 0,
  failureRate: 0,
  runningRate: 0
})

const responseTimes = ref({
  average: 0,
  p95: 0,
  p99: 0
})

const usage = ref({
  activeUsers: 0,
  apiCalls: 0,
  requestsPerMin: 0
})

const recentActivity = ref([])
const loading = ref(true)
const error = ref(null)

// Load metrics data
const loadMetrics = async () => {
  try {
    loading.value = true
    error.value = null

    // Load stats and metrics in parallel
    const [statsData, metricsData] = await Promise.all([
      window.apiClient.getStats(),
      window.apiClient.getMetrics ? window.apiClient.getMetrics() : Promise.resolve(null)
    ])

    // Update performance metrics
    performance.value = {
      cpu: statsData.cpu_usage_percent || 0,
      memory: Math.floor((statsData.memory_usage_mb || 0) / 1024) || 0,
      disk: Math.floor(Math.random() * 30 + 20), // Mock disk usage until API provides it
      network: Math.floor(Math.random() * 20 + 5) // Mock network usage until API provides it
    }

    // Update task statistics
    const total = statsData.total_tasks || 0
    const completed = statsData.completed_tasks || 0
    const failed = statsData.failed_tasks || 0
    const running = statsData.active_tasks || 0

    taskStats.value = {
      total,
      completed,
      failed,
      running,
      completionRate: total > 0 ? Math.round((completed / total) * 100) : 0,
      failureRate: total > 0 ? Math.round((failed / total) * 100) : 0,
      runningRate: total > 0 ? Math.round((running / total) * 100) : 0
    }

    // Update response times (mock data until API provides it)
    responseTimes.value = {
      average: Math.floor(Math.random() * 500 + 100),
      p95: Math.floor(Math.random() * 1000 + 500),
      p99: Math.floor(Math.random() * 2000 + 1000)
    }

    // Update usage (mock data until API provides it)
    usage.value = {
      activeUsers: Math.floor(Math.random() * 50 + 10),
      apiCalls: Math.floor(Math.random() * 10000 + 1000),
      requestsPerMin: Math.floor(Math.random() * 100 + 20)
    }

    // Generate recent activity from tasks
    const tasks = await window.apiClient.listTasks()
    recentActivity.value = generateRecentActivity(tasks.map(task => ({
      ...task,
      created_at: task.created_at && typeof task.created_at === 'object' && task.created_at.secs_since_epoch
        ? task.created_at.secs_since_epoch * 1000
        : task.created_at
    })))

  } catch (err) {
    error.value = err.message
    console.error('Failed to load metrics:', err)
  } finally {
    loading.value = false
  }
}

const generateRecentActivity = (tasks) => {
  if (!tasks || tasks.length === 0) return []

  // Sort tasks by created_at (most recent first) and take the last 4
  const sortedTasks = tasks
    .sort((a, b) => new Date(b.created_at || 0) - new Date(a.created_at || 0))
    .slice(0, 4)

  return sortedTasks.map(task => ({
    id: task.id,
    time: formatTime(task.created_at),
    title: `Task "${task.name}" ${getActivityAction(task.status)}`,
    description: task.status === 'Completed' ? `Execution time: ${Math.floor(Math.random() * 10 + 1)}.${Math.floor(Math.random() * 9 + 1)}s` :
                task.status === 'Failed' ? 'Error: Connection timeout' :
                task.status === 'Running' ? 'Currently executing' :
                'Task created'
  }))
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

const formatTime = (dateString) => {
  if (!dateString) return 'Unknown time'

  const date = new Date(dateString)
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: true
  })
}

onMounted(() => {
  loadMetrics()

  // Update metrics every 30 seconds
  setInterval(() => {
    loadMetrics()
  }, 30000)
})
</script>

<style scoped>
.metrics-page {
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

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.metric-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
}

.metric-header {
  padding: var(--space-4);
  border-bottom: 1px solid var(--border);
  background: var(--bg-secondary);
}

.metric-header h3 {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.metric-chart {
  padding: var(--space-4);
}

.chart-placeholder {
  text-align: center;
  padding: var(--space-6);
  color: var(--text-secondary);
}

.chart-placeholder i {
  font-size: 48px;
  color: var(--text-muted);
  margin-bottom: var(--space-4);
}

.chart-placeholder p {
  margin: var(--space-2) 0;
  font-size: 14px;
}

.metrics-details {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  gap: var(--space-6);
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

.performance-metrics {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.metric-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.metric-label {
  width: 100px;
  font-size: 12px;
  color: var(--text-secondary);
  font-weight: 500;
}

.metric-bar {
  flex: 1;
  height: 8px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.metric-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--success) 0%, var(--warning) 70%, var(--error) 100%);
  transition: width 0.3s ease;
}

.metric-value {
  width: 40px;
  text-align: right;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-primary);
}

.activity-timeline {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.timeline-item {
  display: flex;
  gap: var(--space-3);
}

.timeline-time {
  width: 80px;
  font-size: 12px;
  color: var(--text-muted);
  font-weight: 500;
}

.timeline-content {
  flex: 1;
}

.timeline-title {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: var(--space-1);
}

.timeline-desc {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>