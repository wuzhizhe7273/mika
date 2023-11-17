<script setup lang="ts">
 import {ElMenu, ElSubMenu,ElMenuItemGroup,ElMenuItem} from 'element-plus'
</script>
<template>
    <el-menu>
        <el-sub-menu index="1">
            <template #title>
                <span>导航1</span>
            </template>
            <el-menu-item-group>
                <el-menu-item>
                    <router-link to="/content/category">
                        分类管理
                    </router-link>
                </el-menu-item>
            </el-menu-item-group>
        </el-sub-menu>
    </el-menu>
</template>