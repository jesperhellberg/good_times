<template>
  <div class="container landing-stack">
    <section
      ref="heroRef"
      class="landing-hero"
      :class="{ 'is-auth-open': isAuthPanelOpen }"
      :style="heroStyle"
    >
      <div class="landing-copy" v-show="!isAuthPanelOpen">
        <router-link class="eyebrow" :to="homeTarget">{{ t('landing.eyebrow') }}</router-link>
        <h1 style="margin-top: 0.5rem;">{{ t('landing.title') }}</h1>
        <p class="text-muted" style="margin-top: 0.75rem; max-width: 38rem;">
          {{ t('landing.blurb1') }}
        </p>
        <p class="text-muted" style="margin-top: 0.75rem; max-width: 38rem;">
          {{ t('landing.blurb2') }}
        </p>
      </div>

      <div class="hero-panel" v-show="isAuthPanelOpen">
        <div class="hero-panel-inner">
          <button
            v-if="authPanel"
            type="button"
            class="auth-close"
            aria-label="Close"
            @click="authPanel.hide()"
          >
            <span aria-hidden="true">×</span>
          </button>
          <div class="auth-layout">
            <div class="auth-intro">
              <div class="panel-kicker">Admin</div>
              <h2 class="panel-title">{{ mode === 'login' ? t('auth.login') : t('auth.signup') }}</h2>
            </div>

            <form class="auth-body" @submit.prevent="submit" novalidate>
              <div class="auth-fields">
                <div class="field">
                  <input
                    id="admin-name"
                    v-model="form.name"
                    type="text"
                    autocomplete="username"
                    :placeholder="t('auth.namePlaceholder')"
                    required
                  />
                </div>

                <div class="field">
                  <input
                    id="admin-password"
                    v-model="form.password"
                    type="password"
                    :autocomplete="mode === 'signup' ? 'new-password' : 'current-password'"
                    :placeholder="t('auth.passwordPlaceholder')"
                    required
                  />
                </div>
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
      </div>
    </section>
  </div>
</template>

<script setup>
import { computed, inject, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { api } from '../api'

const { t } = useI18n()
const router = useRouter()
const authPanel = inject('authPanel', null)
const authState = inject('authState', null)
const heroRef = ref(null)
const heroHeight = ref(0)
let resizeObserver

const mode = ref('login')
const form = reactive({
  name: '',
  password: '',
})
const submitting = ref(false)
const error = ref(null)
const isAuthPanelOpen = computed(() => authPanel?.open?.value ?? false)
const homeTarget = computed(() => (authState?.isAuthed?.value ? '/admin' : '/'))
const heroStyle = computed(() => (isAuthPanelOpen.value && heroHeight.value
  ? { height: `${heroHeight.value}px` }
  : {}))

function updateHeroHeight(force = false) {
  if (!heroRef.value || (!force && isAuthPanelOpen.value)) return
  heroHeight.value = heroRef.value.offsetHeight || 0
}

function resetAuthForm() {
  form.name = ''
  form.password = ''
  error.value = null
  submitting.value = false
}

if (authPanel?.mode) {
  watch(
    () => authPanel.mode.value,
    (nextMode) => {
      mode.value = nextMode
    },
    { immediate: true }
  )
}

watch(mode, (nextMode) => {
  if (authPanel?.mode) {
    authPanel.mode.value = nextMode
  }
})

watch(
  isAuthPanelOpen,
  (nextOpen) => {
    if (nextOpen) {
      updateHeroHeight(true)
    } else {
      resetAuthForm()
    }
  },
  { flush: 'pre' }
)

onMounted(() => {
  updateHeroHeight()
  if (heroRef.value && 'ResizeObserver' in window) {
    resizeObserver = new ResizeObserver(() => updateHeroHeight())
    resizeObserver.observe(heroRef.value)
  }
})

onBeforeUnmount(() => {
  if (resizeObserver && heroRef.value) {
    resizeObserver.unobserve(heroRef.value)
    resizeObserver.disconnect()
  }
})

function saveSession({ token, name }) {
  localStorage.setItem('adminToken', token)
  localStorage.setItem('adminName', name)
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
    if (e?.status === 401 || e?.status === 403) {
      error.value = t('auth.errors.invalidCredentials')
    } else {
      error.value = e.message
    }
  } finally {
    submitting.value = false
  }
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
  grid-template-areas: "stack";
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

.landing-copy,
.hero-panel {
  grid-area: stack;
}

.landing-copy {
  justify-self: start;
  align-self: start;
}

.hero-panel {
  justify-self: stretch;
  align-self: start;
  height: 100%;
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
  max-width: 100%;
  padding: 1.3rem 1.6rem;
  border-radius: 14px;
  border: 1px solid rgba(26, 26, 26, 0.08);
  background: rgba(255, 255, 255, 0.85);
  backdrop-filter: blur(6px);
  box-shadow: 0 10px 24px rgba(0, 0, 0, 0.06);
  position: relative;
  height: 100%;
  overflow: auto;
}

.auth-layout {
  display: flex;
  flex-direction: column;
  gap: 0.9rem;
  align-items: center;
  text-align: center;
}

.auth-intro {
  max-width: 28rem;
}

.auth-fields {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.75rem;
  width: 100%;
}

.auth-close {
  position: absolute;
  top: 0.75rem;
  right: 0.75rem;
  width: 2rem;
  height: 2rem;
  border-radius: 999px;
  border: 1px solid rgba(26, 26, 26, 0.12);
  background: #fff;
  color: var(--ink);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 1.1rem;
  line-height: 1;
  cursor: pointer;
  box-shadow: 0 6px 14px rgba(0, 0, 0, 0.08);
}

.auth-close:hover {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-dim);
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
  gap: 0.55rem;
  margin-top: 0;
  width: 100%;
}

.auth-title {
  margin: 0;
  font-size: 1.1rem;
}

.auth-submit {
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
  align-self: stretch;
}

@media (max-width: 900px) {
  .landing-stack {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }

  .landing-hero {
    padding: 1.8rem 1.4rem;
  }

  .auth-fields {
    grid-template-columns: 1fr;
  }
}
</style>
