<template>
  <svg :style="{ width: size + 'px', height: size + 'px' }" aria-hidden="true">
    <use :fill="color" :xlink:href="symbolId"/>
  </svg>
</template>

<script lang="ts" setup>
import {computed} from "vue";

const props = defineProps({
  iconName: {
    type: String,
    required: true
  },
  color: {
    type: String,
    default: ""
  },
  size: {
    type: [Number, String],
    default: 18
  }
});

const symbolId = computed(() => `#icon-${props.iconName}`);
</script>


<style scoped>
.svg-icon {
  fill: currentColor;
  vertical-align: middle;
}
</style>
