<script lang="ts" setup>
import {appDataDir, join} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api/tauri";
import SvgIcon from "../components/SvgIcon.vue";

// config
const config = ref();

// config lock
const editable = ref(false);

async function setEditable(status: boolean) {
  editable.value = status;
  // enter edit mode
  if (status) {
  }
  // leave edit mode
  else {
    // save config
    const appDataDirPath = await appDataDir();
    const configPath = await join(appDataDirPath, "ceres");
    await invoke("write_config", {cfgDirPath: configPath, config: config.value});
  }
}


// change theme
const themes = [["蛋糕", "cupcake"], ["蜡笔", "pastel"], ["夜晚", "dracula"]];

async function changeTheme(newTheme: string) {
  config.value.theme = newTheme;
  // save config
  const appDataDirPath = await appDataDir();
  const configPath = await join(appDataDirPath, "ceres");
  await invoke("write_config", {cfgDirPath: configPath, config: config.value});
}

// load config when mounted
onBeforeMount(async () => {
  // load config
  const appDataDirPath = await appDataDir();
  const configPath = await join(appDataDirPath, "ceres");
  config.value = await invoke("read_config", {cfgDirPath: configPath});
});
</script>

<template>
  <div class="w-auto flex flex-col space-y-2 pb-4">
    <div
        class="alert alert-warning flex whitespace-nowrap h-fit w-auto overflow-x-scroll justify-self-center items-center">
      <SvgIcon class="h-5 w-5" fill="none" icon-name="alert" viewBox="0 0 24 24"/>
      <div class="form-control w-52">
        <label class="cursor-pointer flex space-x-3 justify-self-center items-center">
          <span class="label-text font-bold text-warning-content">配置锁</span>
          <input
              :checked="!editable" class="toggle toggle-secondary" type="checkbox"
              @click="setEditable(!editable);"/>
          <span class="label-text font-bold text-warning-content">{{
              editable ? "编辑中 【关闭编辑模式以保存】" : "锁定中"
            }}</span>
        </label>
      </div>
    </div>

    <div class="w-auto bg-base-200 rounded-box">
      <div v-if="config" class="flex flex-col font-mono m-6 space-y-4">
        <p class="text-base">Notion</p>
        <div class="flex flex-col font-mono space-y-2">
          <div class="flex space-x-4 items-center">
            <p class="text-sm">- Block</p>
            <input
                v-model="config.notion.block_id" :disabled="!editable"
                class="input input-bordered w-full h-8 max-w-xs text-sm font-mono"
                placeholder="Find it in notion url."
                type="text"/>
          </div>
          <div class="flex space-x-4 items-center">
            <p class="text-sm">- Token</p>
            <input
                v-model="config.notion.token" :disabled="!editable"
                class="input input-bordered w-full h-8 max-w-lg text-sm font-mono"
                placeholder="Find it in notion url."
                type="text"/>
          </div>
        </div>
      </div>
    </div>

    <div class="w-auto bg-base-200 rounded-box">
      <div v-if="config" class="flex flex-col font-mono m-6 space-y-4">
        <div class="flex space-x-2 items-center">
          <p class="text-base">酥卷</p>
          <span class="badge badge-sm badge-warning">DEV</span>
        </div>
        <div class="flex flex-col font-mono space-y-2">
          <div class="flex space-x-4 items-center">
            <p class="text-sm">- Token</p>
            <input
                v-model="config.su.token" :disabled="!editable"
                class="input input-bordered w-full h-8 max-w-sm text-sm font-mono"
                placeholder="Find it in notion url."
                type="text"/>
          </div>
        </div>
      </div>
    </div>

    <div class="w-auto bg-base-200 rounded-box">
      <div v-if="config" class="flex flex-col font-mono m-6 space-y-4">
        <p class="text-base">主题</p>
        <p class="text-sm">- AutoHook 根据系统主题自动变化</p>
        <!--        <ul class="menu menu-horizontal bg-base-100 rounded-box w-fit space-x-2">-->
        <!--          <li v-for="theme in themes" :key="theme[1]">-->
        <!--            <a-->
        <!--                :class="{active: config.theme === theme[1] }"-->
        <!--                @click="changeTheme(theme[1])">{{ theme[0] }}</a>-->
        <!--          </li>-->
        <!--        </ul>-->
      </div>
    </div>
  </div>
</template>
