<script setup lang="tsx">
import CrudTable from '@/components/table/CrudTable.vue'
import { type Column, TableV2FixedDir, ElButton, type CheckboxValueType } from 'element-plus'
import { ref, type Ref } from 'vue'
const columns: Array<Column> = [
  {
    dataKey: 'name',
    title: 'Name',
    width: 100,
    align: 'center',
    fixed: TableV2FixedDir.LEFT
  },
  {
    key: 'operations',
    title: 'Operations',
    cellRenderer: () => (
      <>
        <ElButton size="small">修改</ElButton>
        <ElButton size="small">删除</ElButton>
      </>
    ),
    width: 150,
    align: 'center',
    fixed: TableV2FixedDir.RIGHT
  }
]
interface Category {
  name: string
}
let data: Ref<Category[]> = ref([
  {
    name: 'test'
  },
  {
    name: 'test1'
  }
])
const selections: Ref<Set<CheckboxValueType>> = ref(new Set<CheckboxValueType>())
</script>
<template>
  <crud-table
    :columns="columns"
    rowKey="name"
    v-model:selections="selections"
    :data="data"
    :total="500"
  >
  </crud-table>
</template>
