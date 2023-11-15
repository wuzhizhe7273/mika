<script setup lang="ts">
  import AppProvider from './components/common/AppProvider.vue';
  import { ElContainer,ElAside,ElHeader,ElMain } from 'element-plus';
</script>

<template>
  <app-provider>
    <div class="common-layout">
      <el-container>
      <el-aside width="200px">Aside</el-aside>
      <el-container>
        <el-header>Header</el-header>
        <el-main>Main</el-main>
      </el-container>
    </el-container>
    </div>
  </app-provider>
</template>