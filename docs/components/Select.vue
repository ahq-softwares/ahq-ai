<script setup>
import { ref, computed } from 'vue'

// --- Props & Emits for v-model compatibility ---
const props = defineProps({
  placeholder: {
    type: String,
    required: true
  },
  /**
   * The currently selected value (used for v-model).
   */
  modelValue: {
    type: [String, Number],
    required: true
  },
  /**
   * Array of options. Each object should have a 'text' (display) and 'value' (key) property.
   * Example: [{ text: 'Windows', value: 'windows' }, { text: 'MacOS', value: 'macos' }]
   */
  options: {
    type: Array,
    required: true,
    default: () => []
  }
})

const emit = defineEmits(['update:modelValue'])

// --- Internal State & Computed Properties ---
const isOpen = ref(false)

// Function to find the display text for the current selected value
const selectedText = computed(() => {
  const selectedOption = props.options.find(opt => opt.value === props.modelValue)
  return selectedOption ? selectedOption.text : props.placeholder
})

// --- Methods ---

// Handles selecting an option
const selectOption = (value) => {
  emit('update:modelValue', value)
  isOpen.value = false // Close the dropdown after selection
}

// Toggles the visibility of the dropdown list
const toggleDropdown = () => {
  isOpen.value = !isOpen.value
}

// --- Custom Directive for Click Outside ---
// This is defined locally so the component is self-contained.
const vClickOutside = {
  beforeMount(el, binding) {
    el.clickOutsideEvent = (event) => {
      // Check if the click is outside the component element
      if (!(el === event.target || el.contains(event.target))) {
        binding.value();
      }
    };
    document.addEventListener('click', el.clickOutsideEvent);
  },
  unmounted(el) {
    document.removeEventListener('click', el.clickOutsideEvent);
  },
};
</script>

<template>
  <div class="custom-select-wrapper" v-click-outside="() => isOpen = false">
    
    <!-- 1. The Select Button/Header -->
    <button 
      class="select-header" 
      @click="toggleDropdown" 
      aria-haspopup="listbox" 
      :aria-expanded="isOpen"
    >
      <span class="selected-text">{{ selectedText }}</span>
      <!-- Arrow icon, rotates based on 'isOpen' state -->
      <svg :class="{'rotated': isOpen}" width="1em" height="1em" viewBox="0 0 24 24">
        <path fill="currentColor" d="M7 10l5 5l5-5z"/>
      </svg>
    </button>

    <!-- 2. The Dropdown Options List -->
    <ul v-if="isOpen" class="options-list" role="listbox">
      <li 
        v-for="option in options" 
        :key="option.value"
        @click="selectOption(option.value)"
        class="option-item"
        :class="{ 'is-selected': modelValue === option.value }"
        role="option"
        :aria-selected="modelValue === option.value"
      >
        {{ option.text }}
      </li>
    </ul>
  </div>
</template>

<style scoped>
/*
 * Custom styles for a sleek, modern dropdown menu 
 * inspired by the standard VitePress UI.
 */
.custom-select-wrapper {
  position: relative;
  display: inline-block;
  user-select: none;
}

/* The Select Button/Header */
.select-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-width: 150px; /* Ensure a minimum width */
  width: 100%;
  gap: 8px;
  padding: 8px 16px;
  border: 1px solid var(--vp-c-border);
  border-radius: 6px;
  background-color: var(--vp-c-default-3); 
  color: var(--vp-c-text-1);
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: border-color 0.2s, background-color 0.2s;
}

.select-header:hover {
  border-color: var(--vp-c-brand-1); /* Primary brand color on hover */
}

/* Arrow Icon Styles and Rotation */
.select-header svg {
  transition: transform 0.2s;
  color: var(--vp-c-text-3);
}

.select-header svg.rotated {
  transform: rotate(180deg);
  color: var(--vp-c-brand-1);
}

/* The Options List Container (the actual dropdown) */
.options-list {
  position: absolute;
  top: calc(100% + 4px); /* Position slightly below the header */
  left: 0;
  width: 100%;
  margin: 0;
  padding: 8px 0;
  list-style: none;
  border: 1px solid var(--vp-c-border);
  border-radius: 6px;
  background-color: var(--vp-c-bg);
  box-shadow: var(--vp-shadow-3); /* Elevation effect */
  z-index: 50; /* High z-index to stay on top */
}

/* Individual Option Item Style */
.option-item {
  padding: 6px 16px;
  font-size: 14px;
  color: var(--vp-c-text-2);
  cursor: pointer;
  transition: background-color 0.15s, color 0.15s;
}

.option-item:hover {
  background-color: var(--vp-c-brand-soft); /* Light highlight on hover */
  color: var(--vp-c-text-1);
}

.option-item.is-selected {
  background-color: var(--vp-c-brand-soft);
  color: var(--vp-c-brand-1);
  font-weight: 600;
}
</style>
