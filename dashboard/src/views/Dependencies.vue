<template>
  <div class="dependencies-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-text">
          <h1><i class="fas fa-sitemap"></i> Dependencies</h1>
          <p>Manage task dependencies and relationships</p>
        </div>
        <div class="header-actions">
          <button class="btn btn-outline" @click="refreshDependencies">
            <i class="fas fa-refresh"></i> Refresh
          </button>
          <button class="btn btn-primary" @click="showCreateModal = true">
            <i class="fas fa-plus"></i> New Dependency
          </button>
        </div>
      </div>
    </div>

    <div class="dependencies-content">
      <div class="dependencies-grid">
        <div class="dependency-card" v-for="dependency in dependencies" :key="dependency.id">
          <div class="dependency-header">
            <div class="dependency-info">
              <h3>{{ dependency.name }}</h3>
              <p>{{ dependency.description }}</p>
            </div>
            <div class="dependency-status" :class="dependency.status">
              <i :class="getStatusIcon(dependency.status)"></i>
              {{ dependency.status }}
            </div>
          </div>

          <div class="dependency-details">
            <div class="detail-item">
              <span class="detail-label">Depends on:</span>
              <span class="detail-value">{{ dependency.dependsOn }}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Blocks:</span>
              <span class="detail-value">{{ dependency.blocks }}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Priority:</span>
              <span class="detail-value priority" :class="dependency.priority">{{ dependency.priority }}</span>
            </div>
          </div>

          <div class="dependency-actions">
            <button class="btn btn-sm btn-outline" @click="viewDependency(dependency)">
              <i class="fas fa-eye"></i> View
            </button>
            <button class="btn btn-sm btn-outline" @click="editDependency(dependency)">
              <i class="fas fa-edit"></i> Edit
            </button>
            <button class="btn btn-sm btn-danger" @click="deleteDependency(dependency)">
              <i class="fas fa-trash"></i> Delete
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
              <div class="graph-node completed" v-for="node in dependencyGraph" :key="node.id">
                <div class="node-icon">
                  <i :class="node.icon"></i>
                </div>
                <div class="node-label">{{ node.label }}</div>
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

const dependencies = ref([])
const showCreateModal = ref(false)

const dependencyGraph = ref([
  { id: 1, label: 'Schema Design', icon: 'fas fa-database', status: 'completed' },
  { id: 2, label: 'Database Migration', icon: 'fas fa-sync', status: 'completed' },
  { id: 3, label: 'API Development', icon: 'fas fa-code', status: 'running' },
  { id: 4, label: 'Frontend Integration', icon: 'fas fa-palette', status: 'pending' },
  { id: 5, label: 'Testing', icon: 'fas fa-vial', status: 'pending' },
  { id: 6, label: 'Deployment', icon: 'fas fa-rocket', status: 'pending' }
])

const mockDependencies = [
  {
    id: 1,
    name: 'Database Migration',
    description: 'Core database schema updates',
    status: 'completed',
    dependsOn: 'Schema Design',
    blocks: 'API Development',
    priority: 'critical'
  },
  {
    id: 2,
    name: 'API Development',
    description: 'REST API implementation',
    status: 'running',
    dependsOn: 'Database Migration',
    blocks: 'Frontend Integration',
    priority: 'high'
  },
  {
    id: 3,
    name: 'Frontend Integration',
    description: 'UI components and integration',
    status: 'pending',
    dependsOn: 'API Development',
    blocks: 'Testing',
    priority: 'medium'
  }
]

const getStatusIcon = (status) => {
  const icons = {
    completed: 'fas fa-check-circle',
    running: 'fas fa-play-circle',
    pending: 'fas fa-clock',
    failed: 'fas fa-times-circle'
  }
  return icons[status] || 'fas fa-circle'
}

const refreshDependencies = () => {
  dependencies.value = [...mockDependencies]
}

onMounted(() => {
  refreshDependencies()
})
</script>

<style scoped>
.dependencies-page {
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

.dependencies-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.dependency-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: var(--transition);
}

.dependency-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.dependency-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-4);
}

.dependency-info h3 {
  margin: 0 0 var(--space-2) 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.dependency-info p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.dependency-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
}

.dependency-status.completed {
  background: var(--success-bg);
  color: var(--success);
}

.dependency-status.running {
  background: var(--info-bg);
  color: var(--info);
}

.dependency-status.pending {
  background: var(--warning-bg);
  color: var(--warning);
}

.dependency-details {
  margin-bottom: var(--space-4);
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-2);
  font-size: 14px;
}

.detail-label {
  color: var(--text-secondary);
  font-weight: 500;
}

.detail-value {
  color: var(--text-primary);
}

.detail-value.priority {
  text-transform: capitalize;
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
}

.detail-value.priority.critical {
  background: var(--error-bg);
  color: var(--error);
}

.detail-value.priority.high {
  background: var(--error-bg);
  color: var(--error);
}

.detail-value.priority.medium {
  background: var(--warning-bg);
  color: var(--warning);
}

.detail-value.priority.low {
  background: var(--success-bg);
  color: var(--success);
}

.dependency-actions {
  display: flex;
  gap: var(--space-2);
}

.dependencies-visualization {
  margin-top: var(--space-6);
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
  padding: var(--space-6);
}

.dependency-graph {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-4);
  justify-content: center;
  align-items: center;
}

.graph-node {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-3);
  background: var(--bg-secondary);
  border: 2px solid var(--border);
  border-radius: var(--radius-md);
  transition: var(--transition);
}

.graph-node.completed {
  border-color: var(--success);
  background: var(--success-bg);
}

.graph-node.running {
  border-color: var(--info);
  background: var(--info-bg);
}

.graph-node.pending {
  border-color: var(--warning);
  background: var(--warning-bg);
}

.node-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: var(--primary);
  border-radius: var(--radius-sm);
  color: white;
  font-size: 14px;
}

.node-label {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
  text-align: center;
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