<script lang="ts" setup>
import {useRoute, useRouter} from "vue-router";
import SvgIcon from "./SvgIcon.vue";

const router = useRouter();

function switchView(page: string) {
  router.push({name: page});
}

const route = useRoute();
</script>

<template>
  <ul class="menu w-auto bg-base-200 menu-horizontal rounded-box">
    <li>
      <a :class="{ 'active': route.name === 'OfflineParser' }" @click="switchView('OfflineParser')">
        <SvgIcon class="h-5 w-5" fill="none" icon-name="atom" viewBox="0 0 24 24"/>
        离线解析
      </a>
    </li>
    <li>
      <a :class="{ 'active': route.name === 'OnlineParser' }" @click="switchView('OnlineParser')">
        <SvgIcon class="h-5 w-5" fill="none" icon-name="zap" viewBox="0 0 24 24"/>
        实时解析
        <span class="badge badge-sm badge-warning">DEV</span>
      </a>
    </li>
    <li>
      <a :class="{ 'active': route.name === 'BindInfo' }" @click="switchView('BindInfo')">
        <SvgIcon class="h-5 w-5" fill="none" icon-name="link" viewBox="0 0 24 24"/>
        绑定信息
      </a>
    </li>
  </ul>
</template>
