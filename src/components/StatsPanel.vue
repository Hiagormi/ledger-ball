<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits<{
  close: []
}>()

interface Record {
  date: string
  type: string
  category: string
  amount: number
  note: string
}

interface Stats {
  total_income: number
  total_expense: number
  by_category: Record<string, number>
  records: Record[]
}

const stats = ref<Stats | null>(null)
const isLoading = ref(false)
const period = ref<'day' | 'week' | 'month' | 'year'>('month')

const balance = computed(() => {
  if (!stats.value) return 0
  return stats.value.total_income - stats.value.total_expense
})

const sortedCategories = computed(() => {
  if (!stats.value) return []
  return Object.entries(stats.value.by_category)
    .sort((a, b) => b[1] - a[1])
})

const loadStats = async () => {
  isLoading.value = true
  try {
    const result = await invoke<Stats>('get_stats', { period: period.value })
    stats.value = result
  } catch (e) {
    console.error('Failed to load stats:', e)
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  loadStats()
})
</script>

<template>
  <div class="stats-panel">
    <div class="panel-header">
      <h3>📊 统计</h3>
      <button class="close-btn" @click="emit('close')">✕</button>
    </div>

    <div class="panel-body">
      <!-- 周期选择 -->
      <div class="period-tabs">
        <button
          v-for="p in ['day', 'week', 'month', 'year']"
          :key="p"
          :class="['tab', { active: period === p }]"
          @click="period = p as any; loadStats()"
        >
          {{ { day: '今日', week: '本周', month: '本月', year: '本年' }[p as string] }}
        </button>
      </div>

      <!-- 加载中 -->
      <div v-if="isLoading" class="loading">
        加载中...
      </div>

      <!-- 统计数据 -->
      <template v-else-if="stats">
        <!-- 总览 -->
        <div class="overview">
          <div class="overview-item income">
            <span class="label">收入</span>
            <span class="amount">¥{{ stats.total_income.toFixed(2) }}</span>
          </div>
          <div class="overview-item expense">
            <span class="label">支出</span>
            <span class="amount">¥{{ stats.total_expense.toFixed(2) }}</span>
          </div>
          <div class="overview-item balance" :class="{ positive: balance >= 0, negative: balance < 0 }">
            <span class="label">结余</span>
            <span class="amount">¥{{ balance.toFixed(2) }}</span>
          </div>
        </div>

        <!-- 分类统计 -->
        <div class="category-stats">
          <h4>支出分类</h4>
          <div v-if="sortedCategories.length === 0" class="empty">
            暂无数据
          </div>
          <div v-else class="category-list">
            <div
              v-for="[category, amount] in sortedCategories"
              :key="category"
              class="category-item"
            >
              <span class="category-name">{{ category }}</span>
              <div class="category-bar-container">
                <div
                  class="category-bar"
                  :style="{
                    width: `${(amount / stats.total_expense) * 100}%`
                  }"
                ></div>
              </div>
              <span class="category-amount">¥{{ amount.toFixed(2) }}</span>
            </div>
          </div>
        </div>

        <!-- 最近记录 -->
        <div class="recent-records">
          <h4>最近记录</h4>
          <div v-if="stats.records.length === 0" class="empty">
            暂无记录
          </div>
          <div v-else class="record-list">
            <div
              v-for="(record, index) in stats.records.slice(-10).reverse()"
              :key="index"
              class="record-item"
            >
              <div class="record-info">
                <span class="record-category">{{ record.category }}</span>
                <span class="record-note" v-if="record.note">{{ record.note }}</span>
              </div>
              <div class="record-right">
                <span class="record-amount" :class="record.type">
                  {{ record.type === 'income' ? '+' : '-' }}¥{{ record.amount.toFixed(2) }}
                </span>
                <span class="record-date">{{ record.date }}</span>
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.stats-panel {
  position: fixed;
  right: 80px;
  top: 50%;
  transform: translateY(-50%);
  width: 380px;
  max-height: 80vh;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.2);
  z-index: 1001;
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

.period-tabs {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.tab {
  flex: 1;
  padding: 8px;
  border: none;
  background: #f0f0f0;
  border-radius: 8px;
  cursor: pointer;
  font-size: 13px;
  color: #666;
  transition: all 0.2s;
}

.tab.active {
  background: linear-gradient(135deg, #667eea, #764ba2);
  color: white;
}

.loading {
  text-align: center;
  padding: 40px;
  color: #999;
}

.overview {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
}

.overview-item {
  flex: 1;
  padding: 12px;
  border-radius: 8px;
  text-align: center;
}

.overview-item.income {
  background: #e8f5e9;
}

.overview-item.expense {
  background: #ffebee;
}

.overview-item.balance.positive {
  background: #e3f2fd;
}

.overview-item.balance.negative {
  background: #fff3e0;
}

.overview-item .label {
  display: block;
  font-size: 12px;
  color: #666;
  margin-bottom: 4px;
}

.overview-item .amount {
  display: block;
  font-size: 14px;
  font-weight: 600;
}

.category-stats,
.recent-records {
  margin-bottom: 20px;
}

.category-stats h4,
.recent-records h4 {
  font-size: 14px;
  color: #333;
  margin-bottom: 12px;
}

.empty {
  text-align: center;
  padding: 20px;
  color: #999;
  font-size: 13px;
}

.category-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.category-item {
  display: flex;
  align-items: center;
  gap: 10px;
}

.category-name {
  width: 60px;
  font-size: 13px;
  color: #333;
}

.category-bar-container {
  flex: 1;
  height: 8px;
  background: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
}

.category-bar {
  height: 100%;
  background: linear-gradient(135deg, #667eea, #764ba2);
  border-radius: 4px;
}

.category-amount {
  width: 70px;
  text-align: right;
  font-size: 13px;
  color: #333;
}

.record-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 200px;
  overflow-y: auto;
}

.record-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  background: #f9f9f9;
  border-radius: 8px;
}

.record-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.record-category {
  font-size: 13px;
  font-weight: 500;
  color: #333;
}

.record-note {
  font-size: 11px;
  color: #999;
}

.record-right {
  text-align: right;
}

.record-amount {
  display: block;
  font-size: 13px;
  font-weight: 500;
}

.record-amount.income {
  color: #2e7d32;
}

.record-amount.expense {
  color: #c62828;
}

.record-date {
  display: block;
  font-size: 11px;
  color: #999;
}
</style>