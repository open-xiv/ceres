<script lang="ts" setup>
import {writeText} from "@tauri-apps/api/clipboard";
import {open} from "@tauri-apps/api/dialog";
import {listen} from "@tauri-apps/api/event";
import {appDataDir, join, resolveResource, sep} from "@tauri-apps/api/path";
import {invoke} from "@tauri-apps/api/tauri";
import FightTable from "../components/FightTable.vue";
import SvgIcon from "../components/SvgIcon.vue";

// config
const config = ref<SuMentorConfig>();

// load meta data
const meta = ref<Meta>();

// logs reader: log -> fights
const fights = ref<FightRecord[]>([]);

async function updateLogFolder() {
  const selected = await open({
    directory: true,
  });
  if (!selected) {
    // errorMsg.value = "No file selected";
    return;
  }
  // update config value
  config.value?.theme && (config.value.log_folder = typeof selected === "string" ? selected : selected[0]);
  // save default act logs path
  const appDataDirPath = await appDataDir();
  const configPath = await join(appDataDirPath, "su-mentor");
  await invoke("write_config", {cfgDirPath: configPath, config: config.value});
  // load fights
  fights.value = await invoke("load_act_log", {path: config.value?.log_folder, meta: meta.value});
  exportProgress.value = -1;
}

const logFolderName = computed(() => {
  const path = config.value?.log_folder || "";
  const last = path.lastIndexOf(sep);
  return path.slice(path.lastIndexOf(sep, last - 1) + 1);
});

// mark useful fights
function setUseful(idx: number, status: boolean) {
  // eslint-disable-next-line security/detect-object-injection
  fights.value[idx].useful = status;
}

const usefulFlightsNum = computed(() => {
  if (fights) {
    return fights.value.filter((fight) => fight.useful).length;
  }
  return 0;
});


// refresh
// const refreshAnimation = ref(false);

async function refreshExport() {
  fights.value = await invoke("load_act_log", {path: config.value?.log_folder, meta: meta.value});
  exportProgress.value = -1;
  lastExportTime.value = Date.now();
}

// export fights -> notion, json, etc.
// const exportMode = ref(false);
const exportProgress = ref(-1);
const jsonExportAnimation = ref(false);

// auto export
const exportToNotion = ref(false);
const exportToSumemo = ref(false);
const syncMode = computed(() => {
  return exportToNotion.value || exportToSumemo.value;
});
const lastExportTime = ref(0);

// TODO: auto sync logic

async function exportFights(mode: string) {
  if (mode === "notion") {
    // read notion config
    const appDataDirPath = await appDataDir();
    const configPath = await join(appDataDirPath, "su-mentor");
    config.value = await invoke("read_config", {cfgDirPath: configPath});
    // notion export
    exportProgress.value = 0;
    await invoke("to_notion", {fights: fights.value, notionConfig: config.value?.notion});
  } else if (mode === "json") {
    exportProgress.value = 0;
    const json: string = await invoke("to_json", {fights: fights.value});
    await writeText(json);
    exportProgress.value = usefulFlightsNum.value;
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
  // auto refresh every 10s
  setInterval(async () => {
    if (config.value?.log_folder) {
      await refreshExport();
    }
  }, 10000);
});

// load config when mounted
onBeforeMount(async () => {
  // load config
  const appDataDirPath = await appDataDir();
  const configPath = await join(appDataDirPath, "su-mentor");
  config.value = await invoke("read_config", {cfgDirPath: configPath});
  // if log folder is set, load fights
  if (config.value?.log_folder) {
    // load fights
    fights.value = await invoke("load_act_log", {path: config.value?.log_folder, meta: meta.value});
    exportProgress.value = -1;
  }
});

</script>

<template>
  <div :class="{ 'blur-lg': exporting, 'pointer-events-none': exporting }" class="w-auto flex flex-col space-y-2 pb-4">
    <div class="alert alert-warning w-auto">
      <SvgIcon class="h-5 w-5" fill="none" icon-name="git" viewBox="0 0 24 24"/>
      <span class="text-sm"> 这是一个正在开发的页面 稳定性可能有所下降 </span>
      <span class="text-sm font-mono"> by 酥 Mar.24.2024 </span>
    </div>

    <div class="flex space-x-2">
      <button class="w-3/12 h-auto btn btn-primary" @click="updateLogFolder">选择日志文件夹</button>
      <div
          v-if="config?.log_folder"
          class="alert flex whitespace-nowrap h-fit w-full overflow-x-hidden justify-self-center items-center">
        <span class="text-sm font-mono">{{ logFolderName }}</span>
      </div>
      <button v-if="config?.log_folder" :class="{'btn-success': syncMode}" class="btn btn-accent h-auto w-1/12">
        <svg-icon v-if="!syncMode" class="h-5 w-5" fill="none" icon-name="refresh" viewBox="0 0 24 24"/>
        <svg-icon v-if="syncMode" class="h-5 w-5 breathe-effect" fill="none" icon-name="refresh" viewBox="0 0 24 24"/>
      </button>
    </div>

    <div v-if="fights && fights.length !== 0" class="w-auto flex flex-col space-y-2">
      <div class="flex space-x-2">
        <button class="w-3/12 h-12 btn btn-secondary">自动导出到
        </button>
        <button :class="{'btn-success': exportToNotion}" class="w-2/12 h-12 btn btn-base-200" @click="exportToNotion = !exportToNotion">
          Notion
        </button>
        <button :class="{'btn-success': exportToSumemo}" class="w-2/12 h-12 btn btn-base-200" disabled @click="exportFights('sumemo')">酥卷
        </button>
      </div>

      <FightTable
          v-for="(fight, f_idx) in fights" :key="fight.area.op.timestamp" :fight="fight" :idx="f_idx as number"
          @emit-set-useful="setUseful"/>
    </div>

    <div
        v-if="fights && fights.length === 0 && config?.log_folder"
        class="alert bg-base-100 flex whitespace-nowrap h-12 w-auto overflow-x-scroll justify-self-center items-center">
      <svg-icon class="h-5 w-5" fill="none" icon-name="slash" viewBox="0 0 24 24"/>
      <span class="text-sm">无可用战斗数据</span>
    </div>
  </div>

  <div v-if="exporting" class="toast toast-center toast-middle">
    <div class="flex flex-col space-y-3 justify-center items-center">
      <span class="text-lg font-mono font-bold">{{ progressNumber.toFixed(1) + "%" }}</span>
      <progress :max="usefulFlightsNum" :value="exportProgress" class="progress w-56"/>
    </div>
  </div>
</template>

<style scoped>
.breathe-effect {
  animation: breathe 2s ease-in-out infinite;
}

@keyframes breathe {
  0% {
    rotate: 0;
  }
  50% {
    rotate: 180deg;
  }
  100% {
    rotate: 360deg;
  }
}
</style>
