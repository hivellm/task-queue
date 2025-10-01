// Use global Vue and Vue Router
const { ref, computed, onMounted } = Vue;
const { useRoute } = VueRouter;

export default {
  name: 'App',
  components: {
    UserAvatar: {
      template: `
        <div class="user-info">
          <div class="user-dropdown">
            <div class="user-avatar">
              <div class="avatar-circle">U</div>
              <div class="user-details">
                <span class="user-name">User</span>
                <span class="user-role">Admin</span>
              </div>
              <i class="fas fa-chevron-down dropdown-arrow"></i>
            </div>
          </div>
        </div>
      `
    }
  },
  setup() {
    const route = useRoute()

    const loading = ref(false)
    const connected = ref(false)
    const isAuthenticated = ref(false)
    const user = ref(null)

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

    const currentRouteIcon = computed(() => {
      const routeConfig = routes.find(r => r.name === route.name)
      return routeConfig ? routeConfig.icon : 'fas fa-home'
    })

    const checkAuthentication = async () => {
      try {
        if (window.authManager && window.authManager.isAuthenticated) {
          isAuthenticated.value = true
          user.value = window.authManager.user
          return true
        }
        return false
      } catch (error) {
        console.error('Authentication check failed:', error)
        return false
      }
    }

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
        await checkAuthentication()
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
      currentRouteIcon,
      isAuthenticated
    }
  },
  template: `
    <div id="app" class="dashboard">
      <!-- Sidebar -->
      <nav class="sidebar">
        <div class="sidebar-header">
          <h1>Task Queue</h1>
        </div>
        <ul class="nav-menu">
          <li v-for="route in routes" :key="route.name" class="nav-item" :class="{ active: $route.name === route.name }">
            <router-link :to="route.path" class="nav-link">
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
          <div class="breadcrumb">
            <i :class="currentRouteIcon"></i>
            {{ pageTitle }}
          </div>
          <div class="top-bar-actions">
            <div class="connection-status">
              <div class="status-dot" :class="{ online: connected }"></div>
              <span>{{ connected ? 'Connected' : 'Disconnected' }}</span>
            </div>
            <UserAvatar v-if="isAuthenticated" />
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
  `
}
