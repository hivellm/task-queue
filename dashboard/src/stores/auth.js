// Use global Pinia and Vue (available via CDN)
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  // State
  const isAuthenticated = ref(false)
  const user = ref(null)
  const token = ref(null)
  
  // Getters
  const isLoggedIn = computed(() => isAuthenticated.value && user.value !== null)
  
  // Actions
  const checkAuthentication = async () => {
    try {
      if (window.authManager && window.authManager.isAuthenticated) {
        isAuthenticated.value = true
        user.value = window.authManager.user
        token.value = window.authManager.token
        return true
      }
      return false
    } catch (error) {
      console.error('Authentication check failed:', error)
      return false
    }
  }
  
  const login = async (username, password) => {
    try {
      const result = await window.authManager.login(username, password)
      if (result.success) {
        isAuthenticated.value = true
        user.value = result.user
        token.value = result.token
        return { success: true }
      }
      return { success: false, error: result.error }
    } catch (error) {
      console.error('Login failed:', error)
      return { success: false, error: 'Login failed' }
    }
  }
  
  const logout = () => {
    try {
      if (window.authManager) {
        window.authManager.logout()
      }
      isAuthenticated.value = false
      user.value = null
      token.value = null
      // Redirect to login page
      window.location.href = '/dashboard/login.html'
    } catch (error) {
      console.error('Logout failed:', error)
    }
  }
  
  const refreshToken = async () => {
    try {
      if (window.authManager) {
        const result = await window.authManager.refreshToken()
        if (result.success) {
          token.value = result.token
          return true
        }
      }
      return false
    } catch (error) {
      console.error('Token refresh failed:', error)
      return false
    }
  }
  
  return {
    // State
    isAuthenticated,
    user,
    token,
    // Getters
    isLoggedIn,
    // Actions
    checkAuthentication,
    login,
    logout,
    refreshToken
  }
})
