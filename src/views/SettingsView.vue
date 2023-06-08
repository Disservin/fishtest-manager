<template>
    <CenterLayout>
        <div class="flex flex-col gap-2">
            <div>
                <h1 class="text-xl font-bold">Stockfish Repository</h1>
            </div>
            <div>
                <div>
                    <span> Selected: {{ selected }} </span>
                </div>
                <button
                    type="button"
                    class="file-input w-full max-w-xs bg-stone-800 outline-none mt-1"
                    @click="selectPath"
                >
                    Change
                </button>
            </div>
            <div class="divider" />
            <div>
                <h1 class="text-xl font-bold">URL of Test Repository</h1>
            </div>
            <div>
                <input
                    type="text"
                    id="username"
                    class="input bg-stone-800 outline-none mt-1"
                    v-model="selectedTestUrl"
                    @change="updateTestUrl"
                />
            </div>
            <div class="divider" />
            <div>
                <h1 class="text-xl font-bold">Credentials</h1>
            </div>
            <div>
                <div>
                    <span> {{ loggedIn ? 'You are logged in!' : 'Please log in!' }} </span>
                </div>
                <div class="flex justify-start">
                    <label for="username" class="w-24">Username</label>
                    <input
                        type="text"
                        id="username"
                        class="input w-32 max-w-xs bg-stone-800 outline-none mt-1"
                    />
                </div>

                <div class="flex justify-start">
                    <label for="password" class="w-24">Password</label>
                    <input
                        type="password"
                        id="password"
                        class="input w-32 max-w-xs bg-stone-800 outline-none mt-1"
                    />
                </div>
                <input
                    type="submit"
                    value="Login"
                    class="btn bg-emerald-600 hover:bg-emerald-800 text-white py-1 px-2 rounded"
                    @click="login"
                />
                <div>
                    <span v-if="error"> Something went wrong! Please try again. </span>
                </div>
            </div>
        </div>
    </CenterLayout>
</template>

<script setup lang="ts">
import CenterLayout from '@/layouts/CenterLayout.vue'
import { open } from '@tauri-apps/api/dialog'
import { isArray } from '@vue/shared'
import { onMounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api'

onMounted(() => {
    invoke('get_user').then((response) => {
        loggedIn.value = true
    })

    const stored = localStorage.getItem('path')
    if (stored) {
        selected.value = stored
    }
})

const selected = ref<string>('None')
const selectedTestUrl = ref<string>('None')

const selectPath = async () => {
    const path = await open({
        directory: true
    })

    if (path != null && !isArray(path)) {
        selected.value = path
        localStorage.setItem('path', path)
    }
}

const updateTestUrl = () => {
    localStorage.setItem('testUrl', selectedTestUrl.value)
}

const loggedIn = ref(false)
const error = ref(false)

const login = () => {
    invoke('save_user', { username: 'disservin', password: '123456' }).catch((error) => {
        error.value = true
    })
}
</script>

<style scoped></style>
