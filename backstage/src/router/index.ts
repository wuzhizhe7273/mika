import { createRouter, createWebHashHistory, type RouteRecordRaw} from "vue-router";
import CategoryView from '@/views/content/CategoryView.vue'
const routes:RouteRecordRaw[]=[
    {
        path:'/content',
        children:[
            {
                path:'category',
                component:CategoryView
            }
        ]

    }
]

const router=createRouter({
    history:createWebHashHistory(),
    routes
})

export default router