export default {
  name: 'ApiKeys',
  setup() {
    // Mock data for now
    const Vue = window.Vue;
    const { ref } = Vue;

    const apiKeys = ref([
      {
        id: '1',
        name: 'MCP Development Key',
        type: 'mcp',
        key: 'mcp_1234567890abcdef1234567890abcdef',
        status: 'active',
        createdAt: new Date().toISOString(),
        lastUsed: new Date(Date.now() - 86400000).toISOString(),
        showKey: false,
        description: 'Development key for MCP integration'
      },
      {
        id: '2',
        name: 'REST API Key',
        type: 'rest',
        key: 'rest_abcdef1234567890abcdef1234567890',
        status: 'active',
        createdAt: new Date(Date.now() - 172800000).toISOString(),
        lastUsed: null,
        showKey: false,
        description: 'REST API access key'
      }
    ]);

    const loading = ref(false);

    const copyKey = async (key) => {
      try {
        await navigator.clipboard.writeText(key);
        console.log('API key copied to clipboard');
      } catch (error) {
        console.error('Failed to copy API key:', error);
      }
    };

    const toggleVisibility = (keyId) => {
      const key = apiKeys.value.find(k => k.id === keyId);
      if (key) {
        key.showKey = !key.showKey;
      }
    };

    const deleteKey = async (keyId) => {
      if (!confirm('Are you sure you want to delete this API key?')) return;

      try {
        apiKeys.value = apiKeys.value.filter(k => k.id !== keyId);
        console.log('API key deleted successfully');
      } catch (error) {
        console.error('Failed to delete API key:', error);
      }
    };

    const formatDate = (dateString) => {
      if (!dateString) return 'N/A';
      try {
        const date = new Date(dateString);
        return date.toLocaleDateString('pt-BR', {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit'
        });
      } catch (error) {
        return 'Invalid Date';
      }
    };

    return {
      apiKeys,
      loading,
      copyKey,
      toggleVisibility,
      deleteKey,
      formatDate
    }
  },
  template: `
    <div class="api-keys-page">
      <div class="page-header">
        <h1><i class="fas fa-key"></i> API Keys Management</h1>
        <p>Manage API keys for MCP and REST API access</p>
        <button class="btn btn-primary" @click="apiKeysStore.openCreateModal">
          <i class="fas fa-plus"></i> Create New Key
        </button>
      </div>

      <div class="api-keys-content">
        <div v-if="apiKeysStore.loading" class="loading-state">
          <i class="fas fa-spinner fa-spin"></i>
          <span>Loading API keys...</span>
        </div>

        <div v-else-if="apiKeysStore.apiKeys.length === 0" class="empty-state">
          <i class="fas fa-key"></i>
          <h3>No API Keys</h3>
          <p>Create your first API key to start using the MCP and REST APIs</p>
          <button class="btn btn-primary" @click="apiKeysStore.openCreateModal">
            <i class="fas fa-plus"></i> Create API Key
          </button>
        </div>

        <div v-else class="keys-grid">
          <div v-for="key in apiKeysStore.apiKeys" :key="key.id" class="key-card">
            <div class="key-header">
              <div class="key-info">
                <h3>{{ key.name }}</h3>
                <span class="key-type">{{ key.type.toUpperCase() }}</span>
              </div>
              <div class="key-actions">
                <button class="btn btn-sm btn-outline" @click="apiKeysStore.copyKey(key.key)" title="Copy key">
                  <i class="fas fa-copy"></i>
                </button>
                <button class="btn btn-sm btn-outline" @click="apiKeysStore.toggleVisibility(key.id)" :title="key.showKey ? 'Hide key' : 'Show key'">
                  <i :class="key.showKey ? 'fas fa-eye-slash' : 'fas fa-eye'"></i>
                </button>
                <button class="btn btn-sm btn-danger" @click="apiKeysStore.deleteKey(key.id)" title="Delete key">
                  <i class="fas fa-trash"></i>
                </button>
              </div>
            </div>
            <div class="key-details">
              <div class="key-value">
                <code v-if="key.showKey">{{ key.key }}</code>
                <code v-else>••••••••••••••••••••••••••••••••</code>
              </div>
              <div class="key-meta">
                <span class="key-status" :class="key.status">
                  <i class="fas fa-circle"></i>
                  {{ key.status }}
                </span>
                <span class="key-created">
                  Created: {{ apiKeysStore.formatDate(key.createdAt) }}
                </span>
                <span class="key-last-used" v-if="key.lastUsed">
                  Last used: {{ apiKeysStore.formatDate(key.lastUsed) }}
                </span>
              </div>
              <div v-if="key.description" class="key-description">
                {{ key.description }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  `
}
