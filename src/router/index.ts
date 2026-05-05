import { createRouter, createWebHashHistory } from 'vue-router';

import Home from '../views/home.vue';
import Notifications from '../views/notifications.vue';
import CreateTodo from '../views/create-todo.vue';

const routes = [
    {path: '/', component: Home },
    {path: '/notifications', component: Notifications },
    {path: '/create', component: CreateTodo },
    {path: '/edit/:id', component: CreateTodo}
]

export const router = createRouter( {
    history: createWebHashHistory(),
    routes
})