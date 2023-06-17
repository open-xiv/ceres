<script lang="ts" setup>
import SvgIcon from "../components/SvgIcon.vue";
</script>

<template>
  <div class="w-auto flex flex-col space-y-2 pb-4">
    <div class="alert alert-warning w-auto">
      <SvgIcon class="h-5 w-5" fill="none" icon-name="git" viewBox="0 0 24 24"/>
      <span class="text-sm"> 这是一个正在开发的页面 </span>
      <span class="text-sm font-mono"> 预期完成时间 06.20 </span>
    </div>
  </div>
</template>
