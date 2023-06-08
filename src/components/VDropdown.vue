<template>
    <div class="top-16 lg:w-72 md:w-48 w-32">
        <Listbox v-model="selected">
            <div class="relative mt-1">
                <ListboxButton
                    class="relative w-full cursor-default rounded-t-lg bg-stone-700 text-white py-2 pl-3 pr-10 text-left shadow-md focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300 sm:text-sm"
                >
                    <span class="block truncate">{{ selected.name }}</span>
                </ListboxButton>

                <transition
                    leave-active-class="transition duration-100 ease-in"
                    leave-from-class="opacity-100"
                    leave-to-class="opacity-0"
                >
                    <ListboxOptions
                        class="absolute max-h-60 w-full overflow-auto bg-stone-700 py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm z-50"
                    >
                        <ListboxOption
                            v-slot="{ active, selected }"
                            v-for="element in items"
                            :key="element.name"
                            :value="element"
                            as="template"
                        >
                            <li
                                :class="[
                                    active ? 'bg-amber-100 text-amber-900' : 'text-white',
                                    'relative cursor-default select-none py-2 pl-10 pr-4 '
                                ]"
                            >
                                <span
                                    :class="[
                                        selected ? 'font-medium' : 'font-normal',
                                        'block truncate'
                                    ]"
                                    >{{ element.name }}</span
                                >
                            </li>
                        </ListboxOption>
                    </ListboxOptions>
                </transition>
            </div>
        </Listbox>
    </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { Listbox, ListboxButton, ListboxOptions, ListboxOption } from '@headlessui/vue'

const props = defineProps<{
    items: { name: string; id: number }[]
}>()

const selected = ref(props.items[0])

const emit = defineEmits(['update:modelValue'])

watch(selected, () => {
    emit('update:modelValue', selected.value)
})
</script>
