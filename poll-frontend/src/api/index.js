const BASE = '/api'

async function request(method, path, body) {
  const res = await fetch(`${BASE}${path}`, {
    method,
    headers: body ? { 'Content-Type': 'application/json' } : {},
    body: body ? JSON.stringify(body) : undefined,
  })

  if (!res.ok) {
    const text = await res.text().catch(() => res.statusText)
    throw new Error(`${res.status}: ${text}`)
  }

  return res.json()
}

export const api = {
  /** Fetch a poll with all slots, participants, and votes */
  getPoll(id) {
    return request('GET', `/poll/${id}`)
  },

  /** Create a new poll. Returns { id } */
  createPoll(payload) {
    return request('POST', '/poll', payload)
  },

  /** List all events (admin) */
  listEvents() {
    return request('GET', '/events')
  },

  /** Delete a poll (admin). Returns { id } */
  deletePoll(id) {
    return request('DELETE', `/poll/${id}`)
  },

  /** Submit votes for a participant. Returns { participant_id } */
  submitVote(pollId, payload) {
    return request('POST', `/poll/${pollId}/vote`, payload)
  },

  /** Update votes for a participant. Returns { participant_id } */
  updateVotes(pollId, participantId, payload) {
    return request('PUT', `/poll/${pollId}/participant/${participantId}`, payload)
  },
}
