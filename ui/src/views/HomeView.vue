<script setup lang="ts">
import { ref } from "vue";
import GraphVis from '@/components/GraphVis.vue';
import { useToast } from 'primevue/usetoast';
import { Form } from '@primevue/forms';

const selectedCity = ref();
const sizeAlgorithms = ref([
  { name: "Ermolaeva", code: "ERM" },
  { name: "Simple", code: "SP" },
]);
const visible = ref(false);
const mgTextValue = ref("");
const mgSize = ref(0);
const responseNotification = ref("");
const activeTab = ref(0);
const graph_vis = ref();
const decomp_suggestions = ref();
const loading_decomp_suggestions = ref(false);
const state_a_combine = ref("");
const state_b_combine = ref("");

const mgAsRawJson = ref("");

const toast = useToast();

const all_pathways = ref("");
const shortest_pathways = ref("");

/* TODO: put these in one place */
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

function setGrammarTextBox(grammar: string) {
    mgTextValue.value = grammar;
}

function clearGrammarTextBox() {
    mgTextValue.value = "";
}

function setMGSize(size: number) {
    mgSize.value = size;
}

function reload() {
    graph_vis.value.reload_vis();
}

const getMGJson = async () => {
    const response = await fetch('http://127.0.0.1:8000/get-mg-json', { // Adjust the URL as necessary
        method: 'GET',
        headers: {
        'Content-Type': 'application/json',
        },
    });
    if (response.ok) {
        const data = await response.text();
        mgAsRawJson.value = data;
    }
    else {
        showMessage("Unable to Retrieve JSON", "The MG in JSON format could not be shown.", true);
    }
}

const submitGrammar = async (): Promise<string> => {
    showInfoMessage("Processing MG...", "This may take a minute.");
    try {
        // communicate with backend MG API
        const response = await fetch('http://127.0.0.1:8000/build-initial-mg', { // Adjust the URL as necessary
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({ grammar: mgTextValue.value }), // Send the grammar to the backend
        });
        const data = await response.json();

        // update the frontend
        clearGrammarTextBox()
        reload();

        // set the updated values for grammar
        setMGSize(data.size);

        get_pathways();

        getMGJson();

        showMessage("Success!", "Grammar successfully submitted.", false);
        return "Success!";
    } catch (error: any) {
        console.error('Error:', error);
        clearGrammarTextBox();
        showMessage("Error!", error, true);
        return "Failed.";
    }
}

/* TODO
Try to connect this up to get suggestions and display them to the user.
Then allow for decomposition to take place.
*/
const get_suggestions = async(): Promise<string> => {
    loading_decomp_suggestions.value = true;
    try {
        const response = await fetch('http://127.0.0.1:8000/decompose-suggestions', { 
            method: 'GET',
            headers: {
            'Content-Type': 'application/json',
            },
        });
        const data = await response.json();
        showMessage("Success!", "Suggestions Successfully Loaded.", false);
        decomp_suggestions.value = data.prefix_morph_map;
        loading_decomp_suggestions.value = false;
        return "Suggestions Found!"

    } catch (error: any) {
        loading_decomp_suggestions.value = false;
        showMessage("Decomposition Error!", error, true);
        return "No Suggestions Found."
    }
}

const get_pathways = async(): Promise<string> => {
    try {
        const response = await fetch('http://127.0.0.1:8000/pathways', { 
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        // Check if the response is OK and the body is not empty
        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const data = await response.json(); // Get the raw response text
        if (response.ok && data) {
            // const data = JSON.parse(responseText); // Parse it manually
            all_pathways.value = data.all_pathways;
            shortest_pathways.value = data.shortest_pathways;
            showMessage("Pathways Found!", "Pathways successfully found.", false);
            return "Pathways found!";
        } else {
            throw new Error("Empty response body");
        }
        
    } catch (error: any) {
        showMessage("Pathways Error!", error.message || error, true);
        return "No Suggestions Found";
    }
}

const decompose = async (event: any, affix: any, li_vec: any): Promise<string> => {
    try {
        // communicate with backend MG API
        const build_mg_response = await fetch('http://127.0.0.1:8000/decompose', { // Adjust the URL as necessary
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                affix: affix,
                split: 1,
             }), 
        });
        // const build_mg_data = await build_mg_response.json();

        const size_response = await fetch('http://127.0.0.1:8000/calculate-size', { // Adjust the URL as necessary
            method: 'GET',
            headers: {
            'Content-Type': 'application/json',
            },
        });
        const size_data = await size_response.json();
        setMGSize(size_data.size);
        setGrammarTextBox(size_data.grammar);

        // update the frontend
        decomp_suggestions.value = [];

        getMGJson();

        showMessage("Success!", `Decomposition of ${affix} Successful.`, false);
        reload();

        return "Success!"
    } catch (error: any) {
        console.error('Error:', error);
        showMessage("Error!", `Decomposition of ${affix} Failed.`, true);
        return "Failed."
    }
}

const onCombineStates = async (): Promise<string> => {
    try {
        // communicate with backend MG API
        const build_mg_response = await fetch('http://127.0.0.1:8000/combine', { // Adjust the URL as necessary
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                state_a: state_a_combine.value,
                state_b: state_b_combine.value,
             }), 
        });

        // const build_mg_data = await build_mg_response.json();

        const size_response = await fetch('http://127.0.0.1:8000/calculate-size', { // Adjust the URL as necessary
            method: 'GET',
            headers: {
            'Content-Type': 'application/json',
            },
        });
        const size_data = await size_response.json();
        setMGSize(size_data.size);
        setGrammarTextBox(size_data.grammar);

        // update the frontend
        decomp_suggestions.value = [];

        getMGJson();

        showMessage("Success!", `Combination Successful.`, false);
        reload();

        return "Success!"
    } catch (error: any) {
        console.error('Error:', error);
        showMessage("Error!", `Combination Failed.`, true);
        return "Failed."
    }
}
</script>

<template>
  <div class="z-20 flex flex-wrap">
  <Toast />
    <TabView class="z-20" @tab-change="reload" :active-index="activeTab">
        <!-- Editor Tab -->
        <TabPanel header="Editor" :activeIndex="activeTab">
            <div class="flex justify-left items-start gap-4 p-3 border-round border-1 surface-border" style="justify-content: left;">
                <div class="flex flex-column items-start gap-4 p-3 border-round border-1 surface-border">
                    <label for="db-input" class="pb-1 mb-1 text-sm font-semibold">MG Input</label>
                    <Textarea v-model="mgTextValue" autoResize rows="10" cols="50" />
                        <div class="flex gap-3 mt-1" style="width: 30vw;">
                            <Button label="Submit" class="w-full" @click="submitGrammar"/>
                            <Button label="Cancel" severity="secondary" outlined class="w-full" @click="clearGrammarTextBox"/>
                            <p>{{ responseNotification }}</p>
                        </div>
                </div>
                <div class="text-left" style="width: 40%;">
                    <h2>Grammar Input</h2>
                    <p class="flex-wrap m-0">
                        Input your grammar and submit in order to generate its di-graph representation. 
                        Please use two colons to separate phonological forms from feature bundles and end each lexical item with a semi-colon.
                        For instance,
                        <br><br>
                        Mary :: d -k;
                        <br>
                        laugh :: =d v;
                        <br>
                        jump :: =d v;
                        <br>
                        -s :: =>v +k t;
                        <br>
                        -ed :: =>v +k t;
                        <br>
                        praise :: =d =d +k t;
                        <br><br>
                    </p>
                </div>


            </div>

            <div class="flex flex-column">
                <div class="flex flex-row gap-4 mt-5 px-4" style="justify-content: left;">
                    <div>
                        <label class="block text-sm font-semibold mb-2">Visualisation</label>
                        <GraphVis ref="graph_vis" />
                    </div>

                    <div>
                        <label class="block text-sm font-semibold mb-2">MG Details</label>
                        <label class="block text-sm font-semibold mb-2">Size: {{ Math.round(mgSize) }}</label>
                        <Textarea v-model="mgAsRawJson" rows="30" cols="50" class="w-full" style="resize: none;" />
                    </div>

                </div>

                <Panel class="flex flex-column gap-4 mt-5 px-4" style="padding-top: 5%;">
                    <div>
                        <h2>All Pathways</h2>
                        <ul>
                            <li v-for="(item, index) in all_pathways" :key="index">
                                {{ item }}
                            </li>
                        </ul>
                    </div>
                    <div>
                        <h2>Shortest Pathways</h2>
                        <ul>
                            <li v-for="(item, index) in shortest_pathways" :key="index">
                                {{ item }}
                            </li>
                        </ul>
                    </div>
                </Panel>
            </div>


        
        </TabPanel>

        <!-- Decomposition Page-->
        <TabPanel header="Decomposition" :activeIndex="activeTab">
            <div class="flex justify-content-right">
                <Card style="width: 80rem; overflow: hidden">
                    <template #title>Lexical Decomposition</template>
                    <template #content>
                        <p>Here you can decompose the grammar either with the help of suggestions which are generated based on the most common affixes or manually below.</p>
                    <Divider />

                    <div v-if="loading_decomp_suggestions"><p>Loading...</p></div>

                    <div class="card flex flex-row gap-3">
                        <div class="flex flex-row" v-for="(li_vec, affix) in decomp_suggestions" :key="affix" >
                            <Button rounded variant="outlined" @click="decompose($event, affix, li_vec)">{{ affix }}</Button>
                        </div>
                    </div>
 
                    <ButtonGroup class="flex gap-3 mt-1" style="width: 10vw;">
                    </ButtonGroup>

                    <ButtonGroup class="gap-3">
                        <Button label="Analyse" icon="pi pi-check" size="small" @click="get_suggestions"/>
                        <Button label="Delete" icon="pi pi-trash" size="small"/>
                    </ButtonGroup>

                    <Divider />

                    <div class="card flex flex-column gap-1">
                        <h2>Combine States Manually</h2>
                        <br>
                        <div class="flex flex-column">
                            <Form v-slot="$form" @submit="onCombineStates" class="p-fluid flex flex-row gap-4 sm:w-56">
                            <!-- State A & B Inputs -->
                            <div class="p-fluid flex flex-row grid gap-2 mb-4">
                                <div class="">
                                    <InputText v-model="state_a_combine" placeholder="State A" />
                                </div>
                                <div class="">
                                    <InputText v-model="state_b_combine" placeholder="State B" />
                                </div>
                            </div>
                            <div>
                                <Message
                                v-if="$form.username?.invalid"
                                severity="error"
                                size="small"
                                class="mt-1"
                                :closable="false"
                                >{{ $form.username.error?.message }}</Message
                                >
                            </div>
                            </Form>
                            <Button class="col-1" type="submit" severity="secondary" label="Combine States" />
                        </div>
                    </div>

                    <Divider />

                    </template>
                    <template #footer>
                    </template>
                </Card>
            </div>
        </TabPanel>

    </TabView>

    </div>
</template>
