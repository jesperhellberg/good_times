<template>
  <div class="container" style="min-height: 70vh; display: flex; align-items: center;">
    <header class="page-header">
      <p class="eyebrow">{{ t('landing.eyebrow') }}</p>
      <h1 style="margin-top: 0.5rem;">{{ t('landing.title') }}</h1>
      <p class="text-muted" style="margin-top: 0.75rem; max-width: 38rem;">
        {{ t('landing.blurb1') }}
      </p>
      <p class="text-muted" style="margin-top: 0.75rem; max-width: 38rem;">
        {{ t('landing.blurb2') }}
      </p>
    </header>
  </div>
</template>

<script setup>
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>
