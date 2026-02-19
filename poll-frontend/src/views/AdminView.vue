<template>
  <div class="container">
    <header class="page-header">
      <router-link class="eyebrow" :to="homeTarget">{{ t('landing.eyebrow') }}</router-link>
      <h1>{{ t('admin.title') }}</h1>
      <p class="text-muted" style="margin-top: 0.5rem">
        {{ t('admin.subtitle') }}
      </p>
    </header>

    <form @submit.prevent="submit" novalidate>
      <div class="card" style="display: flex; flex-direction: column; gap: 1.25rem;">

        <div class="field">
          <label for="title">{{ t('admin.form.titleLabel') }}</label>
          <input
            id="title"
            v-model="form.title"
            type="text"
            :placeholder="t('admin.form.titlePlaceholder')"
            required
            autofocus
          />
        </div>

        <div class="field">
          <label for="description">
            {{ t('admin.form.descriptionLabel') }}
            <span class="text-muted">({{ t('common.optional') }})</span>
          </label>
          <textarea
            id="description"
            v-model="form.description"
            :placeholder="t('admin.form.descriptionPlaceholder')"
          />
        </div>

        <!-- Time slots -->
        <div>
          <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.75rem;">
            <label class="field" style="gap: 0;">
              <span style="font-size: 0.8rem; font-weight: 600; letter-spacing: 0.06em; text-transform: uppercase; color: var(--ink-muted);">
                {{ t('admin.timeSlots') }}
              </span>
            </label>
            <button type="button" class="btn btn-ghost" style="padding: 0.4rem 0.9rem; font-size: 0.85rem;" @click="addSlot">
              {{ t('admin.addSlot') }}
            </button>
          </div>

          <div style="display: flex; flex-direction: column; gap: 0.6rem;">
            <div
              v-for="(slot, i) in form.slots"
              :key="slot.id"
              class="slot-row"
            >
              <span class="slot-index text-muted text-sm">{{ i + 1 }}</span>
            <div class="field" style="flex: 1; gap: 0.25rem;">
                <label :for="`slot-start-${slot.id}`" class="sr-only">{{ t('admin.form.startLabel') }}</label>
                <VueDatePicker
                  :id="`slot-start-${slot.id}`"
                  v-model="slot.starts_at"
                  :locale="pickerLocale"
                  :format="pickerFormat"
                  :enable-time-picker="true"
                  :minutes-increment="30"
                  :is-24="true"
                  :start-time="pickerStartTime"
                  :clearable="false"
                  @update:model-value="slot.starts_at = snapToHalfHour(slot.starts_at)"
                />
              </div>
              <span class="text-muted text-sm" style="padding: 0 0.25rem;">→</span>
            <div class="field" style="flex: 1; gap: 0.25rem;">
                <label :for="`slot-end-${slot.id}`" class="sr-only">{{ t('admin.form.endLabel') }}</label>
                <VueDatePicker
                  :id="`slot-end-${slot.id}`"
                  v-model="slot.ends_at"
                  :locale="pickerLocale"
                  :format="pickerFormat"
                  :enable-time-picker="true"
                  :minutes-increment="30"
                  :is-24="true"
                  :start-time="pickerStartTime"
                  :clearable="false"
                  @update:model-value="slot.ends_at = snapToHalfHour(slot.ends_at)"
                />
              </div>
              <button
                v-if="form.slots.length > 1"
                type="button"
                class="btn btn-danger"
                style="padding: 0.4rem 0.6rem; font-size: 0.8rem;"
                @click="removeSlot(i)"
                :aria-label="`Remove slot ${i + 1}`"
              >✕</button>
            </div>
          </div>

          <p v-if="slotError" class="text-sm" style="color: var(--no); margin-top: 0.5rem;">
            {{ slotError }}
          </p>
        </div>

        <div style="display: flex; justify-content: flex-end; padding-top: 0.5rem;">
          <button type="submit" class="btn btn-primary" :disabled="submitting">
            {{ submitting ? t('admin.creating') : t('admin.createPoll') }}
          </button>
        </div>

        <p v-if="error" class="text-sm" style="color: var(--no); text-align: right;">
          {{ error }}
        </p>
      </div>
    </form>

    <!-- Share link shown after creation -->
    <transition name="fade">
      <div v-if="createdId" class="card" style="margin-top: 1.5rem;">
        <p class="text-sm text-muted" style="margin-bottom: 0.5rem;">
          {{ t('admin.shareTitle') }}
        </p>
        <div style="display: flex; gap: 0.75rem; align-items: center;">
          <code style="flex: 1; background: var(--paper-2); padding: 0.6rem 0.9rem; border-radius: var(--radius); font-size: 0.9rem; word-break: break-all;">
            {{ shareUrl }}
          </code>
          <button class="btn btn-ghost" style="white-space: nowrap;" @click="copyLink">
            {{ copied ? t('admin.copied') : t('admin.copyLink') }}
          </button>
        </div>
      </div>
    </transition>

    <div class="card" style="margin-top: 1.5rem;">
      <div style="display: flex; align-items: center; justify-content: space-between; gap: 1rem;">
        <div>
          <h2 style="margin: 0;">{{ t('admin.allPolls') }}</h2>
          <p class="text-muted text-sm" style="margin: 0.4rem 0 0;">
            {{ t('admin.allPollsSubtitle') }}
          </p>
        </div>
        <button
          type="button"
          class="btn btn-ghost"
          style="white-space: nowrap;"
          :disabled="eventsLoading"
          @click="loadEvents"
        >
          {{ eventsLoading ? t('admin.refreshing') : t('admin.refresh') }}
        </button>
      </div>

      <div style="margin-top: 1rem;">
        <p v-if="eventsLoading && !events.length" class="text-muted text-sm">
          {{ t('admin.loadingEvents') }}
        </p>
        <p v-else-if="eventsError" class="text-sm" style="color: var(--no);">
          {{ eventsError }}
        </p>
        <p v-else-if="!events.length" class="text-muted text-sm">
          {{ t('admin.noPolls') }}
        </p>
        <ul v-else class="text-sm" style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 0.85rem;">
          <li
            v-for="event in events"
            :key="event.id"
            style="display: flex; flex-direction: column; gap: 0.25rem;"
          >
            <div style="display: flex; align-items: center; justify-content: space-between; gap: 0.75rem;">
              <a :href="`/poll/${event.id}`" style="font-weight: 600;">
                {{ event.title || t('common.untitled') }}
              </a>
              <button
                type="button"
                class="btn btn-danger"
                style="padding: 0.3rem 0.6rem; font-size: 0.75rem;"
                :disabled="deletingId"
                @click="deleteEvent(event)"
                :aria-label="deletingId === event.id ? t('admin.deleting') : t('admin.delete')"
                :title="deletingId === event.id ? t('admin.deleting') : t('admin.delete')"
              >
                {{ deletingId === event.id ? t('admin.deleting') : '🗑' }}
              </button>
            </div>
            <span class="text-muted" style="font-size: 0.8rem;">
              {{ t('admin.createdAt', { date: formatDate(event.created_at) }) }}
            </span>
            <span v-if="event.description" class="text-muted">
              {{ event.description }}
            </span>
          </li>
        </ul>
      </div>
    </div>

    <div style="height: 4rem;" />
  </div>
</template>

<script setup>
import { reactive, ref, computed, onMounted, inject } from 'vue'
import { useRouter } from 'vue-router'
import VueDatePicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'
import { useI18n } from 'vue-i18n'
import { api } from '../api'

const { t, locale } = useI18n()
const router = useRouter()
const authState = inject('authState', null)

const createSlotId = () =>
  (crypto?.randomUUID ? crypto.randomUUID() : `slot-${Date.now()}-${Math.random().toString(16).slice(2)}`)

const form = reactive({
  title: '',
  description: '',
  slots: [{ id: createSlotId(), starts_at: null, ends_at: null }],
})

const submitting = ref(false)
const error      = ref(null)
const slotError  = ref(null)
const createdId  = ref(null)
const copied     = ref(false)
const events     = ref([])
const eventsLoading = ref(false)
const eventsError   = ref(null)
const deletingId    = ref(null)

const homeTarget = computed(() => (authState?.isAuthed?.value ? '/admin' : '/'))
const shareUrl = computed(() =>
  createdId.value ? `${window.location.origin}/poll/${createdId.value}` : ''
)

const dateLocale = computed(() => (locale.value === 'sv' ? 'sv-SE' : 'en-GB'))
const pickerLocale = computed(() => (locale.value === 'sv' ? 'sv' : 'en-GB'))
const pickerFormat = 'yyyy-MM-dd HH:mm'
const pickerStartTime = { hours: 0, minutes: 0 }

const getAdminToken = () => localStorage.getItem('adminToken') || ''
const clearAdminSession = () => {
  localStorage.removeItem('adminToken')
  localStorage.removeItem('adminName')
}

function handleAuthError(e) {
  if (e && (e.status === 401 || e.status === 403)) {
    clearAdminSession()
    router.push('/')
    return true
  }
  return false
}

const snapToHalfHour = (value) => {
  if (!value) return value
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return value
  const minutes = date.getMinutes()
  const snapped = minutes < 15 ? 0 : minutes < 45 ? 30 : 60
  if (snapped === 60) {
    date.setHours(date.getHours() + 1)
    date.setMinutes(0, 0, 0)
  } else {
    date.setMinutes(snapped, 0, 0)
  }
  return date
}

function addSlot() {
  form.slots.push({ id: createSlotId(), starts_at: null, ends_at: null })
}

function removeSlot(i) {
  form.slots.splice(i, 1)
}

function validate() {
  slotError.value = null
  error.value = null

  for (const slot of form.slots) {
    const start = slot.starts_at ? new Date(slot.starts_at) : null
    const end = slot.ends_at ? new Date(slot.ends_at) : null
    if (!start || !end || Number.isNaN(start.getTime()) || Number.isNaN(end.getTime())) {
      slotError.value = t('admin.errors.slotMissing')
      return false
    }
    if (end <= start) {
      slotError.value = t('admin.errors.slotOrder')
      return false
    }
  }

  return true
}

async function submit() {
  if (!validate()) return

  submitting.value = true
  error.value = null

  try {
    const payload = {
      title: form.title.trim(),
      description: form.description.trim() || null,
      time_slots: form.slots.map(s => ({
        starts_at: new Date(s.starts_at).toISOString(),
        ends_at:   new Date(s.ends_at).toISOString(),
      })),
    }

    const { id } = await api.createPoll(payload)
    createdId.value = id
    await loadEvents()
  } catch (e) {
    if (!handleAuthError(e)) {
      error.value = e.message
    }
  } finally {
    submitting.value = false
  }
}

async function copyLink() {
  await navigator.clipboard.writeText(shareUrl.value)
  copied.value = true
  setTimeout(() => { copied.value = false }, 2000)
}

async function deleteEvent(event) {
  if (deletingId.value) return
  if (!confirm(t('admin.deleteConfirm', { title: event.title || t('common.untitled') }))) {
    return
  }

  deletingId.value = event.id
  eventsError.value = null

  try {
    await api.deletePoll(event.id)
    events.value = events.value.filter(item => item.id !== event.id)
  } catch (e) {
    if (!handleAuthError(e)) {
      eventsError.value = e.message
    }
  } finally {
    deletingId.value = null
  }
}

function formatDate(value) {
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return value
  return new Intl.DateTimeFormat(dateLocale.value, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  }).format(date)
}

async function loadEvents() {
  eventsLoading.value = true
  eventsError.value = null
  try {
    events.value = await api.listEvents()
  } catch (e) {
    if (!handleAuthError(e)) {
      eventsError.value = e.message
    }
  } finally {
    eventsLoading.value = false
  }
}

onMounted(() => {
  if (!getAdminToken()) {
    router.push('/')
    return
  }
  loadEvents()
})
</script>

<style scoped>
.slot-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex-wrap: wrap;
  padding: 0.75rem;
  border: 1px solid var(--paper-2);
  border-radius: var(--radius);
  background: var(--paper);
}

.slot-index {
  min-width: 1.6rem;
  height: 1.6rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  flex-shrink: 0;
  border-radius: 999px;
  background: #fff;
  border: 1px solid var(--paper-2);
  font-weight: 600;
}

.slot-row .field {
  flex: 1 1 12rem;
  min-width: 10rem;
}

:deep(.dp__input) {
  width: 100%;
  min-width: 0;
  border-radius: var(--radius);
  border-color: var(--paper-2);
  font-size: 1rem;
}

:deep(.dp__input_focus) {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-dim);
}

:deep(.dp__input_wrap) {
  position: relative;
}

:deep(.dp__input_icon) {
  display: none !important;
}

:deep(.dp__input_wrap .dp__input_icon_pad),
:deep(.dp__input_wrap .dp__input) {
  padding-right: 3.4rem !important;
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32' fill='%236b6b6b'><path d='M29.333 8c0-2.208-1.792-4-4-4h-18.667c-2.208 0-4 1.792-4 4v18.667c0 2.208 1.792 4 4 4h18.667c2.208 0 4-1.792 4-4v-18.667zM26.667 8v18.667c0 0.736-0.597 1.333-1.333 1.333 0 0-18.667 0-18.667 0-0.736 0-1.333-0.597-1.333-1.333 0 0 0-18.667 0-18.667 0-0.736 0.597-1.333 1.333-1.333 0 0 18.667 0 18.667 0 0.736 0 1.333 0.597 1.333 1.333z'/><path d='M20 2.667v5.333c0 0.736 0.597 1.333 1.333 1.333s1.333-0.597 1.333-1.333v-5.333c0-0.736-0.597-1.333-1.333-1.333s-1.333 0.597-1.333 1.333z'/><path d='M9.333 2.667v5.333c0 0.736 0.597 1.333 1.333 1.333s1.333-0.597 1.333-1.333v-5.333c0-0.736-0.597-1.333-1.333-1.333s-1.333 0.597-1.333 1.333z'/><path d='M4 14.667h24c0.736 0 1.333-0.597 1.333-1.333s-0.597-1.333-1.333-1.333h-24c-0.736 0-1.333 0.597-1.333 1.333s0.597 1.333 1.333 1.333z'/></svg>");
  background-repeat: no-repeat;
  background-position: right 0.85rem center;
  background-size: 16px 16px;
}

@media (max-width: 600px) {
  .slot-row {
    flex-direction: column;
    align-items: stretch;
    gap: 0.4rem;
  }

  .slot-index {
    align-self: flex-start;
  }

  .slot-row .text-sm {
    align-self: center;
  }
}
</style>
