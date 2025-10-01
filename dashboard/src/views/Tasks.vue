<template>
  <div class="tasks-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-text">
      <h1><i class="fas fa-tasks"></i> Tasks</h1>
          <p>View and manage all tasks across projects</p>
        </div>
        <div class="header-actions">
          <button class="btn btn-outline" @click="refreshTasks">
            <i class="fas fa-refresh"></i> Refresh
          </button>
          <button class="btn btn-primary" @click="showCreateModal = true">
            <i class="fas fa-plus"></i> New Task
          </button>
        </div>
      </div>
    </div>

    <div class="tasks-content">
      <div class="tasks-filters">
        <div class="filter-group">
          <div class="filter-item">
            <label>Status:</label>
            <select v-model="filters.status" class="form-select">
              <option value="">All Status</option>
              <option value="pending">Pending</option>
              <option value="running">Running</option>
              <option value="completed">Completed</option>
              <option value="failed">Failed</option>
              <option value="cancelled">Cancelled</option>
            </select>
          </div>
          <div class="filter-item">
            <label>Priority:</label>
            <select v-model="filters.priority" class="form-select">
              <option value="">All Priorities</option>
              <option value="low">Low</option>
              <option value="medium">Medium</option>
              <option value="high">High</option>
              <option value="critical">Critical</option>
            </select>
          </div>
          <div class="filter-item">
            <label>Project:</label>
            <select v-model="filters.project" class="form-select">
              <option value="">All Projects</option>
              <option v-for="project in projects" :key="project.id" :value="project.name">
                {{ project.name }}
              </option>
            </select>
          </div>
          <div class="filter-item">
            <label>Search:</label>
            <input 
              v-model="filters.search" 
              type="text" 
              class="form-input" 
              placeholder="Search tasks..."
            >
          </div>
        </div>
      </div>

      <div class="tasks-grid">
        <div 
          class="task-card" 
          v-for="task in filteredTasks" 
          :key="task.id"
        >
          <div class="task-header">
            <div class="task-title">
              <h3>{{ task.name }}</h3>
              <span class="task-id">#{{ task.id }}</span>
            </div>
            <div class="task-status" :class="task.status">
              <i :class="getStatusIcon(task.status)"></i>
              {{ task.status }}
            </div>
          </div>

          <div class="task-meta">
            <span class="task-project">
              <i class="fas fa-folder"></i>
              {{ task.project }}
            </span>
            <span class="task-priority" :class="task.priority">
              <i :class="getPriorityIcon(task.priority)"></i>
              {{ task.priority }}
            </span>
            <span class="task-created">
              <i class="fas fa-clock"></i>
              {{ formatDate(task.createdAt) }}
            </span>
          </div>

          <div class="task-description">
            {{ task.description }}
          </div>

          <div class="task-progress" v-if="task.status === 'running'">
            <div class="progress-info">
              <span class="progress-label">Progress</span>
              <span class="progress-percentage">{{ task.progress || 0 }}%</span>
            </div>
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: (task.progress || 0) + '%' }"
              ></div>
            </div>
          </div>

          <div class="task-details">
            <div class="detail-row">
              <div class="detail-item">
                <i class="fas fa-user"></i>
                <span>Assigned to: {{ task.assignee }}</span>
              </div>
              <div class="detail-item">
                <i class="fas fa-calendar"></i>
                <span>Due: {{ formatDate(task.dueDate) }}</span>
              </div>
            </div>
            <div class="detail-row" v-if="task.dependencies && task.dependencies.length > 0">
              <div class="detail-item">
                <i class="fas fa-link"></i>
                <span>Depends on: {{ task.dependencies.length }} task(s)</span>
              </div>
            </div>
          </div>

          <div class="task-actions">
            <button class="btn btn-sm btn-outline" @click="viewTask(task)">
              <i class="fas fa-eye"></i> View
            </button>
            <button class="btn btn-sm btn-outline" @click="editTask(task)">
              <i class="fas fa-edit"></i> Edit
            </button>
            <button 
              class="btn btn-sm btn-primary" 
              @click="startTask(task)"
              v-if="task.status === 'pending'"
            >
              <i class="fas fa-play"></i> Start
            </button>
            <button 
              class="btn btn-sm btn-warning" 
              @click="pauseTask(task)"
              v-if="task.status === 'running'"
            >
              <i class="fas fa-pause"></i> Pause
            </button>
            <button 
              class="btn btn-sm btn-success" 
              @click="completeTask(task)"
              v-if="task.status === 'running'"
            >
              <i class="fas fa-check"></i> Complete
            </button>
            <button 
              class="btn btn-sm btn-danger" 
              @click="cancelTask(task)"
              v-if="['pending', 'running'].includes(task.status)"
            >
              <i class="fas fa-times"></i> Cancel
            </button>
          </div>
        </div>
      </div>

      <div v-if="filteredTasks.length === 0" class="empty-state">
        <div class="empty-icon">
          <i class="fas fa-tasks"></i>
        </div>
        <h3>No tasks found</h3>
        <p>Create your first task or adjust your filters</p>
        <button class="btn btn-primary" @click="showCreateModal = true">
          <i class="fas fa-plus"></i> Create Task
        </button>
      </div>
    </div>

    <!-- Create Task Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click="showCreateModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h3>Create New Task</h3>
          <button class="btn btn-sm btn-outline" @click="showCreateModal = false">
            <i class="fas fa-times"></i>
          </button>
        </div>
        <div class="modal-content">
          <form @submit.prevent="createTask">
            <div class="form-group">
              <label>Task Name</label>
              <input v-model="newTask.name" type="text" class="form-input" required>
            </div>
            <div class="form-group">
              <label>Description</label>
              <textarea v-model="newTask.description" class="form-textarea" rows="3"></textarea>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>Project</label>
                <select v-model="newTask.project" class="form-select" required>
                  <option value="">Select Project</option>
                  <option v-for="project in projects" :key="project.id" :value="project.name">
                    {{ project.name }}
                  </option>
                </select>
              </div>
              <div class="form-group">
                <label>Priority</label>
                <select v-model="newTask.priority" class="form-select">
                  <option value="low">Low</option>
                  <option value="medium">Medium</option>
                  <option value="high">High</option>
                  <option value="critical">Critical</option>
                </select>
              </div>
            </div>
            <div class="form-row">
              <div class="form-group">
                <label>Assignee</label>
                <input v-model="newTask.assignee" type="text" class="form-input" required>
              </div>
              <div class="form-group">
                <label>Due Date</label>
                <input v-model="newTask.dueDate" type="date" class="form-input">
              </div>
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <button class="btn btn-outline" @click="showCreateModal = false">Cancel</button>
          <button class="btn btn-primary" @click="createTask">Create Task</button>
        </div>
    </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

// Reactive data
const tasks = ref([])
const projects = ref([])
const showCreateModal = ref(false)
const loading = ref(true)
const error = ref(null)
const filters = ref({
  status: '',
  priority: '',
  project: '',
  search: ''
})

const newTask = ref({
  name: '',
  description: '',
  project: '',
  priority: 'medium',
  assignee: '',
  dueDate: ''
})

// Helper functions
const mapTaskStatus = (status) => {
  if (!status) return 'pending'

  const statusMap = {
    'Pending': 'pending',
    'Running': 'running',
    'Completed': 'completed',
    'Failed': 'failed',
    'Cancelled': 'cancelled',
    'Planning': 'pending'
  }

  return statusMap[status] || status.toLowerCase()
}

const mapTaskPriority = (priority) => {
  if (!priority) return 'medium'

  const priorityMap = {
    'Low': 'low',
    'Medium': 'medium',
    'High': 'high',
    'Critical': 'critical',
    'Normal': 'medium'
  }

  return priorityMap[priority] || priority.toLowerCase()
}

const parseApiDate = (dateField) => {
  if (!dateField) return null

  // If it's already a string (ISO format), use it directly
  if (typeof dateField === 'string') {
    return new Date(dateField)
  }

  // If it's an object with secs_since_epoch (Rust timestamp format)
  if (dateField && typeof dateField === 'object' && dateField.secs_since_epoch) {
    // Convert seconds to milliseconds
    return new Date(dateField.secs_since_epoch * 1000)
  }

  return null
}

// Computed properties
const filteredTasks = computed(() => {
  return tasks.value.filter(task => {
    const matchesStatus = !filters.value.status || task.status === filters.value.status
    const matchesPriority = !filters.value.priority || task.priority === filters.value.priority
    const matchesProject = !filters.value.project || task.project === filters.value.project
    const matchesSearch = !filters.value.search || 
      task.name.toLowerCase().includes(filters.value.search.toLowerCase()) ||
      task.description.toLowerCase().includes(filters.value.search.toLowerCase()) ||
      task.id.toLowerCase().includes(filters.value.search.toLowerCase())
    
    return matchesStatus && matchesPriority && matchesProject && matchesSearch
  })
})

// Methods
const getStatusIcon = (status) => {
  const icons = {
    pending: 'fas fa-clock',
    running: 'fas fa-play-circle',
    completed: 'fas fa-check-circle',
    failed: 'fas fa-times-circle',
    cancelled: 'fas fa-ban'
  }
  return icons[status] || 'fas fa-circle'
}

const getPriorityIcon = (priority) => {
  const icons = {
    low: 'fas fa-arrow-down',
    medium: 'fas fa-minus',
    high: 'fas fa-exclamation-triangle',
    critical: 'fas fa-exclamation-circle'
  }
  return icons[priority] || 'fas fa-minus'
}

const formatDate = (date) => {
  return new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  }).format(date)
}

const viewTask = (task) => {
  console.log('View task:', task)
}

const editTask = (task) => {
  console.log('Edit task:', task)
}

const startTask = async (task) => {
  try {
    await window.apiClient.setTaskStatus(task.id, 'Running')
    task.status = 'running'
    task.progress = 0
  } catch (err) {
    error.value = err.message
    console.error('Failed to start task:', err)
  }
}

const pauseTask = async (task) => {
  try {
    await window.apiClient.setTaskStatus(task.id, 'Pending')
    task.status = 'pending'
  } catch (err) {
    error.value = err.message
    console.error('Failed to pause task:', err)
  }
}

const completeTask = async (task) => {
  try {
    await window.apiClient.setTaskStatus(task.id, 'Completed')
    task.status = 'completed'
    task.progress = 100
  } catch (err) {
    error.value = err.message
    console.error('Failed to complete task:', err)
  }
}

const cancelTask = async (task) => {
  try {
    await window.apiClient.cancelTask(task.id)
    task.status = 'cancelled'
  } catch (err) {
    error.value = err.message
    console.error('Failed to cancel task:', err)
  }
}

const createTask = async () => {
  if (newTask.value.name && newTask.value.project && newTask.value.assignee) {
    try {
      // Find project by name to get project_id
      const selectedProject = projects.value.find(p => p.name === newTask.value.project)

      const taskData = {
        name: newTask.value.name,
        command: newTask.value.description,
        description: newTask.value.description,
        project_id: selectedProject ? selectedProject.id : null,
        priority: newTask.value.priority.charAt(0).toUpperCase() + newTask.value.priority.slice(1), // Capitalize
        assigned_to: newTask.value.assignee,
        due_date: newTask.value.dueDate,
        status: 'Pending'
      }

      const createdTask = await window.apiClient.createTask(taskData)

      // Refresh the task list to get updated data
      await refreshTasks()

      // Reset form
      newTask.value = {
        name: '',
        description: '',
        project: '',
        priority: 'medium',
        assignee: '',
        dueDate: ''
      }

      showCreateModal.value = false

    } catch (err) {
      error.value = err.message
      console.error('Failed to create task:', err)
    }
  }
}

const refreshTasks = async () => {
  try {
    loading.value = true
    error.value = null

    // Load tasks and projects in parallel
    const [tasksData, projectsData] = await Promise.all([
      window.apiClient.listTasks(),
      window.apiClient.listProjects()
    ])

    // Transform tasks to match the expected format
    tasks.value = (tasksData || []).map(task => ({
      id: task.id,
      name: task.name,
      description: task.description || task.command || 'No description',
      status: mapTaskStatus(task.status),
      priority: mapTaskPriority(task.priority),
      project: task.project_id ? 'Unknown Project' : 'No Project', // Will be updated when we have project relations
      assignee: task.assigned_to || 'Unassigned',
      progress: task.progress || 0,
      createdAt: parseApiDate(task.created_at) || new Date(),
      dueDate: parseApiDate(task.due_date),
      dependencies: task.dependencies || []
    }))

    // Transform projects
    projects.value = (projectsData || []).map(project => ({
      id: project.id,
      name: project.name
    }))

  } catch (err) {
    error.value = err.message
    console.error('Failed to load tasks:', err)
    // Clear data on error
    tasks.value = []
    projects.value = []
  } finally {
    loading.value = false
  }
}

// Lifecycle
onMounted(() => {
  refreshTasks()
})
</script>

<style scoped>
.tasks-page {
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

.tasks-filters {
  margin-bottom: var(--space-6);
}

.filter-group {
  display: flex;
  gap: var(--space-4);
  align-items: end;
  flex-wrap: wrap;
}

.filter-item {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  min-width: 150px;
}

.filter-item label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.tasks-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(450px, 1fr));
  gap: var(--space-4);
}

.task-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: var(--transition);
}

.task-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
  border-color: var(--primary);
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-3);
}

.task-title {
  flex: 1;
}

.task-title h3 {
  margin: 0 0 var(--space-1) 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.task-id {
  font-size: 12px;
  color: var(--text-muted);
  font-family: monospace;
}

.task-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
}

.task-status.pending {
  background: var(--warning-bg);
  color: var(--warning);
}

.task-status.running {
  background: var(--info-bg);
  color: var(--info);
}

.task-status.completed {
  background: var(--success-bg);
  color: var(--success);
}

.task-status.failed {
  background: var(--error-bg);
  color: var(--error);
}

.task-status.cancelled {
  background: var(--bg-tertiary);
  color: var(--text-muted);
}

.task-meta {
  display: flex;
  gap: var(--space-4);
  margin-bottom: var(--space-3);
  font-size: 12px;
  color: var(--text-secondary);
}

.task-project,
.task-priority,
.task-created {
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

.task-priority {
  text-transform: capitalize;
}

.task-priority.low {
  color: var(--success);
}

.task-priority.medium {
  color: var(--warning);
}

.task-priority.high {
  color: var(--error);
}

.task-priority.critical {
  color: var(--error);
  font-weight: 600;
}

.task-description {
  color: var(--text-secondary);
  font-size: 14px;
  line-height: 1.4;
  margin-bottom: var(--space-3);
}

.task-progress {
  margin-bottom: var(--space-3);
}

.progress-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-2);
  font-size: 12px;
}

.progress-label {
  color: var(--text-secondary);
}

.progress-percentage {
  color: var(--text-primary);
  font-weight: 600;
}

.progress-bar {
  width: 100%;
  height: 6px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--primary);
  transition: width 0.3s ease;
}

.task-details {
  margin-bottom: var(--space-4);
}

.detail-row {
  display: flex;
  gap: var(--space-4);
  margin-bottom: var(--space-2);
}

.detail-item {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: 12px;
  color: var(--text-secondary);
  flex: 1;
}

.detail-item i {
  width: 12px;
  text-align: center;
}

.task-actions {
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
  max-width: 600px;
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