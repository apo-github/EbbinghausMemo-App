<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';

const todos = ref<any[]>([]);
const router = useRouter();
let pressTimer: number | null = null; // 長押し判定用のタイマー

const loadTodos = () => {
  const rawData = localStorage.getItem('my-todos');
  todos.value = rawData ? JSON.parse(rawData) : [];
};

// 削除機能
const deleteTodo = (id: number) => {
  if (window.confirm('このTODOを削除しますか？')) {
    const updated = todos.value.filter(t => t.id !== id);
    localStorage.setItem('my-todos', JSON.stringify(updated));
    todos.value = updated;
  }
};

// 長押し開始（800ミリ秒押し続けたら削除実行）
const startPress = (id: number) => {
  pressTimer = window.setTimeout(() => {
    deleteTodo(id);
    pressTimer = null;
  }, 800);
};

// 指を離した・マウスが離れたらタイマーをキャンセル
const cancelPress = () => {
  if (pressTimer) {
    clearTimeout(pressTimer);
    pressTimer = null;
  }
};

const editTodo = (id: number) => {
  // 長押しタイマーが動いていない（＝普通のタップ）時だけ編集画面へ
  if (!pressTimer) {
    router.push(`/edit/${id}`);
  }
};

onMounted(loadTodos);
</script>

<template>
  <div class="page-container">
    <header class="page-header">
        <h3>一覧</h3>
    </header>
    <div class="list">
      <div 
        v-for="todo in todos" 
        :key="todo.id" 
        class="todo-card"
        @mousedown="startPress(todo.id)"
        @mouseup="cancelPress"
        @mouseleave="cancelPress"
        @touchstart="startPress(todo.id)"
        @touchend="cancelPress"
        @click="editTodo(todo.id)"
      >
        <p>{{ todo.content }}</p>
      </div>
    </div>
  </div>
</template>


<style scoped>
.todo-card {
  /* 長押し時にテキスト選択されないようにする（操作感向上） */
  user-select: none;
  -webkit-user-select: none;
  
  background: white;
  margin: 10px;
  padding: 15px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  border-left: 4px solid #007aff; /* アクセント */
  cursor: pointer;

}
.todo-card p { margin: 0; white-space: pre-wrap; }

.todo-card:active {
  background: #f0f0f0; /* 押してる感のフィードバック */
}
.page-container { padding: 2px; padding-bottom: 80px; }
.page-header { display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 10px; margin-top: 5%;}
</style>