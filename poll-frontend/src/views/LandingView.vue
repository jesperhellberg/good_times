<template>
  <div class="container landing-stack">
    <section class="landing-hero">
      <div class="landing-copy">
        <router-link class="eyebrow" to="/">{{ t('landing.eyebrow') }}</router-link>
        <h1 style="margin-top: 0.5rem;">{{ t('landing.title') }}</h1>
        <p class="text-muted" style="margin-top: 0.75rem; max-width: 38rem;">
          {{ t('landing.blurb1') }}
        </p>
        <p class="text-muted" style="margin-top: 0.75rem; max-width: 38rem;">
          {{ t('landing.blurb2') }}
        </p>
      </div>

      <div class="hero-panel">
        <div class="hero-panel-inner">
          <div class="panel-kicker">Admin</div>
          <h2 class="panel-title">{{ t('auth.title') }}</h2>
          <p class="text-muted text-sm" style="margin: 0.25rem 0 0;">
            {{ t('auth.subtitle') }}
          </p>

          <div v-if="isAuthed" class="auth-body">
            <p class="text-sm" style="margin: 0;">
              {{ t('auth.signedInAs', { name: adminName || '—' }) }}
            </p>
            <div class="auth-actions">
              <button type="button" class="btn btn-primary" @click="goToAdmin">
                {{ t('auth.goToAdmin') }}
              </button>
              <button type="button" class="btn btn-ghost" @click="logout">
                {{ t('auth.logout') }}
              </button>
            </div>
          </div>

          <form v-else class="auth-body" @submit.prevent="submit" novalidate>
            <div class="auth-toggle">
              <button
                type="button"
                class="btn btn-ghost"
                :class="{ active: mode === 'login' }"
                @click="mode = 'login'"
              >
                {{ t('auth.login') }}
              </button>
              <button
                type="button"
                class="btn btn-ghost"
                :class="{ active: mode === 'signup' }"
                @click="mode = 'signup'"
              >
                {{ t('auth.signup') }}
              </button>
            </div>

            <div class="field">
              <label for="admin-name">{{ t('auth.nameLabel') }}</label>
              <input
                id="admin-name"
                v-model="form.name"
                type="text"
                autocomplete="username"
                required
              />
            </div>

            <div class="field">
              <label for="admin-password">{{ t('auth.passwordLabel') }}</label>
              <input
                id="admin-password"
                v-model="form.password"
                type="password"
                :autocomplete="mode === 'signup' ? 'new-password' : 'current-password'"
                required
              />
            </div>

            <button type="submit" class="btn btn-primary auth-submit" :disabled="submitting">
              {{ submitting ? t('auth.submitting') : mode === 'login' ? t('auth.login') : t('auth.signup') }}
            </button>

            <p v-if="error" class="text-sm" style="color: var(--no);">
              {{ error }}
            </p>
          </form>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { computed, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { api } from '../api'

const { t } = useI18n()
const router = useRouter()

const mode = ref('login')
const form = reactive({
  name: '',
  password: '',
})
const submitting = ref(false)
const error = ref(null)
const adminName = ref(localStorage.getItem('adminName') || '')

const isAuthed = computed(() => Boolean(localStorage.getItem('adminToken')))

function saveSession({ token, name }) {
  localStorage.setItem('adminToken', token)
  localStorage.setItem('adminName', name)
  adminName.value = name
}

function clearSession() {
  localStorage.removeItem('adminToken')
  localStorage.removeItem('adminName')
  adminName.value = ''
}

async function submit() {
  error.value = null
  if (!form.name.trim() || !form.password) {
    error.value = t('auth.errors.required')
    return
  }

  submitting.value = true
  try {
    const payload = { name: form.name.trim(), password: form.password }
    const result =
      mode.value === 'login'
        ? await api.loginAdmin(payload)
        : await api.signupAdmin(payload)

    saveSession(result)
    await router.push('/admin')
  } catch (e) {
    error.value = e.message
  } finally {
    submitting.value = false
  }
}

async function logout() {
  try {
    await api.logoutAdmin()
  } catch (_) {
    // Ignore logout errors and clear local session anyway.
  }
  clearSession()
}

function goToAdmin() {
  router.push('/admin')
}
</script>

<style scoped>
.landing-stack {
  min-height: 70vh;
  display: flex;
  flex-direction: column;
  gap: 2rem;
  justify-content: center;
  padding-top: 2.5rem;
  padding-bottom: 2.5rem;
}

.landing-hero {
  position: relative;
  display: grid;
  grid-template-columns: minmax(0, 1.2fr) minmax(0, 0.8fr);
  gap: 2.8rem;
  padding: 2.4rem 2.4rem;
  border-radius: 18px;
  background:
    radial-gradient(circle at 20% 10%, rgba(212, 98, 42, 0.15), transparent 55%),
    radial-gradient(circle at 85% 30%, rgba(42, 125, 79, 0.12), transparent 60%),
    linear-gradient(135deg, #fff7ef 0%, #fff 55%, #f8f3ea 100%);
  border: 1px solid rgba(212, 98, 42, 0.15);
  overflow: hidden;
}

.landing-hero::after {
  content: '';
  position: absolute;
  inset: auto -10% -40% auto;
  width: 360px;
  height: 360px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(42, 125, 79, 0.18), transparent 65%);
  pointer-events: none;
}

.hero-panel {
  display: flex;
  align-items: stretch;
  justify-content: flex-start;
}

.hero-panel-inner {
  width: 100%;
  max-width: 420px;
  padding: 1.2rem 1.3rem;
  border-radius: 14px;
  border: 1px solid rgba(26, 26, 26, 0.08);
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(6px);
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.06);
}

.panel-kicker {
  font-size: 0.7rem;
  letter-spacing: 0.2em;
  text-transform: uppercase;
  color: var(--ink-muted);
}

.panel-title {
  margin: 0.35rem 0 0;
  font-size: 1.4rem;
}

.auth-body {
  display: flex;
  flex-direction: column;
  gap: 0.65rem;
  margin-top: 1rem;
}

.auth-title {
  margin: 0;
  font-size: 1.1rem;
}

.auth-toggle {
  display: flex;
  gap: 0.5rem;
}

.auth-toggle .btn {
  flex: 1;
  padding: 0.45rem 0.8rem;
  font-size: 0.8rem;
}

.auth-toggle .btn.active {
  border-color: var(--accent);
  color: var(--ink);
  box-shadow: 0 0 0 3px var(--accent-dim);
}

.auth-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.6rem;
}

.auth-submit {
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
}

@media (max-width: 900px) {
  .landing-stack {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }

  .landing-hero {
    grid-template-columns: 1fr;
    padding: 1.8rem 1.4rem;
  }
}
</style>
