<script setup lang="ts">
import { ref, onMounted } from "vue";
import Card from 'primevue/card';

</script>

<template>
    <div class="examples">
        <h2 class="font-medium">My Grammars</h2>

        <div class="p-4">
            <!-- Display existing grammars -->
            <div v-for="(grammar, index) in grammars" :key="index" class="mb-2">
            <p>{{ grammar }}</p>
            </div>

            <!-- Form to add new grammar -->
            <input v-model="newGrammar" placeholder="Enter new grammar" class="p-2 border rounded mb-2" />
            <button @click="addGrammar" class="p-2 bg-blue-500 text-white rounded">Add Grammar</button>
        </div>

    </div>
</template>

