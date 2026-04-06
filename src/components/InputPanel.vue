<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const emit = defineEmits<{
  close: []
}>()

const inputText = ref('')
const isLoading = ref(false)
const lastResult = ref<string | null>(null)
const error = ref<string | null>(null)

// 解析输入
const parseInput = async () => {
  if (!inputText.value.trim()) return

  isLoading.value = true
  error.value = null
  lastResult.value = null

  try {
    // 调用后端解析并写入
    const result = await invoke<string>('parse_and_record', {
      input: inputText.value.trim()
    })

    lastResult.value = result
    inputText.value = ''

    // 3秒后清除结果
    setTimeout(() => {
      lastResult.value = null
    }, 3000)
  } catch (e) {
    error.value = String(e)
  } finally {
    isLoading.value = false
  }
}

// 快捷输入模板
const templates = [
  { label: '餐饮', example: '餐饮 午饭 35' },
  { label: '交通', example: '交通 打车 25' },
  { label: '购物', example: '购物 买菜 128' },
  { label: '娱乐', example: '娱乐 电影 50' },
  { label: '收入', example: '收入 工资 +5000' },
]

const useTemplate = (template: string) => {
  inputText.value = template
}
</script>

<template>
  <div class="input-panel">
    <div class="panel-header">
      <h3>📝 记账助手</h3>
      <button class="close-btn" @click="emit('close')">✕</button>
    </div>

    <div class="panel-body">
      <!-- 输入框 -->
      <div class="input-section">
        <input
          v-model="inputText"
          type="text"
          placeholder="输入: 午饭 35 或 餐饮 麦当劳 50"
          class="input-field"
          @keyup.enter="parseInput"
          :disabled="isLoading"
        />
        <button
          class="submit-btn"
          @click="parseInput"
          :disabled="isLoading || !inputText.trim()"
        >
          {{ isLoading ? '...' : '记录' }}
        </button>
      </div>

      <!-- 快捷模板 -->
      <div class="templates">
        <span class="template-label">快捷:</span>
        <button
          v-for="t in templates"
          :key="t.label"
          class="template-btn"
          @click="useTemplate(t.example)"
        >
          {{ t.label }}
        </button>
      </div>

      <!-- 结果显示 -->
      <div v-if="lastResult" class="result success">
        {{ lastResult }}
      </div>

      <!-- 错误显示 -->
      <div v-if="error" class="result error">
        ❌ {{ error }}
      </div>

      <!-- 使用说明 -->
      <div class="help-text">
        <p>💡 支持的输入格式:</p>
        <ul>
          <li><code>分类 金额</code> - 如: 餐饮 35</li>
          <li><code>分类 备注 金额</code> - 如: 餐饮 午饭 35</li>
          <li><code>日期 分类 金额</code> - 如: 3/15 餐饮 50</li>
          <li><code>收入 分类 +金额</code> - 如: 收入 工资 +5000</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.input-panel {
  position: fixed;
  right: 80px;
  top: 50%;
  transform: translateY(-50%);
  width: 320px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.2);
  z-index: 1001;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #eee;
}

.panel-header h3 {
  font-size: 16px;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  cursor: pointer;
  font-size: 18px;
  color: #999;
}

.panel-body {
  padding: 16px;
}

.input-section {
  display: flex;
  gap: 8px;
  margin-bottom: 12px;
}

.input-field {
  flex: 1;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
}

.input-field:focus {
  border-color: #667eea;
}

.submit-btn {
  padding: 12px 16px;
  background: linear-gradient(135deg, #667eea, #764ba2);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
}

.submit-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.templates {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.template-label {
  font-size: 12px;
  color: #666;
}

.template-btn {
  padding: 6px 12px;
  background: #f0f0f0;
  border: none;
  border-radius: 6px;
  font-size: 12px;
  cursor: pointer;
  color: #333;
}

.template-btn:hover {
  background: #e0e0e0;
}

.result {
  padding: 12px;
  border-radius: 8px;
  margin-bottom: 12px;
  font-size: 14px;
}

.result.success {
  background: #e8f5e9;
  color: #2e7d32;
}

.result.error {
  background: #ffebee;
  color: #c62828;
}

.help-text {
  padding: 12px;
  background: #f5f5f5;
  border-radius: 8px;
  font-size: 12px;
  color: #666;
}

.help-text p {
  margin-bottom: 8px;
}

.help-text ul {
  list-style: none;
  padding: 0;
}

.help-text li {
  margin-bottom: 4px;
}

.help-text code {
  background: #e0e0e0;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
}
</style>