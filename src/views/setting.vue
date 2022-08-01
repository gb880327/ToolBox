<template>
  <el-tabs v-model="activeName">
    <el-tab-pane label="环境变量" name="env">
      <env />
    </el-tab-pane>
  </el-tabs>
</template>
<script lang="ts" setup>
import { ref } from 'vue';
import env from '@/components/env.vue'

const activeName = ref("env")

</script>