<script lang="ts" setup>
import {writeText} from "@tauri-apps/api/clipboard";
import {open} from "@tauri-apps/api/dialog";
import {listen} from "@tauri-apps/api/event";
import {appDataDir, join, resolveResource} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api/tauri";
import FightTable from "../components/FightTable.vue";
import SvgIcon from "../components/SvgIcon.vue";

// load meta data
const meta = ref();


// logs reader: log -> fights
const log_path = ref("");
const fights = ref();

async function selectLogFile() {
  const selected = await open({
    multiple: false,
  });
  if (!selected) {
    // errorMsg.value = "No file selected";
    return;
  }
  log_path.value = typeof selected === "string" ? selected : selected[0];
  fights.value = await invoke("load_logs", {path: selected, meta: meta.value});
  exportProgress.value = -1;
}


// mark useful fights
function setUseful(idx: number, status: boolean) {
  // eslint-disable-next-line security/detect-object-injection
  fights.value[idx].useful = status;
}

const usefulFlightsNum = computed(() => {
  if (fights) {
    const num = fights.value.filter((fight) => fight.useful).length;
    console.log(num);
    if (!num) {
      exportMode.value = false;
    }
    return num;
  }
  exportMode.value = false;
  return 0;
});


// export fights -> notion, json, etc.
const config = ref();
const exportMode = ref(false);
const exportProgress = ref(-1);
const jsonExportAnimation = ref(false);

async function exportFights(mode: string) {
  if (mode === "notion") {
    // read notion config
    const appDataDirPath = await appDataDir();
    const configPath = await join(appDataDirPath, "ceres");
    config.value = await invoke("read_config", {cfgDirPath: configPath});
    // notion export
    exportProgress.value = 0;
    await invoke("to_notion", {fights: fights.value, notionConfig: config.value.notion});
  } else if (mode === "json") {
    exportProgress.value = 0;
    const json: string = await invoke("to_json", {fights: fights.value});
    await writeText(json);
    // animation
    jsonExportAnimation.value = true;
    setTimeout(() => {
      jsonExportAnimation.value = false;
    }, 4000);
  }
}

const progressNumber = computed(() => {
  return exportProgress.value / usefulFlightsNum.value * 100;
});
const exporting = computed(() => {
  return exportProgress.value !== -1 && exportProgress.value !== usefulFlightsNum.value;
});


// load meta and register event when mounted
onMounted(async () => {
  // load instances & jobs
  const instancesJsonPath = await resolveResource("resources/instances.json");
  const jobsJsonPath = await resolveResource("resources/jobs.json");
  meta.value = await invoke("load_meta", {instancesPath: instancesJsonPath, jobsPath: jobsJsonPath});
  // register event
  await listen("export-progress", (event) => {
    exportProgress.value = event.payload as number;
  });
});

</script>

<template>
  <div :class="{ 'blur-lg': exporting, 'pointer-events-none': exporting }" class="w-auto flex flex-col space-y-2 pb-4">
    <div class="flex space-x-2">
      <button class="w-2/12 h-12 btn btn-primary" @click="selectLogFile">选择日志</button>
      <div
          v-if="log_path"
          class="alert flex whitespace-nowrap h-12 w-auto overflow-x-scroll justify-self-center items-center">
        <span class="text-sm font-mono">{{ log_path }}</span>
      </div>
    </div>

    <div v-if="fights && fights.length !== 0" class="w-auto flex flex-col space-y-2">
      <div class="alert bg-secondary flex justify-self-center items-center">
        <SvgIcon class="h-5 w-5" fill="none" icon-name="bulb" viewBox="0 0 24 24"/>
        <span>请选择需要导出的数据</span>
      </div>
      <FightTable
          v-for="(fight, f_idx) in fights" :key="fight" :fight="fight" :idx="f_idx as number"
          @emit-set-useful="setUseful"/>
      <div class="flex space-x-2">
        <button
            :disabled="!usefulFlightsNum" class="w-2/12 h-12 btn btn-primary"
            @click="exportMode = !exportMode">导出
        </button>
        <button v-if="exportMode" class="w-2/12 h-12 btn btn-base-200" @click="exportFights('notion')">Notion</button>
        <button
            v-if="exportMode" :class="{ 'btn-success': jsonExportAnimation, 'w-3/12': jsonExportAnimation }"
            class="w-2/12 h-12 btn btn-base-200"
            @click="exportFights('json')">{{
            jsonExportAnimation ? "已复制到剪贴板" : "Json"
          }}
        </button>
      </div>
    </div>

    <div
        v-if="fights && fights.length === 0"
        class="alert bg-base-100 flex whitespace-nowrap h-12 w-auto overflow-x-scroll justify-self-center items-center">
      <SvgIcon class="h-5 w-5" fill="none" icon-name="slash" viewBox="0 0 24 24"/>
      <span class="text-sm">无可用战斗数据</span>
    </div>
  </div>

  <div v-if="exporting" class="toast toast-center toast-middle">
    <div class="flex flex-col space-y-3 justify-center items-center">
      <span class="text-lg text-mono font-bold">{{ progressNumber.toFixed(1) + "%" }}</span>
      <progress :max="usefulFlightsNum" :value="exportProgress" class="progress w-56"/>
    </div>
  </div>
</template>
