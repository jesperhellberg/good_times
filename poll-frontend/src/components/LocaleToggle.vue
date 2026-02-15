<template>
  <div class="locale-toggle">
    <button
      type="button"
      class="locale-btn"
      :class="{ active: locale === 'en' }"
      @click="setLocale('en')"
      aria-label="Switch to English"
    >
      EN
    </button>
    <button
      type="button"
      class="locale-btn"
      :class="{ active: locale === 'sv' }"
      @click="setLocale('sv')"
      aria-label="Byt till svenska"
    >
      SV
    </button>
  </div>
</template>

<script setup>
import { useI18n } from 'vue-i18n'

const { locale } = useI18n()

function setLocale(value) {
  locale.value = value
}
</script>

<style scoped>
.locale-toggle {
  position: fixed;
  top: 1.25rem;
  right: 1.5rem;
  display: inline-flex;
  gap: 0.35rem;
  padding: 0.25rem;
  background: #fff;
  border: 1px solid var(--paper-2);
  border-radius: 999px;
  box-shadow: var(--shadow-sm);
  z-index: 10;
}

.locale-btn {
  border: none;
  background: transparent;
  color: var(--ink-muted);
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  padding: 0.3rem 0.45rem;
  border-radius: 999px;
  cursor: pointer;
}

.locale-btn.active {
  background: var(--paper-2);
  color: var(--ink);
}
</style>
