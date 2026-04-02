import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface TodoItem {
  id: number
  title: string
  status: 'pending' | 'in_progress' | 'completed'
}

export const useTodoStore = defineStore('todo', () => {
  const todos = ref<TodoItem[]>([])

  function setTodos(items: TodoItem[]) {
    todos.value = items
  }

  const pending = computed(() => todos.value.filter((t) => t.status === 'pending'))
  const inProgress = computed(() => todos.value.filter((t) => t.status === 'in_progress'))
  const completed = computed(() => todos.value.filter((t) => t.status === 'completed'))

  return { todos, pending, inProgress, completed, setTodos }
})
