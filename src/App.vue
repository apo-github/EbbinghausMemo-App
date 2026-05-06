<template>
  <main class="container">
    <div class="content">
      <router-view />
    </div>

    <router-link v-if="$route.path !== '/create' && $route.path !== '/notifications' && !$route.path.startsWith('/edit')" to="/create" class="fab-button">
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="white" viewBox="0 0 16 16">
        <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
      </svg>
    </router-link>

    <nav class="tab-bar">
      <router-link to="/" class="tab-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" viewBox="0 0 16 16">
          <path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5"/>
        </svg>
      </router-link>
      <router-link to="/notifications" class="tab-item">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
          <path d="M8 16a2 2 0 0 0 2-2H6a2 2 0 0 0 2 2m.995-14.901a1 1 0 1 0-1.99 0A5 5 0 0 0 3 6c0 1.098-.5 6-2 7h14c-1.5-1-2-5.902-2-7 0-2.42-1.72-4.44-4.005-4.901"/>
        </svg>
      </router-link>
    </nav>
  </main>
</template>

<style scoped>
.container {
  /* 固定の 100vh ではなく、最小高さにする */
  min-height: 100vh;
  display: block; /* flex から変更 */
  position: relative;
}

.content {
  /* ここでスクロールを管理せず、body側に任せる */
  padding-bottom: 80px; 
}

/* フローティングボタン(FAB)のスタイル */
.fab-button {
  position: fixed;
  right: 8%; /* 通知ボタン（右側50%領域）の中央付近 */
  bottom: calc(80px + env(safe-area-inset-bottom)); /* タブバーの上 */
  width: 56px;
  height: 56px;
  background-color: #007aff;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 12px rgba(0,0,0,0.2);
  z-index: 10;
  transition: transform 0.2s;
}
.fab-button:active {
  transform: scale(0.9);
}

/* 他のスタイルは以前のものを継承 */
.tab-bar {
  position: fixed;
  bottom: 0;
  width: 100%;
  height: 60px;
  display: flex;
  background: #fff;
  border-top: 1px solid #eee;
  padding-bottom: env(safe-area-inset-bottom);
  z-index: 5;
}
.tab-item { flex: 1; display: flex; align-items: center; justify-content: center; text-decoration: none; color: #333; }
.router-link-active { background-color: rgb(198, 242, 255); }
</style>