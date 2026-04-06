<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const emit = defineEmits<{
  close: []
}>()

interface Config {
  parse_mode: 'local' | 'ai'
  ai_provider: string
  api_key: string
  api_base: string
  model: string
}

const config = ref<Config>({
  parse_mode: 'local',
  ai_provider: 'deepseek',
  api_key: '',
  api_base: '',
  model: ''
})

const isSaving = ref(false)
const saveMessage = ref('')

// AI 服务商配置
const aiProviders: Record<string, { name: string; baseUrl: string; model: string }> = {
  deepseek: { name: 'DeepSeek', baseUrl: 'https://api.deepseek.com/v1', model: 'deepseek-chat' },
  step: { name: '阶跃AI', baseUrl: 'https://api.stepfun.com/v1', model: 'step-1-8k' },
  openai: { name: 'OpenAI', baseUrl: 'https://api.openai.com/v1', model: 'gpt-3.5-turbo' },
  anthropic: { name: 'Claude', baseUrl: 'https://api.anthropic.com', model: 'claude-3-haiku-20240307' },
  custom: { name: '自定义', baseUrl: '', model: '' },
}

const onProviderChange = () => {
  const provider = aiProviders[config.value.ai_provider]
  if (provider && config.value.ai_provider !== 'custom') {
    config.value.api_base = provider.baseUrl
    config.value.model = provider.model
  }
}

const loadConfig = async () => {
  try {
    const saved = await invoke<Config>('get_config')
    if (saved) {
      config.value = saved
    }
  } catch (e) {
    console.error('Failed to load config:', e)
  }
}

const saveConfig = async () => {
  isSaving.value = true
  saveMessage.value = ''

  try {
    await invoke('save_config', { config: config.value })
    saveMessage.value = '✓ 保存成功'
    setTimeout(() => {
      saveMessage.value = ''
    }, 2000)
  } catch (e) {
    saveMessage.value = '❌ 保存失败: ' + String(e)
  } finally {
    isSaving.value = false
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<template>
  <div class="settings-panel">
    <div class="panel-header">
      <h3>⚙️ 设置</h3>
      <button class="close-btn" @click="emit('close')">✕</button>
    </div>

    <div class="panel-body">
      <!-- 解析模式 -->
      <div class="setting-item">
        <label>解析模式</label>
        <div class="radio-group">
          <label class="radio-label">
            <input
              type="radio"
              v-model="config.parse_mode"
              value="local"
            />
            <span>本地规则 (免费、快速)</span>
          </label>
          <label class="radio-label">
            <input
              type="radio"
              v-model="config.parse_mode"
              value="ai"
            />
            <span>AI解析 (智能理解)</span>
          </label>
        </div>
      </div>

      <!-- AI 配置 -->
      <template v-if="config.parse_mode === 'ai'">
        <div class="setting-item">
          <label>AI 服务商</label>
          <select
            v-model="config.ai_provider"
            @change="onProviderChange"
            class="select-field"
          >
            <option v-for="(provider, key) in aiProviders" :key="key" :value="key">
              {{ provider.name }}
            </option>
          </select>
        </div>

        <div class="setting-item">
          <label>API Key</label>
          <input
            v-model="config.api_key"
            type="password"
            placeholder="输入你的 API Key"
            class="input-field"
          />
        </div>

        <div class="setting-item">
          <label>API Base URL</label>
          <input
            v-model="config.api_base"
            type="text"
            placeholder="API 地址"
            class="input-field"
          />
        </div>

        <div class="setting-item">
          <label>模型名称</label>
          <input
            v-model="config.model"
            type="text"
            placeholder="模型名称"
            class="input-field"
          />
        </div>
      </template>

      <!-- 保存按钮 -->
      <button
        class="save-btn"
        @click="saveConfig"
        :disabled="isSaving"
      >
        {{ isSaving ? '保存中...' : '保存设置' }}
      </button>

      <div v-if="saveMessage" class="save-message">
        {{ saveMessage }}
      </div>

      <!-- 使用说明 -->
      <div class="help-section">
        <h4>💡 使用说明</h4>
        <p><strong>本地模式:</strong> 无需配置，开箱即用。支持固定格式的输入。</p>
        <p><strong>AI模式:</strong> 需要配置API Key，可以理解自然语言输入。</p>
        <p class="privacy">🔒 所有数据存储在本地，不会上传到任何服务器。</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-panel {
  position: fixed;
  right: 80px;
  top: 50%;
  transform: translateY(-50%);
  width: 360px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.2);
  z-index: 1001;
  max-height: 80vh;
  overflow-y: auto;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #eee;
  position: sticky;
  top: 0;
  background: white;
  border-radius: 12px 12px 0 0;
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

.setting-item {
  margin-bottom: 16px;
}

.setting-item > label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: #333;
  margin-bottom: 8px;
}

.input-field,
.select-field {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
}

.input-field:focus,
.select-field:focus {
  border-color: #667eea;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #333;
  cursor: pointer;
}

.save-btn {
  width: 100%;
  padding: 12px;
  background: linear-gradient(135deg, #667eea, #764ba2);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.save-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.save-message {
  text-align: center;
  margin-top: 12px;
  font-size: 14px;
  color: #2e7d32;
}

.help-section {
  margin-top: 20px;
  padding: 16px;
  background: #f5f5f5;
  border-radius: 8px;
}

.help-section h4 {
  font-size: 14px;
  margin-bottom: 12px;
  color: #333;
}

.help-section p {
  font-size: 12px;
  color: #666;
  margin-bottom: 8px;
  line-height: 1.5;
}

.help-section .privacy {
  color: #667eea;
  margin-top: 12px;
}
</style>