const BASE = '/api'

function getAdminToken() {
  return localStorage.getItem('adminToken') || ''
}

async function request(method, path, body) {
  const res = await fetch(`${BASE}${path}`, {
    method,
    headers: body ? { 'Content-Type': 'application/json' } : {},
    body: body ? JSON.stringify(body) : undefined,
  })

  if (!res.ok) {
    const text = await res.text().catch(() => res.statusText)
    const err = new Error(`${res.status}: ${text}`)
    err.status = res.status
    err.body = text
    throw err
  }

  return res.json()
}

async function adminRequest(method, path, body) {
  const token = getAdminToken()
  const headers = body ? { 'Content-Type': 'application/json' } : {}
  if (token) headers['X-Admin-Token'] = token

  const res = await fetch(`${BASE}${path}`, {
    method,
    headers,
    body: body ? JSON.stringify(body) : undefined,
  })

  if (!res.ok) {
    const text = await res.text().catch(() => res.statusText)
    const err = new Error(`${res.status}: ${text}`)
    err.status = res.status
    err.body = text
    throw err
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
    return adminRequest('POST', '/poll', payload)
  },

  /** List all events (admin) */
  listEvents() {
    return adminRequest('GET', '/events')
  },

  /** Delete a poll (admin). Returns { id } */
  deletePoll(id) {
    return adminRequest('DELETE', `/poll/${id}`)
  },

  /** Submit votes for a participant. Returns { participant_id } */
  submitVote(pollId, payload) {
    return request('POST', `/poll/${pollId}/vote`, payload)
  },

  /** Update votes for a participant. Returns { participant_id } */
  updateVotes(pollId, participantId, payload) {
    return request('PUT', `/poll/${pollId}/participant/${participantId}`, payload)
  },

  /** Admin signup. Returns { token, admin_id, name } */
  signupAdmin(payload) {
    return request('POST', '/admin/signup', payload)
  },

  /** Admin login. Returns { token, admin_id, name } */
  loginAdmin(payload) {
    return request('POST', '/admin/login', payload)
  },

  /** Admin logout. Returns { ok } */
  logoutAdmin() {
    return adminRequest('POST', '/admin/logout')
  },
}
