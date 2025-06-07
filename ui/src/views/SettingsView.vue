<script setup lang="ts">
import { ref } from "vue";
import { useToast } from 'primevue/usetoast';
import Panel from 'primevue/panel';

const visible = ref(false);

const db_addr = ref("");
const db_name = ref("");
const db_username = ref("");
const db_password = ref("");

const db_auth_stored = ref("");
const valid_connection = ref("");
const toast = useToast();

function showMessage(summary: string, detail: string, is_error: boolean) {
    const sev = is_error ? "error" : "success";
    toast.add({
        severity: sev,  
        summary: summary,
        detail: detail,
        life: 5000 
    });
};

function toast_testing_connection() {
    toast.add({
        severity: 'info',        
        summary: 'This may take a minute.',
        detail: 'Checking database connection...',
        life: 5000,                 
    });
}

async function save_db_auth_details() {
    visible.value = false;
    const response = await fetch('http://127.0.0.1:8000/store-db-auth', { // Adjust the URL as necessary
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ 
            db_addr: db_addr.value,
            db_name: db_name.value,
            username: db_username.value,
            password: db_password.value,
        }), 
    });

    if (response.ok) {
        showMessage("Neo4J Database Details Saved.", "Make sure to test your connection first.", false);
    }
    else {
        showMessage("Unable to save Neo4J Database Details.", "Please input your details again.", true);
    }

    db_auth_stored.value = await response.text();
}

async function test_db_connection() {
    visible.value = false;
    toast_testing_connection()

    const response = await fetch('http://127.0.0.1:8000/test-db-auth', { // Adjust the URL as necessary
        method: 'GET',
        headers: {
        'Content-Type': 'application/json',
        },
    });

    valid_connection.value = await response.text();
    if (response.ok) {
        showMessage("DB Authentication Successful.", "You are ready to create MG Graphs.", false);
    }
    else {
        showMessage("DB Authentication Failed.", "Please input your details again.", true);
    }
}

</script>

<template>
    <div class="examples">

        <Panel header="Neo4J Authentication Settings">
            <p class="m-0">
                Here you can set up the authentication details for Neo4J.<br> 
            </p>
            <Divider />

            <div class="flex gap-5">
                <Button label="Enter Neo4J Details" @click="visible = true" />

                <Dialog :closable="false" :visible="visible" modal header="Neo4J Authentication" :style="{ width: '25rem' }">
                    <div class="flex flex-column items-center gap-4 mb-4">
                        <label for="grammar" class="font-semibold w-24">Database Address</label>
                        <InputText id="new_grammar_text" v-model="db_addr" autoResize rows="10" cols="50" />
                    </div>
                    <div class="flex flex-column items-center gap-4 mb-4">
                        <label for="grammar" class="font-semibold w-24">Database Name</label>
                        <InputText id="new_grammar_text" v-model="db_name" autoResize rows="10" cols="50" />
                    </div>
                    <div class="flex flex-column items-center gap-4 mb-4">
                        <label for="title" class="font-semibold w-24">Neo4J Username</label>
                        <InputText id="title" class="flex-auto" v-model="db_username" autocomplete="off" />
                    </div>
                    <div class="flex flex-column gap-4 mb-4">
                        <label for="lang" class="font-semibold w-24">Neo4J Password</label>
                        <InputText id="lang" class="flex-auto" v-model="db_password" autocomplete="off" />
                    </div>
                    <div class="flex justify-end gap-2">
                        <Button type="button" label="Cancel" severity="secondary" @click="visible = false"></Button>
                        <Button type="button" label="Save" @click="save_db_auth_details"></Button>
                    </div>
                </Dialog>

                <Button label="Test Connection" @click="test_db_connection" />
            </div>
        </Panel>

        <Panel header="Local Server Setup">
            <p class="m-0">
                A local server is required in order to run the backend.<br> 
            </p>
            <Divider />
        </Panel>

    </div>
</template>

