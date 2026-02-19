<template>
  <div class="locale-toggle">
    <button
      type="button"
      class="locale-btn"
      :class="{ active: locale === 'en' }"
      @click="setLocale('en')"
      aria-label="Switch to English"
    >
      🇬🇧
    </button>
    <button
      type="button"
      class="locale-btn"
      :class="{ active: locale === 'sv' }"
      @click="setLocale('sv')"
      aria-label="Byt till svenska"
    >
      🇸🇪
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
  display: inline-flex;
  gap: 0.35rem;
  padding: 0.25rem;
  background: #fff;
  border: 1px solid var(--paper-2);
  border-radius: 999px;
  box-shadow: var(--shadow-sm);
}

.locale-btn {
  border: none;
  background: transparent;
  color: var(--ink-muted);
  font-size: 1rem;
  line-height: 1;
  padding: 0.2rem 0.4rem;
  border-radius: 999px;
  cursor: pointer;
}

.locale-btn.active {
  background: var(--paper-2);
  color: var(--ink);
}

@media (max-width: 600px) {
  .locale-toggle {
    padding: 0.2rem;
  }

  .locale-btn {
    font-size: 0.65rem;
    padding: 0.25rem 0.4rem;
  }
}
</style>
