<template>
  <div>
    <input v-model="name" placeholder="Enter your name" />
    <p v-if="name">Hello {{ name }}!</p>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const name = ref('');
</script>
