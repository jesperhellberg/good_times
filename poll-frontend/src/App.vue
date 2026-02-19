<template>
  <div class="top-bar">
    <LocaleToggle />
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
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LocaleToggle from './components/LocaleToggle.vue'
import { api } from './api'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const adminName = ref(localStorage.getItem('adminName') || '')

const showAdminBar = computed(() => route.path.startsWith('/admin'))

function syncAdminName() {
  adminName.value = localStorage.getItem('adminName') || ''
}

watch(
  () => route.fullPath,
  () => {
    syncAdminName()
  }
)

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
  position: fixed;
  top: 1.25rem;
  right: 1.5rem;
  display: inline-flex;
  align-items: center;
  gap: 0.75rem;
  z-index: 10;
  max-width: calc(100vw - 1.5rem);
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
  }

  .admin-bar {
    display: none;
  }
}
</style>
