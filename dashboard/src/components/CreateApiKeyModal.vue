<template>
  <div v-if="apiKeysStore.showCreateModal" class="modal-overlay" @click.self="apiKeysStore.closeCreateModal">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2><i class="fas fa-key"></i> Create New API Key</h2>
        <button class="btn btn-icon" @click="apiKeysStore.closeCreateModal">
          <i class="fas fa-times"></i>
        </button>
      </div>
      <div class="modal-body">
        <form @submit.prevent="handleSubmit">
          <div class="form-group">
            <label for="key-name">Key Name</label>
            <input 
              type="text" 
              id="key-name" 
              v-model="apiKeysStore.newApiKey.name" 
              class="form-control" 
              placeholder="e.g., MCP Development Key" 
              required
            >
          </div>
          <div class="form-group">
            <label for="key-type">Key Type</label>
            <select 
              id="key-type" 
              v-model="apiKeysStore.newApiKey.type" 
              class="form-control" 
              required
            >
              <option value="mcp">MCP (Model Context Protocol)</option>
              <option value="rest">REST API</option>
              <option value="both">Both MCP & REST</option>
            </select>
          </div>
          <div class="form-group">
            <label for="key-description">Description (Optional)</label>
            <textarea 
              id="key-description" 
              v-model="apiKeysStore.newApiKey.description" 
              class="form-control" 
              placeholder="Describe what this key will be used for..."
              rows="3"
            ></textarea>
          </div>
          <div class="form-group">
            <label for="key-expiry">Expiry Date (Optional)</label>
            <input 
              type="date" 
              id="key-expiry" 
              v-model="apiKeysStore.newApiKey.expiryDate" 
              class="form-control"
            >
          </div>
          <div class="form-group">
            <label class="checkbox-label">
              <input 
                type="checkbox" 
                v-model="apiKeysStore.newApiKey.showKeyImmediately"
              >
              <span class="checkmark"></span>
              Show key immediately after creation
            </label>
          </div>
        </form>
      </div>
      <div class="modal-footer">
        <button class="btn btn-outline" @click="apiKeysStore.closeCreateModal">
          Cancel
        </button>
        <button 
          class="btn btn-primary" 
          @click="handleSubmit" 
          :disabled="apiKeysStore.creating || !apiKeysStore.newApiKey.name.trim()"
        >
          <i v-if="apiKeysStore.creating" class="fas fa-spinner fa-spin"></i>
          <i v-else class="fas fa-plus"></i>
          {{ apiKeysStore.creating ? 'Creating...' : 'Create Key' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import { useApiKeysStore } from '../stores/apiKeys'

export default {
  name: 'CreateApiKeyModal',
  setup() {
    const apiKeysStore = useApiKeysStore()
    
    const handleSubmit = async () => {
      if (!apiKeysStore.newApiKey.name.trim() || apiKeysStore.creating) return
      
      const result = await apiKeysStore.createApiKey()
      if (result.success) {
        showToast('API key created successfully', 'success')
      } else {
        showToast(result.error || 'Failed to create API key', 'error')
      }
    }
    
    const showToast = (message, type = 'info') => {
      // Simple toast implementation - you can replace with a proper toast library
      console.log(`Toast [${type}]: ${message}`)
    }
    
    return {
      apiKeysStore,
      handleSubmit
    }
  }
}
</script>

<style scoped>
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

.modal-content {
  background: var(--bg-card);
  border-radius: var(--radius-lg);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  max-width: 500px;
  width: 90%;
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

.modal-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.modal-body {
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
  font-weight: 500;
  color: var(--text-primary);
}

.form-control {
  width: 100%;
  padding: var(--space-3);
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 14px;
  transition: var(--transition);
}

.form-control:focus {
  border-color: var(--primary);
  outline: none;
  box-shadow: 0 0 0 2px rgba(var(--primary-rgb), 0.2);
}

.form-control::placeholder {
  color: var(--text-muted);
}

textarea.form-control {
  min-height: 80px;
  resize: vertical;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  cursor: pointer;
  font-size: 14px;
  color: var(--text-primary);
}

.checkbox-label input[type="checkbox"] {
  display: none;
}

.checkmark {
  width: 18px;
  height: 18px;
  border: 2px solid var(--border);
  border-radius: var(--radius-sm);
  position: relative;
  transition: var(--transition);
}

.checkbox-label input[type="checkbox"]:checked + .checkmark {
  background: var(--primary);
  border-color: var(--primary);
}

.checkbox-label input[type="checkbox"]:checked + .checkmark::after {
  content: '';
  position: absolute;
  left: 5px;
  top: 2px;
  width: 4px;
  height: 8px;
  border: solid white;
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}

.btn {
  padding: var(--space-2) var(--space-4);
  border: none;
  border-radius: var(--radius-md);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition);
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--primary);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-dark);
}

.btn-outline {
  background: transparent;
  color: var(--text-primary);
  border: 1px solid var(--border);
}

.btn-outline:hover:not(:disabled) {
  background: var(--bg-tertiary);
}

.btn-icon {
  padding: var(--space-2);
  background: transparent;
  color: var(--text-muted);
  border: none;
  border-radius: var(--radius-md);
}

.btn-icon:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}
</style>
