<script setup lang="ts">
import { ref } from "vue";
import Dialog from "primevue/dialog";
const visible = ref(false);
</script>

<template>
    <div class="about">
        <h1>About</h1>
        <p>This is a simple application to visualize Neo4j data.</p>
        <p>It uses Vue.js for the frontend and Neo4j for the backend.</p>
        <p>To get started, check out the documentation on how to set up your environment.</p>

        <br>
        <Button label="Settings" @click="visible = true" />

        <Dialog v-model:visible="visible" modal header="Edit Profile" :style="{ width: '25rem' }">
            <span class="text-surface-500 dark:text-surface-400 block mb-8">Update your information.</span>
            <div class="flex items-center gap-4 mb-4">
                <label for="username" class="font-semibold w-24">Username</label>
                <InputText id="username" class="flex-auto" autocomplete="off" />
            </div>
            <div class="flex items-center gap-4 mb-8">
                <label for="email" class="font-semibold w-24">Password</label>
                <InputText id="password" class="flex-auto" autocomplete="off" />
            </div>
            <div class="flex justify-end gap-2">
                <Button type="button" label="Cancel" severity="secondary" @click="visible = false"></Button>
                <Button type="button" label="Save" @click="visible = false"></Button>
            </div>
        </Dialog>
    </div>
</template>

