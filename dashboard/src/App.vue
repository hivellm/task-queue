<template>
  <div id="app" class="dashboard">
    <!-- Sidebar -->
    <nav class="sidebar">
      <div class="sidebar-header">
        <h2>Task Queue</h2>
      </div>
      <ul class="nav-list">
        <li v-for="route in routes" :key="route.name" class="nav-item">
          <router-link :to="route.path" class="nav-link" :class="{ active: $route.name === route.name }">
            <i :class="route.icon"></i>
            <span>{{ route.label }}</span>
          </router-link>
        </li>
      </ul>
    </nav>

    <!-- Main Content -->
    <main class="main-content">
      <!-- Top Bar -->
      <header class="top-bar">
        <div class="top-bar-left">
          <h1>{{ pageTitle }}</h1>
        </div>
        <div class="top-bar-right">
          <div class="connection-status" :class="{ connected: connected }">
            <i class="fas fa-circle"></i>
            <span>{{ connected ? 'Connected' : 'Disconnected' }}</span>
          </div>
          <UserAvatar v-if="authStore.isAuthenticated" />
        </div>
      </header>

      <!-- Page Content -->
      <div class="page-content">
        <router-view />
      </div>
    </main>

    <!-- Loading Overlay -->
    <div v-if="loading" class="loading-overlay">
      <div class="loading-spinner">
        <i class="fas fa-spinner fa-spin"></i>
        <span>Loading...</span>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from './stores/auth'
import UserAvatar from './components/UserAvatar.vue'

export default {
  name: 'App',
  components: {
    UserAvatar
  },
  setup() {
    const route = useRoute()
    const authStore = useAuthStore()
    
    const loading = ref(false)
    const connected = ref(false)
    
    const routes = [
      { name: 'overview', path: '/', label: 'Overview', icon: 'fas fa-tachometer-alt' },
      { name: 'projects', path: '/projects', label: 'Projects', icon: 'fas fa-folder' },
      { name: 'tasks', path: '/tasks', label: 'Tasks', icon: 'fas fa-tasks' },
      { name: 'workflows', path: '/workflows', label: 'Workflows', icon: 'fas fa-project-diagram' },
      { name: 'dependencies', path: '/dependencies', label: 'Dependencies', icon: 'fas fa-sitemap' },
      { name: 'api-keys', path: '/api-keys', label: 'API Keys', icon: 'fas fa-key' },
      { name: 'metrics', path: '/metrics', label: 'Metrics', icon: 'fas fa-chart-line' },
      { name: 'console', path: '/console', label: 'Console', icon: 'fas fa-terminal' }
    ]
    
    const pageTitle = computed(() => {
      const routeConfig = routes.find(r => r.name === route.name)
      return routeConfig ? routeConfig.label : 'Dashboard'
    })
    
    const checkConnection = async () => {
      try {
        const connection = await window.apiClient.testConnection()
        connected.value = connection.connected
      } catch (error) {
        console.error('Connection check failed:', error)
        connected.value = false
      }
    }
    
    onMounted(async () => {
      loading.value = true
      try {
        await authStore.checkAuthentication()
        await checkConnection()
      } finally {
        loading.value = false
      }
    })
    
    return {
      loading,
      connected,
      routes,
      pageTitle,
      authStore
    }
  }
}
</script>

<style scoped>
.dashboard {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
}

.sidebar {
  width: 250px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: var(--space-4);
  border-bottom: 1px solid var(--border);
}

.sidebar-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
  flex: 1;
}

.nav-item {
  margin: 0;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  color: var(--text-secondary);
  text-decoration: none;
  transition: var(--transition);
  border-left: 3px solid transparent;
}

.nav-link:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.nav-link.active {
  background: var(--bg-tertiary);
  color: var(--primary);
  border-left-color: var(--primary);
}

.nav-link i {
  width: 16px;
  text-align: center;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-4);
  background: var(--bg-card);
  border-bottom: 1px solid var(--border);
}

.top-bar-left h1 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--text-primary);
}

.top-bar-right {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.connection-status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: 14px;
  color: var(--text-muted);
}

.connection-status.connected {
  color: var(--success);
}

.connection-status i {
  font-size: 8px;
}

.page-content {
  flex: 1;
  padding: var(--space-6);
  overflow-y: auto;
}

.loading-overlay {
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

.loading-spinner {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-3);
  color: white;
  font-size: 16px;
}

.loading-spinner i {
  font-size: 24px;
}
</style>
