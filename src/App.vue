<script setup lang="ts">
import { ref, onMounted } from 'vue'
import FloatingBall from './components/FloatingBall.vue'
import InputPanel from './components/InputPanel.vue'
import SettingsPanel from './components/SettingsPanel.vue'
import StatsPanel from './components/StatsPanel.vue'

const isExpanded = ref(false)
const showSettings = ref(false)
const showStats = ref(false)
const position = ref({ x: 0, y: 0 })

const togglePanel = () => {
  isExpanded.value = !isExpanded.value
  if (!isExpanded.value) {
    showSettings.value = false
    showStats.value = false
  }
}

const openSettings = () => {
  showSettings.value = true
  showStats.value = false
}

const closeSettings = () => {
  showSettings.value = false
}

const openStats = () => {
  showStats.value = true
  showSettings.value = false
}

const closeStats = () => {
  showStats.value = false
}

// 监听窗口位置变化
onMounted(async () => {
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    const win = getCurrentWindow()

    win.onMoved(({ payload }) => {
      position.value = { x: payload.x, y: payload.y }
    })
  } catch (e) {
    console.log('Window API not available')
  }
})
</script>

<template>
  <div class="app-container">
    <!-- 悬浮球 -->
    <FloatingBall
      @click="togglePanel"
      @settings="openSettings"
      @stats="openStats"
    />

    <!-- 输入面板 -->
    <InputPanel
      v-if="isExpanded && !showSettings && !showStats"
      @close="isExpanded = false"
    />

    <!-- 设置面板 -->
    <SettingsPanel
      v-if="showSettings"
      @close="closeSettings"
    />

    <!-- 统计面板 -->
    <StatsPanel
      v-if="showStats"
      @close="closeStats"
    />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  background: transparent;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

.app-container {
  position: relative;
  width: 100vw;
  height: 100vh;
  background: transparent;
}
</style>