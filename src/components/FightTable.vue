<script lang="ts" setup>

// props
const props = defineProps<{
  idx: number;
  fight: FightRecord;
}>();

// emits
const emits = defineEmits(["emit-set-useful"]);

// mark as useful
const checked = ref(false);

function markUseful() {
  checked.value = !checked.value;
  emits("emit-set-useful", props.idx, checked);
  console.log(props.idx, checked);
}
</script>

<template>
  <div class="flex items-center space-x-4">
    <input
        class="checkbox ml-2" type="checkbox"
        @click="markUseful()"/>
    <div class="collapse collapse-arrow border border-base-300 bg-base-200" tabindex="0">
      <div class="collapse-title flex items-center justify-between">
        <p class="ml-2 text-base font-medium"> {{ props.fight.area.instance.name }}</p>
        <p class="text-sm font-mono"> {{ props.fight.players[0].job.name + " lv." + props.fight.players[0].level }}
        </p>
      </div>
      <div class="collapse-content card glass">
        <table class="table table-sm">
          <thead>
          <tr>
            <th></th>
            <th>ID</th>
            <th>职业</th>
            <th>等级</th>
          </tr>
          </thead>
          <tbody>
          <tr
              v-for="(player, p_idx) in props.fight.players" :key="player.id"
              :class="{ 'bg-base-300': p_idx == 0 }">
            <td class="font-mono">{{ p_idx + 1 }}</td>
            <td>{{ player.name }}</td>
            <td>{{ player.job.name }}</td>
            <td class="font-mono">{{ "lv." + player.level }}</td>
          </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
