<template>
  <div class="api-keys-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-text">
          <h1><i class="fas fa-key"></i> API Keys</h1>
          <p>Manage your API keys and access tokens</p>
        </div>
        <div class="header-actions">
          <button class="btn btn-primary" @click="showCreateModal = true">
            <i class="fas fa-plus"></i> New API Key
          </button>
        </div>
      </div>
    </div>

    <div class="api-keys-content">
      <div class="api-keys-grid">
        <div class="api-key-card" v-for="key in apiKeys" :key="key.id">
          <div class="key-header">
            <div class="key-info">
              <h3>{{ key.name }}</h3>
              <p>{{ key.description }}</p>
            </div>
            <div class="key-status" :class="key.status">
              <i :class="getStatusIcon(key.status)"></i>
              {{ key.status }}
            </div>
          </div>

          <div class="key-details">
            <div class="detail-item">
              <span class="detail-label">Key ID:</span>
              <span class="detail-value">{{ key.keyId }}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Created:</span>
              <span class="detail-value">{{ formatDate(key.createdAt) }}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">Last Used:</span>
              <span class="detail-value">{{ key.lastUsed ? formatDate(key.lastUsed) : 'Never' }}</span>
            </div>
          </div>

          <div class="key-actions">
            <button class="btn btn-sm btn-outline" @click="viewKey(key)">
              <i class="fas fa-eye"></i> View
            </button>
            <button class="btn btn-sm btn-warning" @click="regenerateKey(key)">
              <i class="fas fa-sync"></i> Regenerate
            </button>
            <button class="btn btn-sm btn-danger" @click="deleteKey(key)">
              <i class="fas fa-trash"></i> Delete
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Create API Key Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click="showCreateModal = false">
      <div class="modal" @click.stop>
        <div class="modal-header">
          <h3>Create New API Key</h3>
          <button class="btn btn-sm btn-outline" @click="showCreateModal = false">
            <i class="fas fa-times"></i>
          </button>
        </div>
        <div class="modal-content">
          <form @submit.prevent="createApiKey">
            <div class="form-group">
              <label>Key Name</label>
              <input v-model="newKey.name" type="text" class="form-input" required>
            </div>
            <div class="form-group">
              <label>Description</label>
              <textarea v-model="newKey.description" class="form-textarea" rows="3"></textarea>
            </div>
          </form>
        </div>
        <div class="modal-footer">
          <button class="btn btn-outline" @click="showCreateModal = false">Cancel</button>
          <button class="btn btn-primary" @click="createApiKey">Create Key</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'

const apiKeys = ref([])
const showCreateModal = ref(false)
const newKey = ref({ name: '', description: '' })

const mockApiKeys = [
  {
    id: 1,
    name: 'Production API',
    description: 'Main production API access',
    keyId: 'ak_prod_123456',
    status: 'active',
    createdAt: new Date('2024-01-01'),
    lastUsed: new Date('2024-01-15')
  },
  {
    id: 2,
    name: 'Development API',
    description: 'Development and testing access',
    keyId: 'ak_dev_789012',
    status: 'active',
    createdAt: new Date('2024-01-05'),
    lastUsed: new Date('2024-01-14')
  },
  {
    id: 3,
    name: 'Monitoring API',
    description: 'Monitoring and metrics collection',
    keyId: 'ak_mon_345678',
    status: 'revoked',
    createdAt: new Date('2024-01-10'),
    lastUsed: new Date('2024-01-12')
  }
]

const getStatusIcon = (status) => {
  return status === 'active' ? 'fas fa-check-circle' : 'fas fa-times-circle'
}

const formatDate = (date) => {
  return new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  }).format(date)
}

const createApiKey = () => {
  if (newKey.value.name) {
    const key = {
      id: Date.now(),
      ...newKey.value,
      keyId: `ak_${Math.random().toString(36).substr(2, 9)}`,
      status: 'active',
      createdAt: new Date(),
      lastUsed: null
    }
    apiKeys.value.unshift(key)
    newKey.value = { name: '', description: '' }
    showCreateModal.value = false
  }
}

onMounted(() => {
  apiKeys.value = [...mockApiKeys]
})
</script>

<style scoped>
.api-keys-page {
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

.api-keys-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
  gap: var(--space-4);
}

.api-key-card {
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: var(--space-4);
  transition: var(--transition);
}

.api-key-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-lg);
}

.key-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-4);
}

.key-info h3 {
  margin: 0 0 var(--space-2) 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.key-info p {
  margin: 0;
  color: var(--text-secondary);
  font-size: 14px;
}

.key-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  font-size: 12px;
  font-weight: 500;
  text-transform: uppercase;
}

.key-status.active {
  background: var(--success-bg);
  color: var(--success);
}

.key-status.revoked {
  background: var(--error-bg);
  color: var(--error);
}

.key-details {
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
  font-family: monospace;
}

.key-actions {
  display: flex;
  gap: var(--space-2);
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