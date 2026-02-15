<template>
  <div class="container">

    <!-- Loading -->
    <div v-if="loading" style="padding: 5rem 0; text-align: center; color: var(--ink-muted);">
      Loading poll…
    </div>

    <!-- Error -->
    <div v-else-if="fetchError" style="padding: 5rem 0; text-align: center;">
      <p style="color: var(--no); margin-bottom: 1rem;">{{ fetchError }}</p>
      <button class="btn btn-ghost" @click="loadPoll">Try again</button>
    </div>

    <template v-else-if="poll">
      <header class="page-header">
        <p class="eyebrow">Good Times</p>
        <h1>{{ poll.title }}</h1>
        <p v-if="poll.description" class="text-muted" style="margin-top: 0.5rem;">
          {{ poll.description }}
        </p>
      </header>

      <!-- Results table -->
      <section style="margin-bottom: 2.5rem;">
        <h2 style="margin-bottom: 1.25rem;">Availability</h2>
        <ResultsTable :poll="poll" />
      </section>

      <!-- Vote form -->
      <section class="card">
        <template v-if="!submitted">
          <h2 style="margin-bottom: 0.25rem;">Add your availability</h2>
          <p class="text-muted text-sm" style="margin-bottom: 1.5rem;">
            Mark each slot as available or not, then submit.
          </p>

          <div class="field" style="margin-bottom: 1.5rem; max-width: 280px;">
            <label for="participant-name">Your name</label>
            <input
              id="participant-name"
              v-model="name"
              type="text"
              placeholder="e.g. Alice"
              autofocus
            />
          </div>

          <div style="display: flex; flex-direction: column; gap: 0.5rem; margin-bottom: 1.75rem;">
            <div
              v-for="slot in poll.time_slots"
              :key="slot.id"
              class="vote-row"
              :class="{ 'is-yes': votes[slot.id] === true, 'is-no': votes[slot.id] === false }"
              @click="toggleVote(slot.id)"
              role="checkbox"
              :aria-checked="votes[slot.id] === true"
              tabindex="0"
              @keydown.space.prevent="toggleVote(slot.id)"
              @keydown.enter.prevent="toggleVote(slot.id)"
            >
              <span class="vote-icon">
                <template v-if="votes[slot.id] === true">✓</template>
                <template v-else-if="votes[slot.id] === false">✕</template>
                <template v-else>–</template>
              </span>
              <span class="vote-label">{{ formatSlot(slot) }}</span>
            </div>
          </div>

          <div style="display: flex; align-items: center; justify-content: flex-end; gap: 1rem;">
            <p v-if="voteError" class="text-sm" style="color: var(--no);">{{ voteError }}</p>
            <button class="btn btn-primary" :disabled="submitting" @click="submitVotes">
              {{ submitting ? 'Submitting…' : 'Submit' }}
            </button>
          </div>
        </template>

        <template v-else>
          <div style="text-align: center; padding: 1rem 0;">
            <div style="font-size: 2rem; margin-bottom: 0.75rem;">🎉</div>
            <h3 style="margin-bottom: 0.4rem;">Thanks, {{ submittedName }}!</h3>
            <p class="text-muted text-sm">Your availability has been recorded.</p>
          </div>
        </template>
      </section>

      <div style="height: 4rem;" />
    </template>

  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { api } from '../api'
import ResultsTable from '../components/ResultsTable.vue'

const route = useRoute()
const pollId = route.params.id

const poll       = ref(null)
const loading    = ref(true)
const fetchError = ref(null)

const name        = ref('')
const votes       = reactive({})   // slot.id -> true | false | undefined
const submitting  = ref(false)
const voteError   = ref(null)
const submitted   = ref(false)
const submittedName = ref('')

async function loadPoll() {
  loading.value    = true
  fetchError.value = null
  try {
    poll.value = await api.getPoll(pollId)
    // Initialise vote map with undefined for each slot
    for (const slot of poll.value.time_slots) {
      if (!(slot.id in votes)) votes[slot.id] = undefined
    }
  } catch (e) {
    fetchError.value = e.message
  } finally {
    loading.value = false
  }
}

function toggleVote(slotId) {
  // Cycle: undefined → true → false → true
  votes[slotId] = votes[slotId] === true ? false : true
}

function formatSlot(slot) {
  const start = new Date(slot.starts_at)
  const end   = new Date(slot.ends_at)
  const date  = start.toLocaleDateString('en-GB', { weekday: 'short', day: 'numeric', month: 'short' })
  const from  = start.toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit' })
  const to    = end.toLocaleTimeString('en-GB',   { hour: '2-digit', minute: '2-digit' })
  return `${date}, ${from} – ${to}`
}

async function submitVotes() {
  voteError.value = null

  if (!name.value.trim()) {
    voteError.value = 'Please enter your name.'
    return
  }

  const unvoted = poll.value.time_slots.filter(s => votes[s.id] === undefined)
  if (unvoted.length > 0) {
    voteError.value = 'Please mark every slot before submitting.'
    return
  }

  submitting.value = true
  try {
    await api.submitVote(pollId, {
      participant_name: name.value.trim(),
      votes: poll.value.time_slots.map(s => ({
        time_slot_id: s.id,
        available:    votes[s.id],
      })),
    })

    submittedName.value = name.value.trim()
    submitted.value = true

    // Refresh results so the new votes show up in the table
    await loadPoll()
  } catch (e) {
    voteError.value = e.message
  } finally {
    submitting.value = false
  }
}

onMounted(loadPoll)
</script>

<style scoped>
.vote-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  border-radius: var(--radius);
  border: 1.5px solid var(--paper-2);
  cursor: pointer;
  user-select: none;
  transition: background var(--transition), border-color var(--transition);
}

.vote-row:hover { border-color: var(--ink-faint); background: var(--paper); }

.vote-row.is-yes {
  background: var(--yes-bg);
  border-color: var(--yes);
}

.vote-row.is-no {
  background: var(--no-bg);
  border-color: var(--no);
}

.vote-icon {
  width: 1.5rem;
  height: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: var(--paper-2);
  font-size: 0.8rem;
  font-weight: 700;
  flex-shrink: 0;
  transition: background var(--transition), color var(--transition);
}

.is-yes .vote-icon { background: var(--yes); color: #fff; }
.is-no  .vote-icon { background: var(--no);  color: #fff; }

.vote-label {
  font-size: 0.95rem;
}
</style>
