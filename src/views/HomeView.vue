<script setup lang="ts">
// Layouts
import CenterLayout from '@/layouts/CenterLayout.vue'

// Components
import VDiff from '@/components/VDiff.vue'

// Utilities
import { nextTick, onBeforeUnmount, onMounted, ref } from 'vue'

import DropdownVue from '@/components/VDropdown.vue'

import { Command } from '@tauri-apps/api/shell'
import { invoke } from '@tauri-apps/api'

function runCommand(command: string, path: string, args: string[]) {
    const process = new Command(command, args, {
        cwd: path
    })
    return process.execute()
}

const types = [
    {
        id: 1,
        name: 'STC',
        tc: '10+0.1',
        options: 'Hash=16 Use NNUE=true',
        threads: 1
    },
    {
        id: 2,
        name: 'LTC',
        tc: '60+0.6',
        options: 'Hash=64 Use NNUE=true',
        threads: 1
    },
    {
        id: 3,
        name: 'SMP STC',
        tc: '5+0.05',
        options: 'Hash=64 Use NNUE=true',
        threads: 8
    },
    {
        id: 4,
        name: 'SMP LTC',
        tc: '20+0.2',
        options: 'Hash=256 Use NNUE=true',
        threads: 8
    }
]

const bounds = [
    {
        id: 1,
        name: 'Standard STC',
        sprt_elo0: 0,
        sprt_elo1: 2
    },
    {
        id: 1,
        name: 'Standard LTC',
        sprt_elo0: 0.5,
        sprt_elo1: 2.5
    },
    {
        id: 2,
        name: 'Non Regression STC',
        sprt_elo0: -1.75,
        sprt_elo1: 0.25
    },
    {
        id: 3,
        name: 'Non Regression LTC',
        sprt_elo0: -1.75,
        sprt_elo1: 0.25
    }
]

const form = ref({
    type: types[0],
    bound: bounds[0],
    branch: '',
    description: ''
})

const path = localStorage.getItem('path') || ''
const changes = ref('')
const newBranch = ref(false)
const formSubmited = ref(false)

const diff = async () => {
    let command = new Command('git', ['diff'], {
        cwd: path
    })

    let output = await command.execute()

    if (output.stdout !== '') {
        changes.value = output.stdout
        return
    }

    if (!changes.value) {
        command = new Command('git', ['diff', '--staged'], {
            cwd: path
        })
        output = await command.execute()

        if (output.stdout !== '') {
            changes.value = output.stdout
            return
        }
    }
}

let nInterval: number

onMounted(() => {
    diff()
    nInterval = setInterval(diff, 1000)
})

onBeforeUnmount(() => {
    clearInterval(nInterval)
})

const submit = () => {
    formSubmited.value = true
    compile()
}

const consoleContainer = ref<HTMLElement | null>(null)

const liveConsole = ref<
    {
        line: string
        id: number
    }[]
>([])

const compile = () => {
    const command = new Command('make', undefined, {
        cwd: path + '/src'
    })

    let number = 0

    liveConsole.value = []

    command.stdout.on('data', (line: string) => {
        liveConsole.value.push({
            line,
            id: number++
        })

        if (consoleContainer.value) {
            consoleContainer.value.scrollTop = consoleContainer.value.scrollHeight
        }
    })

    command
        .execute()
        .then(() => {
            formSubmited.value = false
            error.value = false
            msg.value = 'Compilation successfull!'
        })
        .catch(() => {
            formSubmited.value = false
            error.value = true
            msg.value = 'Compilation failed!'
        })
        .finally(() => {
            if (newBranch.value) {
                createNewBranch()
            } else {
                commit()
            }
        })

    setInterval(() => {
        msg.value = ''
    }, 5000)
}

const createNewBranch = () => {
    console.log('Creating new branch')

    runCommand('git', path, ['checkout', '-b', form.value.branch])
        .then(() => {
            error.value = false
            msg.value = 'Branch created successfully!'
        })
        .catch(() => {
            console.log('Branch creation failed')
            error.value = true
            msg.value = 'Branch creation failed! Please check the branch name and try again.'
        })
        .finally(() => {
            commit()
        })

    setInterval(() => {
        msg.value = ''
    }, 5000)
}

const getCurrentBranchName = async () => {
    const result = await runCommand('git', path, ['rev-parse', '--abbrev-ref', 'HEAD'])
    return result.stdout
}

const commit = async () => {
    await runCommand('git', path, ['add', '.'])
    const bench = await getBench()
    await runCommand('git', path, ['commit', '-m', 'Bench: ' + bench, '-m', form.value.description])

    const curent = form.value.branch ? await getCurrentBranchName() : form.value.branch

    await runCommand('git', path, ['push', '--set-upstream', 'origin', curent])

    error.value = false
    msg.value = 'Push successfull!'
    setInterval(() => {
        msg.value = ''
    }, 5000)

    submitForm(bench)
}

const getMasterBench = async () => {
    const benchSearch = /(^|\s)[Bb]ench[ :]+([0-9]+)/gm

    const response = await fetch(
        'https://api.github.com/repos/official-stockfish/Stockfish/commits'
    )
    const commits = await response.json()

    for (let idx = 0; idx < commits.length; idx++) {
        const commit = commits[idx]
        if (!commit.commit) {
            return 0
        }

        const benchMatch = benchSearch.exec(commit.commit.message)
        if (benchMatch) {
            return Number(benchMatch[2])
        }
    }

    return 0
}

const getBench = async () => {
    const lines: string[] = await invoke('stockfish', { path: path + '/src' })

    return Number(lines[lines.length - 2].split(' ').slice(-1)[0])
}

const fishtestForm = {
    odds: 'off',
    base_tag: 'master',

    num_games: 800000,

    book: 'UHO_XXL_+0.90_+1.19.epd',
    book_depth: '8',

    auto_purge: false,
    throughput: 1000,
    priority: 0,
    adjudication: true,

    elo_model: 'normalized',
    stop_rule: 'sprt',

    new_tag: '',
    tc: '',
    threads: 1,
    base_options: '',
    new_options: '',
    info: '',
    base_signature: 0,
    new_signature: 0,
    tests_repo: ''
}

const submitForm = async (bench: number) => {
    fishtestForm.new_tag = form.value.branch
    fishtestForm.tc = form.value.type.tc
    fishtestForm.threads = form.value.type.threads
    fishtestForm.base_options = form.value.type.options
    fishtestForm.new_options = form.value.type.options
    fishtestForm.info = form.value.description

    fishtestForm.base_signature = await getMasterBench()
    fishtestForm.new_signature = bench
    fishtestForm.tests_repo = localStorage.getItem('testUrl') || ''

    formSubmited.value = false
}

const msg = ref('')
const error = ref(false)
</script>

<template>
    <Transition name="fly-in">
        <div v-if="msg" class="toast toast-top toast-end">
            <div class="alert alert-success" v-if="!error">
                <span>{{ msg }}</span>
            </div>
            <div class="alert alert-error" v-if="error">
                <span>{{ msg }}</span>
            </div>
        </div>
    </Transition>

    <CenterLayout>
        <h1 class="text-3xl font-bold">Fishtest Test Manager</h1>
        <div class="flex gap-2 h-full">
            <div class="mt-5 w-1/2">
                <VDiff :diff-text="changes" />

                <form class="my-5 flex flex-col gap-2" @submit.prevent="submit">
                    <h1 class="text-2xl font-bold">Submit Test</h1>

                    <label for="types"> Test type </label>
                    <DropdownVue id="types" :items="types" v-model="form.type" />

                    <label for="bounds"> SPRT Bounds </label>
                    <DropdownVue id="bounds" :items="bounds" v-model="form.bound" />

                    <div class="flex items-center">
                        <input
                            type="text"
                            class="p-2 rounded-md"
                            placeholder="Test Branch Name"
                            v-model="form.branch"
                        />
                        <input type="checkbox" class="checkbox ml-2" v-model="newBranch" />
                        <label for="checkbox" class="ml-2"> Create new branch </label>
                    </div>

                    <textarea
                        class="p-2 rounded-md"
                        placeholder="Test Description"
                        v-model="form.description"
                    />
                    <input
                        type="submit"
                        value="Submit"
                        class="btn bg-emerald-600 hover:bg-emerald-800 border-none text-white font-bold py-2 px-4 rounded w-full"
                        :disabled="formSubmited"
                    />
                    <span v-if="formSubmited" class="loading loading-spinner loading-sm"></span>
                </form>
            </div>
            <div class="mt-5 w-1/2 h-full">
                <div
                    class="p-5 flex flex-col h-64 w-full bg-stone-700 overflow-y-scroll"
                    ref="consoleContainer"
                >
                    <span v-for="line in liveConsole" :key="line.id">{{ line.line }}</span>
                </div>
            </div>
        </div>
    </CenterLayout>
</template>

<style scoped>
.fly-in-enter-active {
    animation: fly-in 0.2s forwards;
}

.fly-in-leave-active {
    animation: fade-out 0.2s forwards;
}

@keyframes fly-in {
    from {
        transform: translateX(100%);
    }
    to {
        transform: translateX(0);
    }
}

@keyframes fade-out {
    from {
        opacity: 1;
    }
    to {
        opacity: 0;
    }
}
</style>
