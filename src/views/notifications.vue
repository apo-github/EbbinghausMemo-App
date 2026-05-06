<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useRouter } from 'vue-router';

const allTodos = ref<any[]>([]);
const router = useRouter();

// 朝4時基準の経過日数計算
const getAdjustedDiffDays = (id: number) => {
  const now = new Date();
  const adjustedNow = new Date(now.getTime() - (4 * 60 * 60 * 1000));
  const todayStart = new Date(adjustedNow.getFullYear(), adjustedNow.getMonth(), adjustedNow.getDate()).getTime();
  
  const createdDate = new Date(id);
  const createdStart = new Date(createdDate.getFullYear(), createdDate.getMonth(), createdDate.getDate()).getTime();
  
  const MS_PER_DAY = 24 * 60 * 60 * 1000;
  return Math.floor((todayStart - createdStart) / MS_PER_DAY);
};

const loadTodos = () => {
  const rawData = localStorage.getItem('my-todos');
  allTodos.value = rawData ? JSON.parse(rawData) : [];
};

const notificationTodos = computed(() => {
  return allTodos.value
    .map(todo => {
      const diffDays = getAdjustedDiffDays(todo.id);
      return { ...todo, currentDiffDays: diffDays };
    })
    .filter(todo => {
      // 復習スケジュール（テスト用: 0, 本番: 1, 3, 7, 30）
      const schedule = [0, 1, 3, 7, 30]; 
      
      // 【修正】チェック済みかどうかに関わらず、その日が復習日なら表示する
      // これにより「朝4時」が来るまでリストに残り続けます
      return schedule.includes(todo.currentDiffDays);
    });
});

// チェックボックス操作時の処理
const toggleComplete = (id: number, event: Event) => {
  const isChecked = (event.target as HTMLInputElement).checked;
  const diffDays = getAdjustedDiffDays(id);

  const updated = allTodos.value.map(todo => {
    if (todo.id === id) {
      // チェックされたら今日の経過日数を保存、外されたらクリア（または以前の値に）
      return { ...todo, lastCompletedDay: isChecked ? diffDays : (diffDays - 1) };
    }
    return todo;
  });

  localStorage.setItem('my-todos', JSON.stringify(updated));
  allTodos.value = updated;
};

const editTodo = (id: number) => {
  router.push(`/edit/${id}`);
};

onMounted(loadTodos);
</script>

<template>
  <div class="page-container">
    <header class="page-header">
      <h3>今日の復習</h3>
      <span class="info-text">朝4時に自動リセットされます</span>
    </header>
    
    <div v-if="notificationTodos.length === 0" class="empty-message">
      今日の復習タスクはありません。
    </div>

    <div class="list">
      <div 
        v-for="todo in notificationTodos" 
        :key="todo.id" 
        class="todo-card"
        :class="{ 'is-completed': todo.lastCompletedDay === todo.currentDiffDays }"
        @click="editTodo(todo.id)"
      >
        <div class="card-layout">
          <div class="checkbox-area" @click.stop>
            <input 
              type="checkbox" 
              class="complete-checkbox" 
              :checked="todo.lastCompletedDay === todo.currentDiffDays"
              @change="toggleComplete(todo.id, $event)"
            >
          </div>

          <div class="card-content">
            <p :class="{ 'strike-text': todo.lastCompletedDay === todo.currentDiffDays }">
              {{ todo.content }}
            </p>
            <div class="card-footer">
              <span class="tag">
                {{ todo.currentDiffDays === 0 ? '当日の復習' : todo.currentDiffDays + '日目の復習' }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 基本レイアウト */
.todo-card {
  background: white;
  margin-bottom: 15px;
  padding: 15px;
  margin: 10px;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0,0,0,0.05);
  border-left: 5px solid #007aff;
  transition: all 0.3s ease;
}

.todo-card p { margin: 0; white-space: pre-wrap; }

.card-layout {
  display: flex;
  align-items: flex-start; /* チェックボックスを上揃えに */
  gap: 15px;
}

/* 右側のコンテンツエリア */
.card-content {
  flex: 1;
  display: flex;
  flex-direction: column; /* 縦に並べる */
  min-height: 50px; /* タグが下に沈むように高さを確保 */
}

/* タグを囲むコンテナを右寄せにする */
.card-footer {
  margin-top: 8px;
  display: flex;
  justify-content: flex-end; /* これで右端に寄る */
}

.tag {
  font-size: 11px;
  background: #e1f0ff;
  color: #007aff;
  padding: 4px 10px;
  border-radius: 6px;
  font-weight: bold;
  white-space: nowrap; /* 改行を防ぐ */
}

/* 完了時のスタイル */
.todo-card.is-completed {
  opacity: 0.6;
  border-left-color: #ccc;
  background: #fafafa;
}

.strike-text {
  text-decoration: line-through;
  color: #999;
}

/* チェックボックスの大きさ */
.complete-checkbox {
  width: 20px;
  height: 20px;
  cursor: pointer;
  accent-color: #007aff;
}

/* その他 */
.page-container { padding: 2px; padding-bottom: 80px; }
.page-header { display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 10px; margin-top: 5%;}
.info-text { font-size: 11px; color: #999; }
.empty-message { text-align: center; color: #999; margin-top: 50px; }
</style>