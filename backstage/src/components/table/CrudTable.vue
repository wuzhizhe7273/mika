<script setup lang="tsx">
import {
  ElTableV2,
  ElPagination,
  ElAutoResizer,
  TableV2FixedDir,
  type CheckboxValueType,
  ElCheckbox
} from 'element-plus'
import type { Column } from 'element-plus/lib/components/index.js'
import { unref, type FunctionalComponent, type Ref, ref, computed, type ComputedRef } from 'vue'
export interface Props {
  columns: Array<Column>
  selections: Set<CheckboxValueType>
  rowKey: any
  data: any[]
  total?:number
}
const props = defineProps<Props>()
const emit = defineEmits<{
  'update:selections': [value: Set<CheckboxValueType>]
}>()
type SelectionCellProps = {
  value: boolean
  intermediate?: boolean
  onChange: (value: CheckboxValueType) => void
}
const SelectionCell: FunctionalComponent<SelectionCellProps> = ({
  value,
  intermediate = false,
  onChange
}) => {
  return <ElCheckbox onChange={onChange} modelValue={value} indeterminate={intermediate} />
}
const handerField = function <T extends object, K extends keyof T>(obj: T, key: K) {
  return obj[key]
}

const tableColumns: ComputedRef<Array<Column>> = computed(() => {
  let selectionCol: Array<Column> = [
    {
      key: 'selection',
      width: 50,
      fixed: TableV2FixedDir.LEFT,
      cellRenderer: ({ rowData }) => {
        let checked=props.selections.has(rowData[props.rowKey]) as boolean
        const onChange = (value: CheckboxValueType) => {
          let newSelections: Set<CheckboxValueType> = new Set(props.selections.values())
          if (value===false) {
            newSelections.delete(rowData[props.rowKey])
          }else{
            newSelections.add(rowData[props.rowKey])
          }
          console.log(newSelections)
          emit('update:selections', newSelections)
        }
        return <SelectionCell value={checked} onChange={onChange}></SelectionCell>
      },
      headerCellRenderer: () => {
        const _data = unref(props.data)
        const onChange = (value: CheckboxValueType) =>{
          if (value===true){
            let newSelections: Set<CheckboxValueType>=new Set(_data.map((row)=>handerField(row,props.rowKey)))
            emit('update:selections',newSelections)
          }else{
            emit('update:selections',new Set<CheckboxValueType>())
          }
        }
        const allSelected = props.selections.size===_data.length
        const contaionsChecked = _data.some((row) => props.selections.has(row[props.rowKey]))
        return (
          <SelectionCell
            value={allSelected}
            intermediate={contaionsChecked && !allSelected}
            onChange={onChange}
          />
        )
      }
    }
  ]
  let newColumns = selectionCol.concat(props.columns)
  return newColumns
})
</script>
<template>
  <div style="height: 800px">
    <el-auto-resizer #default="{ height, width }">
      <el-table-v2 :columns="tableColumns" :data="props.data" :width="width" :height="height">
        <template #footer>
          <div class="flex justify-end items-center">
            <el-pagination
              :background="true"
              layout="sizes, prev, pager, next, jumper"
              :total="props.total"
              :page-sizes="[5, 10, 20]"
            >
            </el-pagination>
          </div>
        </template>
      </el-table-v2>
    </el-auto-resizer>
  </div>
</template>
