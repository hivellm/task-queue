// Use global Pinia and Vue (available via CDN)
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useApiKeysStore = defineStore('apiKeys', () => {
  // State
  const apiKeys = ref([])
  const loading = ref(false)
  const showCreateModal = ref(false)
  const creating = ref(false)
  const newApiKey = ref({
    name: '',
    type: 'mcp',
    description: '',
    expiryDate: '',
    showKeyImmediately: false
  })
  
  // Getters
  const activeKeys = computed(() => apiKeys.value.filter(key => key.status === 'active'))
  const mcpKeys = computed(() => apiKeys.value.filter(key => key.type === 'mcp'))
  const restKeys = computed(() => apiKeys.value.filter(key => key.type === 'rest'))
  
  // Actions
  const loadApiKeys = async () => {
    loading.value = true
    try {
      // Mock data for now - replace with actual API call
      apiKeys.value = [
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
      ]
    } catch (error) {
      console.error('Failed to load API keys:', error)
      throw error
    } finally {
      loading.value = false
    }
  }
  
  const createApiKey = async () => {
    if (!newApiKey.value.name.trim() || creating.value) return
    
    creating.value = true
    try {
      const generatedKey = `${newApiKey.value.type}_${Math.random().toString(36).substring(2, 15)}${Math.random().toString(36).substring(2, 15)}`
      
      const key = {
        id: Date.now().toString(),
        name: newApiKey.value.name,
        type: newApiKey.value.type,
        key: generatedKey,
        status: 'active',
        createdAt: new Date().toISOString(),
        lastUsed: null,
        showKey: newApiKey.value.showKeyImmediately,
        description: newApiKey.value.description,
        expiryDate: newApiKey.value.expiryDate
      }
      
      apiKeys.value.unshift(key)
      
      // Reset form
      resetNewApiKey()
      showCreateModal.value = false
      
      return { success: true, key }
    } catch (error) {
      console.error('Failed to create API key:', error)
      return { success: false, error: error.message }
    } finally {
      creating.value = false
    }
  }
  
  const deleteApiKey = async (keyId) => {
    try {
      apiKeys.value = apiKeys.value.filter(k => k.id !== keyId)
      return { success: true }
    } catch (error) {
      console.error('Failed to delete API key:', error)
      return { success: false, error: error.message }
    }
  }
  
  const toggleKeyVisibility = (keyId) => {
    const key = apiKeys.value.find(k => k.id === keyId)
    if (key) {
      key.showKey = !key.showKey
    }
  }
  
  const copyApiKey = async (key) => {
    try {
      await navigator.clipboard.writeText(key)
      return { success: true }
    } catch (error) {
      console.error('Failed to copy API key:', error)
      return { success: false, error: error.message }
    }
  }
  
  const openCreateModal = () => {
    showCreateModal.value = true
  }
  
  const closeCreateModal = () => {
    showCreateModal.value = false
    resetNewApiKey()
  }
  
  const resetNewApiKey = () => {
    newApiKey.value = {
      name: '',
      type: 'mcp',
      description: '',
      expiryDate: '',
      showKeyImmediately: false
    }
  }
  
  const formatDate = (dateString) => {
    if (!dateString) return 'N/A'
    try {
      const date = new Date(dateString)
      return date.toLocaleDateString('pt-BR', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      })
    } catch (error) {
      return 'Invalid Date'
    }
  }
  
  return {
    // State
    apiKeys,
    loading,
    showCreateModal,
    creating,
    newApiKey,
    // Getters
    activeKeys,
    mcpKeys,
    restKeys,
    // Actions
    loadApiKeys,
    createApiKey,
    deleteApiKey,
    toggleKeyVisibility,
    copyApiKey,
    openCreateModal,
    closeCreateModal,
    resetNewApiKey,
    formatDate
  }
})
