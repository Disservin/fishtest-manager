<script setup lang="ts">
// Layouts
import CenterLayout from '@/layouts/CenterLayout.vue'

// Components
import VDiff from '@/components/VDiff.vue'

// Utilities
import { ref } from 'vue'

import DropdownVue from '@/components/VDropdown.vue'

const types = [
    {
        id: 1,
        name: 'STC'
    },
    {
        id: 2,
        name: 'LTC'
    },
    {
        id: 3,
        name: 'SMP STC'
    },
    {
        id: 4,
        name: 'SMP LTC'
    }
]

const bounds = [
    {
        id: 1,
        name: 'Standard'
    },
    {
        id: 2,
        name: 'Non Regression'
    }
]

const form = ref({
    type: types[0],
    bound: bounds[0],
    branch: '',
    description: ''
})

const submit = () => {
    console.log(form.value)
}
</script>

<template>
    <CenterLayout>
        <h1 class="text-3xl font-bold">Fishtest Test Manager</h1>
        <div class="mt-5">
            <VDiff />

            <form class="my-5 flex flex-col gap-2" @submit.prevent="submit">
                <h1 class="text-2xl font-bold">Submit Test</h1>

                <label for="types"> Test type </label>
                <DropdownVue id="types" :items="types" v-model="form.type" />

                <label for="bounds"> SPRT Bounds </label>
                <DropdownVue id="bounds" :items="bounds" v-model="form.bound" />

                <input
                    type="text"
                    class="p-2 rounded-md"
                    placeholder="Test Branch Name"
                    v-model="form.branch"
                    required
                />
                <textarea
                    class="p-2 rounded-md"
                    placeholder="Test Description"
                    v-model="form.description"
                />
                <input
                    type="submit"
                    value="Submit"
                    class="btn bg-emerald-600 hover:bg-emerald-800 text-white font-bold py-2 px-4 rounded"
                />
            </form>
        </div>
    </CenterLayout>
</template>
