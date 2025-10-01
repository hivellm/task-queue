// Use global Vue
const { ref, onMounted, onUnmounted } = Vue;

export default {
  name: 'UserAvatar',
  setup() {
    const showUserDropdown = ref(false)

    const toggleUserDropdown = () => {
      showUserDropdown.value = !showUserDropdown.value
    }

    const getUserInitials = (username) => {
      if (!username) return 'U'
      return username.substring(0, 2).toUpperCase()
    }

    const logout = () => {
      try {
        if (window.authManager) {
          window.authManager.logout()
        }
        // Redirect to login page
        window.location.href = '/dashboard/login.html'
      } catch (error) {
        console.error('Logout failed:', error)
      }
    }

    const handleClickOutside = (event) => {
      if (showUserDropdown.value && !event.target.closest('.user-dropdown')) {
        showUserDropdown.value = false
      }
    }

    onMounted(() => {
      document.addEventListener('click', handleClickOutside)
    })

    onUnmounted(() => {
      document.removeEventListener('click', handleClickOutside)
    })

    return {
      showUserDropdown,
      toggleUserDropdown,
      getUserInitials,
      logout
    }
  },
  template: `
    <div class="user-info">
      <div class="user-dropdown" @click="toggleUserDropdown" :class="{ 'active': showUserDropdown }">
        <div class="user-avatar">
          <div class="avatar-circle">
            {{ getUserInitials('User') }}
          </div>
          <div class="user-details">
            <span class="user-name">User</span>
            <span class="user-role">Admin</span>
          </div>
          <i class="fas fa-chevron-down dropdown-arrow"></i>
        </div>
        <div class="dropdown-menu" v-if="showUserDropdown">
          <div class="dropdown-header">
            <div class="avatar-circle large">
              {{ getUserInitials('User') }}
            </div>
            <div class="user-info-details">
              <div class="user-name">User</div>
              <div class="user-email">user@example.com</div>
              <div class="user-roles">Admin</div>
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
  `
}
