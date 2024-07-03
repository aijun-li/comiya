<script setup lang="ts">
import { Toaster } from '@/components/ui/sonner';
import { RouterView } from 'vue-router';
</script>

<template>
  <div class="h-screen w-screen flex-center">
    <RouterView />

    <Toaster position="top-center" :duration="1500" />
  </div>
</template>
