<template>
    <div
        class="left-0 z-[1004] fixed h-screen bottom-0 flex pointer-events-auto shadow-sm sm:w-48 w-40 bg-stone-800"
    >
        <ul class="w-full menu flex flex-col text-white gap-2">
            <li><router-link to="/">Home</router-link></li>
            <li><router-link to="/settings">Settings</router-link></li>
        </ul>
    </div>
</template>

<script setup lang="ts"></script>

<style scoped></style>
