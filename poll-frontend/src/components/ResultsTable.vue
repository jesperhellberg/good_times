<template>
  <div class="results-wrapper">
    <table class="results-table">
      <thead>
        <tr>
          <th class="name-col"></th>
          <th
            v-for="slot in poll.time_slots"
            :key="slot.id"
            class="slot-col"
            :class="{ 'is-best': slot.id === bestSlotId }"
          >
            <div class="slot-header">
              <span class="slot-date">{{ slotDate(slot) }}</span>
              <span class="slot-time">{{ slotTime(slot) }}</span>
              <span class="slot-count">
                {{ slot.available_count }}/{{ poll.participants.length }}
              </span>
            </div>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="participant in poll.participants" :key="participant.id">
          <td class="name-col">{{ participant.name }}</td>
          <td
            v-for="slot in poll.time_slots"
            :key="slot.id"
            class="vote-cell"
            :class="cellClass(participant, slot)"
          >
            <span class="cell-icon">
              {{ cellIcon(participant, slot) }}
            </span>
          </td>
        </tr>
        <tr v-if="poll.participants.length === 0">
          <td :colspan="poll.time_slots.length + 1" class="empty-row">
            No responses yet — be the first!
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  poll: { type: Object, required: true },
})

const bestSlotId = computed(() => {
  if (!props.poll.time_slots.length) return null
  return [...props.poll.time_slots]
    .sort((a, b) => b.available_count - a.available_count)[0].id
})

function getVote(participant, slot) {
  return participant.votes.find(v => v.time_slot_id === slot.id)
}

function cellClass(participant, slot) {
  const vote = getVote(participant, slot)
  if (!vote) return 'cell-unknown'
  return vote.available ? 'cell-yes' : 'cell-no'
}

function cellIcon(participant, slot) {
  const vote = getVote(participant, slot)
  if (!vote) return '–'
  return vote.available ? '✓' : '✕'
}

function slotDate(slot) {
  return new Date(slot.starts_at).toLocaleDateString('en-GB', {
    weekday: 'short', day: 'numeric', month: 'short',
  })
}

function slotTime(slot) {
  const from = new Date(slot.starts_at).toLocaleTimeString('en-GB', { hour: '2-digit', minute: '2-digit' })
  const to   = new Date(slot.ends_at).toLocaleTimeString('en-GB',   { hour: '2-digit', minute: '2-digit' })
  return `${from}–${to}`
}
</script>

<style scoped>
.results-wrapper {
  overflow-x: auto;
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow);
}

.results-table {
  width: 100%;
  border-collapse: collapse;
  background: #fff;
  border-radius: var(--radius-lg);
  overflow: hidden;
  font-size: 0.9rem;
}

.results-table th,
.results-table td {
  padding: 0.65rem 0.9rem;
  text-align: center;
  border-bottom: 1px solid var(--paper-2);
}

.results-table tbody tr:last-child td {
  border-bottom: none;
}

/* Name column — left aligned, sticky */
.name-col {
  text-align: left !important;
  white-space: nowrap;
  font-weight: 500;
  color: var(--ink);
  position: sticky;
  left: 0;
  background: #fff;
  z-index: 1;
  min-width: 100px;
}

thead .name-col {
  background: var(--paper);
}

/* Slot header column */
.slot-col {
  background: var(--paper);
  min-width: 110px;
}

.slot-col.is-best {
  background: var(--yes-bg);
}

.slot-header {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
  align-items: center;
}

.slot-date {
  font-weight: 600;
  color: var(--ink);
  font-size: 0.8rem;
}

.slot-time {
  font-size: 0.75rem;
  color: var(--ink-muted);
}

.slot-count {
  font-size: 0.75rem;
  font-weight: 700;
  color: var(--yes);
  margin-top: 0.1rem;
}

.is-best .slot-count {
  color: var(--yes);
}

/* Vote cells */
.vote-cell { font-size: 0.85rem; }

.cell-yes     { background: var(--yes-bg); color: var(--yes);     font-weight: 700; }
.cell-no      { background: var(--no-bg);  color: var(--no);      font-weight: 700; }
.cell-unknown { color: var(--ink-faint); }

.cell-icon {
  display: inline-block;
  width: 1.25rem;
  text-align: center;
}

.empty-row {
  text-align: center !important;
  color: var(--ink-muted);
  font-style: italic;
  padding: 1.5rem !important;
}
</style>
