<script setup lang="ts">
  import AppProvider from './components/common/AppProvider.vue';
  import SideNavBar from '@/components/navbar/SideNavBar.vue';
  import { ElContainer,ElAside,ElHeader,ElMain,ElButton } from 'element-plus';
</script>

<template>
  <app-provider>
      <el-container>
      <el-aside width="200px">
        <side-nav-bar>

        </side-nav-bar>
      </el-aside>
      <el-container>
        <el-header>Header</el-header>
        <el-main>
          <router-view>
            
          </router-view>
        </el-main>
      </el-container>
    </el-container>
  </app-provider>
</template>