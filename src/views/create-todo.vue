<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';

const text = ref('');
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const router = useRouter();
const route = useRoute();

// URLにIDがあるか（編集モードか）を確認
const editId = route.params.id ? Number(route.params.id) : null;

onMounted(() => {
  // 編集モードなら、保存されたデータを読み込む
  if (editId) {
    const rawData = localStorage.getItem('my-todos');
    if (rawData) {
      const todos = JSON.parse(rawData);
      const target = todos.find((t: any) => t.id === editId);
      if (target) text.value = target.content;
    }
  }
  textareaRef.value?.focus();
});

const save = async () => {
  if (!text.value.trim()) {
    router.back();
    return;
  }

  const rawData = localStorage.getItem('my-todos');
  let todos = rawData ? JSON.parse(rawData) : [];

  if (editId) {
    // 【編集モード】既存のデータを更新
    todos = todos.map((t: any) => 
      t.id === editId ? { ...t, content: text.value } : t
    );
  } else {
    // 新規データを追加
    todos.unshift({ 
      id: Date.now(), 
      content: text.value, 
      lastCompletedDay: 0 // 追加
    });
    // Rustにて通知予約
    try {
      await invoke('schedule_forgetting_curve_notifications', { 
        content: text.value 
      });
      console.log("通知がOSのスケジューラに登録されました");
    } catch (err) {
      console.error("通知の予約に失敗しました:", err);
    }
  }

  localStorage.setItem('my-todos', JSON.stringify(todos));
  router.push('/');
};
</script>

<template>
  <div class="full-screen-editor">
    <header class="editor-header">
      <button @click="router.back()" class="text-btn">キャンセル</button>
      <h3>{{ editId ? '編集' : '新規作成' }}</h3>
      <button @click="save" class="save-btn">完了</button>
    </header>

    <textarea 
      ref="textareaRef"
      v-model="text" 
      placeholder="内容を入力..." 
      class="full-textarea"
    ></textarea>
  </div>
</template>

<style scoped>
.full-screen-editor {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: white;
  display: flex;
  flex-direction: column;
  z-index: 100;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  border-bottom: 1px solid #f0f0f0;
}

.full-textarea {
  flex: 1; /* 残りの画面をすべて占有 */
  width: 100%;
  padding: 20px;
  border: none;
  outline: none; /* フォーカス時の枠線を消す */
  font-size: 18px;
  line-height: 1.6;
  resize: none; /* 右下のサイズ調整を消す */
  box-sizing: border-box;
}

.save-btn {
  background: #007aff;
  color: white;
  border: none;
  padding: 6px 16px;
  border-radius: 20px;
  font-weight: bold;
}

.text-btn {
  background: none;
  border: none;
  color: #666;
  font-size: 16px;
}
</style>