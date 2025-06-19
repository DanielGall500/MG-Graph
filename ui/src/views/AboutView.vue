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
    </div>
</template>

