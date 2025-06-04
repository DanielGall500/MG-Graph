<script setup lang="ts">
import { ref } from "vue";
import Card from 'primevue/card';

const visible = ref(false);

const db_port = ref("");
const db_name = ref("");
const db_username = ref("");
const db_password = ref("");

async function save_db_auth_details() {
    visible.value = false;
    try {
        const response = await fetch('http://127.0.0.1:8000/store-db-auth', { // Adjust the URL as necessary
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                username: db_port.value,
                password: db_name.value,
                db_name: db_username.value,
                db_port: db_password.value
             }), 
        });
    } catch (error) {
        console.error('Error saving text:', error);
    }
}

</script>

<template>
    <div class="examples">
        <h2 class="font-medium">Neo4J Authentication Settings</h2>
        <p class="flex-wrap font-medium m-0">
            Here you can set up the authentication details for Neo4J.<br> 
        </p>

        <Divider />

    <Button label="Neo4J Authentication" @click="visible = true" />

    <Dialog :closable="false" :visible="visible" modal header="Neo4J Authentication" :style="{ width: '25rem' }">
        <div class="flex flex-column items-center gap-4 mb-4">
            <label for="title" class="font-semibold w-24">Neo4J Username</label>
            <InputText id="title" class="flex-auto" v-model="db_username" autocomplete="off" />
        </div>
        <div class="flex flex-column gap-4 mb-4">
            <label for="lang" class="font-semibold w-24">Neo4J Password</label>
            <InputText id="lang" class="flex-auto" v-model="db_password" autocomplete="off" />
        </div>
        <div class="flex flex-column items-center gap-4 mb-4">
            <label for="grammar" class="font-semibold w-24">Database Name</label>
            <InputText id="new_grammar_text" v-model="db_name" autoResize rows="10" cols="50" />
        </div>
        <div class="flex flex-column items-center gap-4 mb-4">
            <label for="grammar" class="font-semibold w-24">Port</label>
            <InputText id="new_grammar_text" v-model="db_port" autoResize rows="10" cols="50" />
        </div>
        <div class="flex justify-end gap-2">
            <Button type="button" label="Cancel" severity="secondary" @click="visible = false"></Button>
            <Button type="button" label="Save" @click="save_db_auth_details"></Button>
        </div>
    </Dialog>
    </div>
</template>

