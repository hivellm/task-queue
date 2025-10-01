import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import './style.css'

// Import views
import Overview from './views/Overview.vue'
import Projects from './views/Projects.vue'
import Tasks from './views/Tasks.vue'
import Workflows from './views/Workflows.vue'
import Dependencies from './views/Dependencies.vue'
import ApiKeys from './views/ApiKeys.vue'
import Metrics from './views/Metrics.vue'
import Console from './views/Console.vue'

// Router configuration
const routes = [
  { path: '/', name: 'overview', component: Overview },
  { path: '/projects', name: 'projects', component: Projects },
  { path: '/tasks', name: 'tasks', component: Tasks },
  { path: '/workflows', name: 'workflows', component: Workflows },
  { path: '/dependencies', name: 'dependencies', component: Dependencies },
  { path: '/api-keys', name: 'api-keys', component: ApiKeys },
  { path: '/metrics', name: 'metrics', component: Metrics },
  { path: '/console', name: 'console', component: Console }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

const pinia = createPinia()

const app = createApp(App)

app.use(pinia)
app.use(router)

app.mount('#app')
