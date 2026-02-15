<template>
  <div class="container">
    <header class="page-header">
      <p class="eyebrow">Good Times</p>
      <h1>New poll</h1>
      <p class="text-muted" style="margin-top: 0.5rem">
        Set up a time slot poll and share the link with your group.
      </p>
    </header>

    <form @submit.prevent="submit" novalidate>
      <div class="card" style="display: flex; flex-direction: column; gap: 1.25rem;">

        <div class="field">
          <label for="title">Poll title</label>
          <input
            id="title"
            v-model="form.title"
            type="text"
            placeholder="e.g. Summer dinner"
            required
            autofocus
          />
        </div>

        <div class="field">
          <label for="description">Description <span class="text-muted">(optional)</span></label>
          <textarea
            id="description"
            v-model="form.description"
            placeholder="Any details the group should know…"
          />
        </div>

        <!-- Time slots -->
        <div>
          <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 0.75rem;">
            <label class="field" style="gap: 0;">
              <span style="font-size: 0.8rem; font-weight: 600; letter-spacing: 0.06em; text-transform: uppercase; color: var(--ink-muted);">
                Time slots
              </span>
            </label>
            <button type="button" class="btn btn-ghost" style="padding: 0.4rem 0.9rem; font-size: 0.85rem;" @click="addSlot">
              + Add slot
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
                <label :for="`slot-start-${i}`" class="sr-only">Start</label>
                <input
                  :id="`slot-start-${i}`"
                  v-model="slot.starts_at"
                  type="datetime-local"
                />
              </div>
              <span class="text-muted text-sm" style="padding: 0 0.25rem;">→</span>
              <div class="field" style="flex: 1; gap: 0.25rem;">
                <label :for="`slot-end-${i}`" class="sr-only">End</label>
                <input
                  :id="`slot-end-${i}`"
                  v-model="slot.ends_at"
                  type="datetime-local"
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
            {{ submitting ? 'Creating…' : 'Create poll' }}
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
        <p class="text-sm text-muted" style="margin-bottom: 0.5rem;">Poll created — share this link:</p>
        <div style="display: flex; gap: 0.75rem; align-items: center;">
          <code style="flex: 1; background: var(--paper-2); padding: 0.6rem 0.9rem; border-radius: var(--radius); font-size: 0.9rem; word-break: break-all;">
            {{ shareUrl }}
          </code>
          <button class="btn btn-ghost" style="white-space: nowrap;" @click="copyLink">
            {{ copied ? 'Copied!' : 'Copy link' }}
          </button>
        </div>
      </div>
    </transition>

    <div style="height: 4rem;" />
  </div>
</template>

<script setup>
import { reactive, ref, computed } from 'vue'
import { api } from '../api'

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

const shareUrl = computed(() =>
  createdId.value ? `${window.location.origin}/poll/${createdId.value}` : ''
)

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
      slotError.value = 'Please fill in all time slots.'
      return false
    }
    if (slot.ends_at <= slot.starts_at) {
      slotError.value = "Each slot's end time must be after its start time."
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
</style>
