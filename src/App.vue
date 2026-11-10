<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const status = ref("尚未检查");
const checking = ref(false);

async function checkBackend() {
  checking.value = true;
  status.value = "正在连接…";

  try {
    status.value = await invoke<string>("backend_health");
  } catch (error) {
    status.value = String(error);
  } finally {
    checking.value = false;
  }
}
</script>

<template>
  <main>
    <p class="label">LOG AGENT · PROJECT 01</p>
    <h1>日志排障助手</h1>
    <p class="description">
      本地桌面工具 · Java 后端连接验证
    </p>

    <section aria-labelledby="backend-title">
      <h2 id="backend-title">后端状态</h2>
      <p role="status">{{ status }}</p>

      <button
        type="button"
        :disabled="checking"
        @click="checkBackend"
      >
        {{ checking ? "检查中…" : "检查连接" }}
      </button>
    </section>
  </main>
</template>

<style scoped>
main {
  max-width: 720px;
  margin: 10vh auto;
  padding: 32px;
  font-family: "Microsoft YaHei", sans-serif;
  color: #e9eef8;
}
.label { color: #79e0ad; letter-spacing: 0.12em; }
h1 { margin-bottom: 8px; font-size: 36px; }
.description { color: #b7c3d7; }
section {
  margin-top: 36px;
  padding: 24px;
  border: 1px solid #445269;
  border-radius: 12px;
  background: #182235;
}
button {
  min-height: 44px;
  padding: 0 20px;
  border: 0;
  border-radius: 8px;
  background: #79e0ad;
  color: #10251a;
  cursor: pointer;
}
button:focus-visible { outline: 3px solid white; outline-offset: 3px; }
button:disabled { opacity: 0.6; cursor: wait; }
:global(body) { margin: 0; background: #0f172a; }
</style>