<template>
  <div class="user-info">
    <div class="user-dropdown" @click="toggleUserDropdown" :class="{ 'active': showUserDropdown }">
      <div class="user-avatar">
        <div class="avatar-circle">
          {{ getUserInitials(authStore.user?.username) }}
        </div>
        <div class="user-details">
          <span class="user-name">{{ authStore.user?.username }}</span>
          <span class="user-role">{{ authStore.user?.roles?.join(', ') }}</span>
        </div>
        <i class="fas fa-chevron-down dropdown-arrow"></i>
      </div>
      <div class="dropdown-menu" v-if="showUserDropdown">
        <div class="dropdown-header">
          <div class="avatar-circle large">
            {{ getUserInitials(authStore.user?.username) }}
          </div>
          <div class="user-info-details">
            <div class="user-name">{{ authStore.user?.username }}</div>
            <div class="user-email">{{ authStore.user?.email }}</div>
            <div class="user-roles">{{ authStore.user?.roles?.join(', ') }}</div>
          </div>
        </div>
        <div class="dropdown-divider"></div>
        <div class="dropdown-item" @click="logout">
          <i class="fas fa-sign-out-alt"></i>
          <span>Logout</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted, onUnmounted } from 'vue'
import { useAuthStore } from '../stores/auth'

export default {
  name: 'UserAvatar',
  setup() {
    const authStore = useAuthStore()
    const showUserDropdown = ref(false)
    
    const toggleUserDropdown = () => {
      showUserDropdown.value = !showUserDropdown.value
    }
    
    const getUserInitials = (username) => {
      if (!username) return 'U'
      return username.substring(0, 2).toUpperCase()
    }
    
    const logout = () => {
      authStore.logout()
      showUserDropdown.value = false
    }
    
    const handleClickOutside = (event) => {
      if (showUserDropdown.value && !event.target.closest('.user-dropdown')) {
        showUserDropdown.value = false
      }
    }
    
    const handleKeydown = (event) => {
      if (event.key === 'Escape' && showUserDropdown.value) {
        showUserDropdown.value = false
      }
    }
    
    onMounted(() => {
      document.addEventListener('click', handleClickOutside)
      document.addEventListener('keydown', handleKeydown)
    })
    
    onUnmounted(() => {
      document.removeEventListener('click', handleClickOutside)
      document.removeEventListener('keydown', handleKeydown)
    })
    
    return {
      authStore,
      showUserDropdown,
      toggleUserDropdown,
      getUserInitials,
      logout
    }
  }
}
</script>

<style scoped>
.user-info {
  position: relative;
}

.user-dropdown {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: var(--transition);
}

.user-dropdown:hover {
  background: var(--bg-tertiary);
}

.user-dropdown.active {
  background: var(--bg-tertiary);
}

.user-avatar {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.avatar-circle {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--primary);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
}

.avatar-circle.large {
  width: 48px;
  height: 48px;
  font-size: 16px;
}

.user-details {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.user-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.user-role {
  font-size: 12px;
  color: var(--text-muted);
}

.dropdown-arrow {
  font-size: 12px;
  color: var(--text-muted);
  transition: var(--transition);
}

.user-dropdown.active .dropdown-arrow {
  transform: rotate(180deg);
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: var(--space-2);
  background: var(--bg-card);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-width: 250px;
  z-index: 1000;
}

.dropdown-header {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-4);
}

.user-info-details {
  flex: 1;
}

.user-info-details .user-name {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: var(--space-1);
}

.user-email {
  font-size: 14px;
  color: var(--text-secondary);
  margin-bottom: var(--space-1);
}

.user-roles {
  font-size: 12px;
  color: var(--text-muted);
}

.dropdown-divider {
  height: 1px;
  background: var(--border);
  margin: 0;
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  color: var(--text-primary);
  cursor: pointer;
  transition: var(--transition);
}

.dropdown-item:hover {
  background: var(--bg-tertiary);
}

.dropdown-item i {
  width: 16px;
  text-align: center;
  color: var(--text-muted);
}
</style>
