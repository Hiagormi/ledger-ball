<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits<{
  click: []
  settings: []
  stats: []
}>()

const isHovered = ref(false)
const showMenu = ref(false)

const handleClick = () => {
  if (showMenu.value) {
    showMenu.value = false
  } else {
    emit('click')
  }
}

const handleRightClick = (e: MouseEvent) => {
  e.preventDefault()
  showMenu.value = !showMenu.value
}

const handleSettings = () => {
  showMenu.value = false
  emit('settings')
}

const handleStats = () => {
  showMenu.value = false
  emit('stats')
}
</script>

<template>
  <div
    class="floating-ball"
    :class="{ expanded: isHovered }"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false; showMenu = false"
    @click="handleClick"
    @contextmenu="handleRightClick"
  >
    <div class="ball-inner">
      <span class="ball-icon">💰</span>
    </div>

    <!-- 右键菜单 -->
    <div v-if="showMenu" class="context-menu">
      <div class="menu-item" @click.stop="handleStats">
        📊 统计
      </div>
      <div class="menu-item" @click.stop="handleSettings">
        ⚙️ 设置
      </div>
    </div>
  </div>
</template>

<style scoped>
.floating-ball {
  position: fixed;
  right: 20px;
  top: 50%;
  transform: translateY(-50%);
  width: 56px;
  height: 56px;
  border-radius: 50%;
  cursor: pointer;
  z-index: 1000;
  transition: all 0.3s ease;
}

.ball-inner {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 20px rgba(102, 126, 234, 0.4);
  transition: all 0.3s ease;
}

.floating-ball:hover .ball-inner {
  transform: scale(1.1);
  box-shadow: 0 6px 25px rgba(102, 126, 234, 0.6);
}

.ball-icon {
  font-size: 24px;
}

.context-menu {
  position: absolute;
  right: 70px;
  top: 50%;
  transform: translateY(-50%);
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  padding: 8px 0;
  min-width: 120px;
}

.menu-item {
  padding: 10px 16px;
  cursor: pointer;
  transition: background 0.2s;
  font-size: 14px;
  color: #333;
}

.menu-item:hover {
  background: #f5f5f5;
}
</style>