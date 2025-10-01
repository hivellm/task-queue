<template>
  <div class="projects-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-text">
          <h1><i class="fas fa-folder"></i> Projects</h1>
          <p>Manage your projects and track progress</p>
        </div>
        <div class="header-actions">
          <button class="btn btn-outline" @click="refreshProjects">
            <i class="fas fa-refresh"></i> Refresh
          </button>
          <button class="btn btn-primary" @click="showCreateModal = true">
            <i class="fas fa-plus"></i> New Project
          </button>
        </div>
      </div>
    </div>

    <div class="projects-content">
      <div class="projects-filters">
        <div class="filter-group">
          <div class="filter-item">
            <label>Status:</label>
            <select v-model="filters.status" class="form-select">
              <option value="">All Status</option>
              <option value="active">Active</option>
              <option value="completed">Completed</option>
              <option value="paused">Paused</option>
              <option value="archived">Archived</option>
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
            <label>Search:</label>
            <input 
              v-model="filters.search" 
              type="text" 
              class="form-input" 
              placeholder="Search projects..."
            >
          </div>
        </div>
      </div>

      <div class="projects-grid">
        <div 
          class="project-card" 
          v-for="project in filteredProjects" 
          :key="project.id"
          @click="selectProject(project)"
        >
          <div class="project-header">
            <div class="project-info">
              <h3 class="project-name">{{ project.name }}</h3>
              <p class="project-description">{{ project.description }}</p>
            </div>
            <div class="project-status" :class="project.status">
              <i :class="getStatusIcon(project.status)"></i>
              {{ project.status }}
            </div>
          </div>

          <div class="project-metrics">
            <div class="metric">
              <div class="metric-label">Tasks</div>
              <div class="metric-value">{{ project.taskCount }}</div>
            </div>
            <div class="metric">
              <div class="metric-label">Progress</div>
              <div class="metric-value">{{ project.progress }}%</div>
            </div>
            <div class="metric">
              <div class="metric-label">Priority</div>
              <div class="metric-value priority" :class="project.priority">
                {{ project.priority }}
              </div>
            </div>
          </div>

          <div class="project-progress">
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :style="{ width: project.progress + '%' }"
                :class="getProgressColor(project.progress)"
              ></div>
            </div>
            <div class="progress-text">{{ project.progress }}% Complete</div>
          </div>

          <div class="project-details">
            <div class="detail-item">
              <i class="fas fa-calendar"></i>
              <span>Created: {{ formatDate(project.createdAt) }}</span>
            </div>
            <div class="detail-item">
              <i class="fas fa-user"></i>
              <span>Owner: {{ project.owner }}</span>
            </div>
            <div class="detail-item">
              <i class="fas fa-clock"></i>
              <span>Last Updated: {{ formatDate(project.updatedAt) }}</span>
            </div>
          </div>

          <div class="project-actions">
            <button class="btn btn-sm btn-outline" @click.stop="viewProject(project)">
              <i class="fas fa-eye"></i> View
            </button>
            <button class="btn btn-sm btn-outline" @click.stop="editProject(project)">
              <i class="fas fa-edit"></i> Edit
            </button>
            <button class="btn btn-sm btn-danger" @click.stop="deleteProject(project)">
              <i class="fas fa-trash"></i> Delete
            </button>
          </div>
        </div>
      </div>

      <div v-if="filteredProjects.length === 0" class="empty-state">
        <div class="empty-icon">
          <i class="fas fa-folder-open"></i>
        </div>
        <h3>No projects found</h3>
        <p>Create your first project to get started</p>
        <button class="btn btn-primary" @click="showCreateModal = true">
          <i class="fas fa-plus"></i> Create Project
        </button>
      </div>
    </div>

    <!-- Create Project Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click="showCreateModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h3>Create New Project</h3>
          <button class="btn btn-sm btn-outline" @click="showCreateModal = false">
            <i class="fas fa-times"></i>
          </button>
        </div>
        <div class="modal-content">
          <form @submit.prevent="createProject">
            <div class="form-group">
              <label>Project Name</label>
              <input v-model="newProject.name" type="text" class="form-input" required>
            </div>
            <div class="form-group">
              <label>Description</label>
              <textarea v-model="newProject.description" class="form-textarea" rows="3"></textarea>
            </div>
            <div class="form-group">
              <label>Priority</label>
              <select v-model="newProject.priority" class="form-select">
                <option value="low">Low</option>
                <option value="medium">Medium</option>
                <option value="high">High</option>
                <option value="critical">Critical</option>
              </select>
            </div>
            <div class="form-group">
              <label>Owner</label>
              <input v-model="newProject.owner" type="text" class="form-input" required>
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <button class="btn btn-outline" @click="showCreateModal = false">Cancel</button>
          <button class="btn btn-primary" @click="createProject">Create Project</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

// Reactive data
const projects = ref([])
const showCreateModal = ref(false)
const loading = ref(true)
const error = ref(null)
const filters = ref({
  status: '',
  priority: '',
  search: ''
})

const newProject = ref({
  name: '',
  description: '',
  priority: 'medium',
  owner: ''
})

// Helper functions
const mapProjectStatus = (status) => {
  if (!status) return 'planning'

  const statusMap = {
    'Planning': 'planning',
    'InProgress': 'active',
    'Testing': 'active',
    'Completed': 'completed',
    'Cancelled': 'cancelled',
    'Failed': 'failed'
  }

  return statusMap[status] || status.toLowerCase()
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

// Computed properties
const filteredProjects = computed(() => {
  return projects.value.filter(project => {
    const matchesStatus = !filters.value.status || project.status === filters.value.status
    const matchesPriority = !filters.value.priority || project.priority === filters.value.priority
    const matchesSearch = !filters.value.search || 
      project.name.toLowerCase().includes(filters.value.search.toLowerCase()) ||
      project.description.toLowerCase().includes(filters.value.search.toLowerCase())
    
    return matchesStatus && matchesPriority && matchesSearch
  })
})

// Methods
const getStatusIcon = (status) => {
  const icons = {
    active: 'fas fa-play-circle',
    completed: 'fas fa-check-circle',
    paused: 'fas fa-pause-circle',
    archived: 'fas fa-archive'
  }
  return icons[status] || 'fas fa-circle'
}

const getProgressColor = (progress) => {
  if (progress >= 80) return 'success'
  if (progress >= 50) return 'warning'
  return 'danger'
}

const formatDate = (date) => {
  return new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  }).format(date)
}

const selectProject = (project) => {
  console.log('Selected project:', project)
}

const viewProject = (project) => {
  console.log('View project:', project)
}

const editProject = (project) => {
  console.log('Edit project:', project)
}

const deleteProject = async (project) => {
  if (confirm(`Are you sure you want to delete "${project.name}"?`)) {
    try {
      await window.apiClient.deleteProject(project.id)
      // Remove from local list
      projects.value = projects.value.filter(p => p.id !== project.id)
    } catch (err) {
      error.value = err.message
      console.error('Failed to delete project:', err)
    }
  }
}

const createProject = async () => {
  if (newProject.value.name) {
    try {
      const projectData = {
        name: newProject.value.name,
        description: newProject.value.description
      }

      const createdProject = await window.apiClient.createProject(projectData)

      // Refresh the project list to get updated data
      await refreshProjects()

      // Reset form
      newProject.value = {
        name: '',
        description: '',
        priority: 'medium',
        owner: ''
      }

      showCreateModal.value = false

    } catch (err) {
      error.value = err.message
      console.error('Failed to create project:', err)
    }
  }
}

const refreshProjects = async () => {
  try {
    loading.value = true
    error.value = null

    const projectsData = await window.apiClient.listProjects()

    // Transform projects to match the expected format
    projects.value = (projectsData || []).map(project => ({
      id: project.id,
      name: project.name,
      description: project.description || 'No description',
      status: mapProjectStatus(project.status),
      priority: 'medium', // API doesn't provide priority yet
      taskCount: 0, // Will be calculated when we have task-project relations
      progress: calculateProjectProgress(project.status),
      owner: 'System', // API doesn't provide owner yet
      createdAt: project.created_at ? new Date(project.created_at) : new Date(),
      updatedAt: project.updated_at ? new Date(project.updated_at) : new Date()
    }))

    // Projects API returns dates as ISO strings

  } catch (err) {
    error.value = err.message
    console.error('Failed to load projects:', err)
    // Clear data on error
    projects.value = []
  } finally {
    loading.value = false
  }
}

// Lifecycle
onMounted(() => {
  refreshProjects()
})
</script>

<style scoped>
.projects-page {
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

.projects-filters {
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

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: var(--space-4);
}

.project-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: var(--transition);
  cursor: pointer;
}

.project-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
  border-color: var(--primary);
}

.project-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-4);
}

.project-info {
  flex: 1;
}

.project-name {
  margin: 0 0 var(--space-2) 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.project-description {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
  line-height: 1.4;
}

.project-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
}

.project-status.active {
  background: var(--info-bg);
  color: var(--info);
}

.project-status.completed {
  background: var(--success-bg);
  color: var(--success);
}

.project-status.paused {
  background: var(--warning-bg);
  color: var(--warning);
}

.project-status.archived {
  background: var(--bg-tertiary);
  color: var(--text-muted);
}

.project-metrics {
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

.metric-value.priority {
  text-transform: capitalize;
}

.metric-value.priority.low {
  color: var(--success);
}

.metric-value.priority.medium {
  color: var(--warning);
}

.metric-value.priority.high {
  color: var(--error);
}

.metric-value.priority.critical {
  color: var(--error);
  font-weight: 700;
}

.project-progress {
  margin-bottom: var(--space-4);
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  overflow: hidden;
  margin-bottom: var(--space-2);
}

.progress-fill {
  height: 100%;
  transition: width 0.3s ease;
}

.progress-fill.success {
  background: var(--success);
}

.progress-fill.warning {
  background: var(--warning);
}

.progress-fill.danger {
  background: var(--error);
}

.progress-text {
  font-size: 12px;
  color: var(--text-secondary);
  text-align: center;
}

.project-details {
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

.project-actions {
  display: flex;
  gap: var(--space-2);
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