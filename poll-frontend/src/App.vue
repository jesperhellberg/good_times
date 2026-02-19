<template>
  <div class="top-bar">
    <LocaleToggle v-if="isLanding" />
    <div v-if="isLanding && !isAuthed" class="auth-nav">
      <button
        type="button"
        class="btn btn-ghost"
        :class="{ active: authPanel.open && authPanel.mode === 'login' }"
        @click="showAuthPanel('login')"
      >
        {{ t('auth.login') }}
      </button>
      <button
        type="button"
        class="btn btn-ghost"
        :class="{ active: authPanel.open && authPanel.mode === 'signup' }"
        @click="showAuthPanel('signup')"
      >
        {{ t('auth.signup') }}
      </button>
    </div>
    <div v-if="showAdminBar" class="admin-bar">
      <span class="text-sm text-muted">
        {{ t('auth.signedInAs', { name: adminName || '—' }) }}
      </span>
      <button type="button" class="btn btn-ghost admin-logout" @click="logout">
        {{ t('auth.logout') }}
      </button>
    </div>
  </div>

  <router-view v-slot="{ Component }">
    <transition name="fade" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>
</template>

<script setup>
import { computed, onMounted, onUnmounted, provide, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LocaleToggle from './components/LocaleToggle.vue'
import { api } from './api'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const adminName = ref(localStorage.getItem('adminName') || '')
const adminToken = ref(localStorage.getItem('adminToken') || '')
const authPanelOpen = ref(false)
const authPanelMode = ref('login')
const authPanel = { open: authPanelOpen, mode: authPanelMode }

const isAuthed = computed(() => Boolean(adminToken.value))
const showAdminBar = computed(() => isAuthed.value)
const isLanding = computed(() => route.path === '/')

function showAuthPanel(nextMode) {
  authPanelMode.value = nextMode
  authPanelOpen.value = true
}

function hideAuthPanel() {
  authPanelOpen.value = false
}

provide('authPanel', {
  open: authPanel.open,
  mode: authPanel.mode,
  show: showAuthPanel,
  hide: hideAuthPanel,
})

provide('authState', {
  isAuthed,
})

function syncAdminName() {
  adminName.value = localStorage.getItem('adminName') || ''
  adminToken.value = localStorage.getItem('adminToken') || ''
}

watch(
  () => route.fullPath,
  () => {
    syncAdminName()
    if (route.path !== '/' || isAuthed.value) {
      hideAuthPanel()
    }
  }
)

watch(isAuthed, (nextAuthed) => {
  if (nextAuthed) {
    hideAuthPanel()
  }
})

onMounted(() => {
  syncAdminName()
  window.addEventListener('storage', syncAdminName)
})

onUnmounted(() => {
  window.removeEventListener('storage', syncAdminName)
})

async function logout() {
  try {
    await api.logoutAdmin()
  } catch (_) {
    // ignore logout errors
  }
  localStorage.removeItem('adminToken')
  localStorage.removeItem('adminName')
  syncAdminName()
  router.push('/')
}
</script>

<style scoped>
.top-bar {
  position: absolute;
  top: 1.25rem;
  right: 1.5rem;
  display: inline-flex;
  align-items: center;
  gap: 0.75rem;
  z-index: 10;
  max-width: calc(100vw - 1.5rem);
}

.auth-nav {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
}

.auth-nav .btn {
  padding: 0.35rem 0.7rem;
  font-size: 0.75rem;
}

.auth-nav .btn.active {
  border-color: var(--accent);
  color: var(--ink);
  box-shadow: 0 0 0 3px var(--accent-dim);
}

.admin-bar {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0.5rem;
  background: #fff;
  border: 1px solid var(--paper-2);
  border-radius: 999px;
  box-shadow: var(--shadow-sm);
}

.admin-logout {
  padding: 0.35rem 0.6rem;
  font-size: 0.75rem;
}

@media (max-width: 600px) {
  .top-bar {
    top: 0.75rem;
    right: 0.75rem;
    gap: 0.5rem;
    flex-wrap: wrap;
    justify-content: flex-end;
  }

  .auth-nav .btn {
    padding: 0.3rem 0.5rem;
  }

  .admin-bar {
    padding: 0.2rem 0.4rem;
    gap: 0.35rem;
    max-width: min(100%, 20rem);
  }

  .admin-bar .text-sm {
    max-width: 10rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .admin-logout {
    padding: 0.3rem 0.5rem;
    font-size: 0.7rem;
  }
}
</style>
