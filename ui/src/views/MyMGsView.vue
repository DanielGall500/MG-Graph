<script setup lang="ts">
import { ref, onMounted } from "vue";
import Card from 'primevue/card';
import Dialog from "primevue/dialog";
import { useToast } from 'primevue/usetoast';

interface MGExample {
  title: string;
  lang: string,
  grammar: string
}

const grammars = ref<MGExample[]>([]);
const new_grammar_title = ref("");
const new_grammar_lang = ref("");
const new_grammar_text = ref("");
const visible = ref(false);
const toast = useToast();

function showMessage(summary: string, detail: string, is_error: boolean) {
    const sev = is_error ? "error" : "success";
    toast.add({
        severity: sev,  
        summary: summary,
        detail: detail,
        life: 3000 // Display time in milliseconds
    });
};

function showInfoMessage(summary: string, detail: string) {
    toast.add({
        severity: "info",  
        summary: summary,
        detail: detail,
        life: 5000 // Display time in milliseconds
    });
}

const is_valid_mg = (mg: string): boolean => {
    return mg
    .split(";")
    .filter(lexical_item => lexical_item.length > 1)
    .every(lexical_item => lexical_item.includes("::"));
}

function parse_saved_mg(mg: string) {
    return mg
    .split(";")
    .map(
        item => item.trim().concat(";")
    )
    .filter(lexical_item => lexical_item.length > 1)
}

async function saveGrammar() {
    visible.value = false;
    const mg = new_grammar_text.value;
    if (!is_valid_mg(mg)) {
        showMessage("MG Invalid", "Please check your syntax.", true);
    }
    else {
        const new_grammar_as_list = parse_saved_mg(new_grammar_text.value);

        try {
            const response = await fetch('http://127.0.0.1:8000/store-mg', { // Adjust the URL as necessary
                method: 'POST',
                headers: {
                'Content-Type': 'application/json',
                },
                body: JSON.stringify({ 
                    title: new_grammar_title.value,
                    lang: new_grammar_lang.value,
                    grammar: new_grammar_as_list
                }), 
            });

            const result = await response.text();
            await loadText();
            showMessage("MG Added", "Your MG has been saved.", false);
        }
        catch (error) {
            showMessage("MG Couldn't Be Stored", "There appears to be a local server issue.", true);
        }
    }
}

async function loadText() {
    try {
        const response = await fetch('http://127.0.0.1:8000/load-mg-collection', { // Adjust the URL as necessary
            method: 'GET',
            headers: {
            'Content-Type': 'application/json',
            },
        });


        if (!response.ok) {
            throw new Error(`Failed to load text: ${response.statusText}`);
        }

        grammars.value = await response.json();
    } catch (error) {
        console.error('Error loading text:', error);
    }
}

onMounted(() => {
    loadText();
});

</script>

<template>
    <div class="examples">

        <Panel header="My MGs">
            <p class="flex-wrap font-medium m-0">
                This page contains your very own MGs which you can create and save.<br> 
                They will be stored on your device and you can access them at any time.
            </p>
            <Divider />
        </Panel>

        <div class="p-4 grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <Card v-for="(grammar, index) in grammars" :key="index" 
                class="transition duration-200 hover:shadow-xl hover:bg-gray-100 cursor-pointer shadow-lg rounded-2xl">
            <template #title>
                <div class="text-xl font-semibold">{{ grammar.title }}</div>
            </template>

            <template #subtitle>
                <div class="text-sm text-gray-500">{{ grammar.lang }}</div>
            </template>

            <template #content>
                <div v-for="(line, i) in grammar.grammar" :key="i" class="text-sm text-gray-500">{{ line }} <br> </div>
            </template>
            </Card>
        </div>

    <Button label="New MG" @click="visible = true" />

<Dialog :closable="false" :visible="visible" modal header="New Grammar" :style="{ width: '25rem' }">
    <div class="flex flex-column items-center gap-4 mb-4">
        <label for="title" class="font-semibold w-24">Title</label>
        <InputText id="title" class="flex-auto" v-model="new_grammar_title" autocomplete="off" />
    </div>
    <div class="flex flex-column gap-4 mb-8">
        <label for="lang" class="font-semibold w-24">Language</label>
        <InputText id="lang" class="flex-auto" v-model="new_grammar_lang" autocomplete="off" />
    </div>
    <div class="flex flex-column items-center gap-4 mb-8">
        <label for="grammar" class="font-semibold w-24">Grammar</label>
        <Textarea id="new_grammar_text" v-model="new_grammar_text" autoResize rows="10" cols="50" />
    </div>
    <div class="flex justify-end gap-2">
        <Button type="button" label="Cancel" severity="secondary" @click="visible = false"></Button>
        <Button type="button" label="Save" @click="saveGrammar"></Button>
    </div>
</Dialog>

    </div>
</template>

