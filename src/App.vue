<script lang="ts" setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { RouterView } from "vue-router";
import CeresMenu from "./components/CeresMenu.vue";
</script>

<template>
  <div class="m-4 flex flex-col space-y-4">
    <CeresMenu/>
    <RouterView/>
  </div>
</template>
