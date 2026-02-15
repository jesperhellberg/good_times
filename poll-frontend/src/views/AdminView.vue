<template>
  <div class="container">
    <header class="page-header">
      <p class="eyebrow">{{ t('landing.eyebrow') }}</p>
      <h1>{{ t('admin.title') }}</h1>
      <p class="text-muted" style="margin-top: 0.5rem">
        {{ t('admin.subtitle') }}
      </p>
    </header>

    <div class="card" style="margin-bottom: 1.5rem;">
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
              :key="i"
              class="slot-row"
            >
              <span class="slot-index text-muted text-sm">{{ i + 1 }}</span>
            <div class="field" style="flex: 1; gap: 0.25rem;">
                <label :for="`slot-start-${i}`" class="sr-only">{{ t('admin.form.startLabel') }}</label>
                <FlatPickr
                  :id="`slot-start-${i}`"
                  v-model="slot.starts_at"
                  :config="pickerConfig"
                />
              </div>
              <span class="text-muted text-sm" style="padding: 0 0.25rem;">→</span>
            <div class="field" style="flex: 1; gap: 0.25rem;">
                <label :for="`slot-end-${i}`" class="sr-only">{{ t('admin.form.endLabel') }}</label>
                <FlatPickr
                  :id="`slot-end-${i}`"
                  v-model="slot.ends_at"
                  :config="pickerConfig"
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

    <div style="height: 4rem;" />
  </div>
</template>

<script setup>
import { reactive, ref, computed, onMounted } from 'vue'
import FlatPickr from 'vue-flatpickr-component'
import 'flatpickr/dist/flatpickr.css'
import { useI18n } from 'vue-i18n'
import { api } from '../api'

const { t, locale } = useI18n()

const form = reactive({
  title: '',
  description: '',
  slots: [{ starts_at: '', ends_at: '' }],
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

const shareUrl = computed(() =>
  createdId.value ? `${window.location.origin}/poll/${createdId.value}` : ''
)

const dateLocale = computed(() => (locale.value === 'sv' ? 'sv-SE' : 'en-GB'))

const pickerConfig = {
  enableTime: true,
  time_24hr: true,
  minuteIncrement: 30,
  dateFormat: "Y-m-d\\TH:i",
  altInput: true,
  altFormat: "Y-m-d H:i",
  altInputClass: "alt-datetime-input",
}

function addSlot() {
  form.slots.push({ starts_at: '', ends_at: '' })
}

function removeSlot(i) {
  form.slots.splice(i, 1)
}

function validate() {
  slotError.value = null
  error.value = null

  for (const slot of form.slots) {
    if (!slot.starts_at || !slot.ends_at) {
      slotError.value = t('admin.errors.slotMissing')
      return false
    }
    if (slot.ends_at <= slot.starts_at) {
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
    error.value = e.message
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
    eventsError.value = e.message
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
    eventsError.value = e.message
  } finally {
    eventsLoading.value = false
  }
}

onMounted(loadEvents)
</script>

<style scoped>
.slot-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.slot-index {
  width: 1.25rem;
  text-align: center;
  flex-shrink: 0;
}

:deep(.alt-datetime-input) {
  padding-right: 2.2rem;
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='18' height='18' viewBox='0 0 24 24' fill='none' stroke='%236b6b6b' stroke-width='1.6' stroke-linecap='round' stroke-linejoin='round'><rect x='3' y='4' width='18' height='18' rx='2' ry='2'/><line x1='16' y1='2.5' x2='16' y2='6.5'/><line x1='8' y1='2.5' x2='8' y2='6.5'/><line x1='3' y1='10' x2='21' y2='10'/></svg>");
  background-repeat: no-repeat;
  background-position: right 0.7rem center;
  background-size: 16px 16px;
}
</style>
