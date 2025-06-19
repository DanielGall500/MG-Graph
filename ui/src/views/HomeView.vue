<script setup lang="ts">
import { ref } from "vue";
import GraphVis from '@/components/GraphVis.vue';
import { useToast } from 'primevue/usetoast';
import { Form } from '@primevue/forms';

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

// Pathways
const path_start_node = ref("d");
const path_end_node = ref("t");
const all_pathways = ref("");
const shortest_pathways = ref("");

// Minimum Description Length
const mdl_alphabet_size = ref(26);
const mdl_num_types = ref(7);

const mdl_num_features = ref(0);
const mdl_num_phonemes = ref(0);
const mdl_enc_per_symbol = ref(0);

const mdl_metrics = ref([
    {
        "metric": "Number of Phonemes",
        "value": -1
    },
    {
        "metric": "Number of Features",
        "value": -1
    },
    {
        "metric": "Encoding Cost Per Symbol",
        "value": -1
    },
    {
        "metric": "Size (bits)",
        "value": -1
    }
])

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

async function reload() {
    await graph_vis.value.reload_vis();
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
            body: JSON.stringify({ 
                grammar: mgTextValue.value, 
                alphabet_size: mdl_alphabet_size.value,
                num_types: mdl_num_types.value
            }), // Send the grammar to the backend
        });
        const data = await response.json();

        // update the frontend
        clearGrammarTextBox()
        await reload();

        // set the updated values for grammar
        mdl_metrics.value[0]["value"] = data.size.n_phonemes;
        mdl_metrics.value[1]["value"] = data.size.n_features;
        mdl_metrics.value[2]["value"] = Math.round(data.size.encoding_cost_per_symbol);
        mdl_metrics.value[3]["value"] = Math.round(data.size.mdl);
        setMGSize(data.size.mdl);

        /*
        Next Step: Show other MDL bits
        */

        await get_pathways();

        await getMGJson();

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
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                start_item: path_start_node.value,
                end_item: path_end_node.value,
            }),
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
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                alphabet_size: mdl_alphabet_size.value,
                num_types: mdl_num_types.value
            }), // Send the grammar to the backend
        });
        const size_data = await size_response.json();
        setMGSize(size_data.size);
        setGrammarTextBox(size_data.grammar);

        // update the frontend
        decomp_suggestions.value = [];

        await get_pathways();
        getMGJson();

        showMessage("Success!", `Decomposition of ${affix} Successful.`, false);
        await reload();

        return "Success!"
    } catch (error: any) {
        console.error('Error:', error);
        showMessage("Error!", `Decomposition of ${affix} Failed.`, true);
        return "Failed."
    }
}

const onCombineStates = async (): Promise<string> => {
    showInfoMessage("Combining states..", "Attempting two combine the two given states.");
    try {
        // communicate with backend MG API
        const response = await fetch('http://127.0.0.1:8000/combine', { 
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                state_a: state_a_combine.value,
                state_b: state_b_combine.value,
             }), 
        });

        if (!response.ok) {
            const errorData = await response.json();
            console.error('Server Error:', errorData.error);
            showMessage("Combine States Failed", errorData.error, true);
            return "Failed"
        }

        // const build_mg_data = await build_mg_response.json();

        const size_response = await fetch('http://127.0.0.1:8000/calculate-size', { 
            method: 'POST',
            headers: {
            'Content-Type': 'application/json',
            },
            body: JSON.stringify({ 
                alphabet_size: mdl_alphabet_size.value,
                num_types: mdl_num_types.value
            }), // Send the grammar to the backend
        });
        const size_data = await size_response.json();
        setMGSize(size_data.size);
        setGrammarTextBox(size_data.grammar);

        // update the frontend
        decomp_suggestions.value = [];

        await get_pathways();
        getMGJson();

        showMessage("Success!", `Combination Successful.`, false);
        await reload();

        return "Success!"
    } catch (error: any) {
        console.error('Error:', error);
        showMessage("Error!", error, true);
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
            <div class="flex items-start gap-4 p-3 border-round border-1 surface-border" style="justify-content: center;">
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
                        praise :: =d =d +k v;
                        <br><br>
                    </p>

                    <h2>What is the size of an MG?</h2>
                    <p>The size of an MG is measured using the Minimum Description Length. This can be defined as follows:</p>
                    <math xmlns="http://www.w3.org/1998/Math/MathML" display="block">
                        <munder>
                            <mo>∑</mo>
                            <mrow>
                            <mi>s</mi>
                            <mo>::</mo>
                            <mi>δ</mi>
                            <mo>∈</mo>
                            <mi>Lex</mi>
                            </mrow>
                        </munder>
                        <mrow>
                            <mo>(</mo>
                            <mrow>
                            <mo>|</mo><mi>s</mi><mo>|</mo>
                            <mo>+</mo>
                            <mn>2</mn><mo>×</mo><mo>|</mo><mi>δ</mi><mo>|</mo>
                            <mo>+</mo>
                            <mn>1</mn>
                            </mrow>
                            <mo>)</mo>
                            <mo>×</mo>
                            <mrow>
                            <msub>
                                <mi>log</mi>
                                <mn>2</mn>
                            </msub>
                            <mo>(</mo>
                            <mo>|</mo><mi>Σ</mi><mo>|</mo>
                            <mo>+</mo>
                            <mo>|</mo><mi>Types</mi><mo>|</mo>
                            <mo>+</mo>
                            <mo>|</mo><mi>Base</mi><mo>|</mo>
                            <mo>+</mo>
                            <mn>1</mn>
                            <mo>)</mo>
                            </mrow>
                        </mrow>
                        <mo>.</mo>
                    </math>
                    <p>We let |s| be the length of a given string and |δ| be the number of features.</p>
                    <p>Types = {category, right selector, left selector, morphological selector, overt licensor, covert licensor, licensee}</p>
                    <p>Σ is the set of characters in an alphabet, for instance in English this is equal to 26.</p>
                    <p>|Base| is the total number of categories.</p>
                    <p>The first part determines the total number of symbols used, while the second part determines the cost of encoding a symbol.</p>

                </div>

                <div class="flex flex-column items-start gap-4 p-3 border-round border-1 surface-border">
                    <div class="flex flex-column">
                        <h2>MG Builder</h2>
                        <label class="flex-wrap m-0 pb-1 mb-1">
                            Depending on the language you're parsing, you may need to adjust the alphabet size.
                            <br>
                            The default alphabet size is for English.
                            <br>
                            Types correspond to the standard for Lexical Decomposition.
                        </label>
                        <br>
                        <div class="flex flex-row">
                            <div class="flex flex-column" style="margin-right: 2vw; align-items: center;">
                                <label class="mb-3 gap-3">|Σ|</label>
                                <InputNumber v-model="mdl_alphabet_size" showButtons buttonLayout="vertical" style="width: 3rem" :min="0" :max="99">
                                    <template #incrementbuttonicon>
                                        <span class="pi pi-plus" />
                                    </template>
                                    <template #decrementbuttonicon>
                                        <span class="pi pi-minus" />
                                    </template>
                                </InputNumber>
                            </div>
                            <div class="flex flex-column" style="margin-left: 2vw; align-items: center;">
                                <label class="mb-3 gap-3">|Types|</label>
                                <InputNumber v-model="mdl_num_types" showButtons buttonLayout="vertical" style="width: 3rem" :min="0" :max="99">
                                    <template #incrementbuttonicon>
                                        <span class="pi pi-plus" />
                                    </template>
                                    <template #decrementbuttonicon>
                                        <span class="pi pi-minus" />
                                    </template>
                                </InputNumber>
                            </div>
                            <div class="flex flex-column" style="justify-content: center; margin-left: 4vw;">
                                    <div class="flex flex-column" style="margin-bottom: 2vh; align-items: center;justify-content: center;;">
                                        <label for="over_label" >Start Item</label>
                                        <InputText class="w-5rem" id="over_label" v-model="path_start_node" />
                                    </div>
                                    <div class="flex flex-column" style="align-items: center; justify-content: left;">
                                        <label for="over_label" >End Item</label>
                                        <InputText class="w-5rem" id="over_label" v-model="path_end_node" />
                                    </div>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-column">
                        <div class="flex flex-row">
                        <Textarea v-model="mgTextValue" autoResize rows="15" cols="50" />
                        </div>
                        <div class="flex flex-row gap-3 mt-1" style="width: 30vw;">
                            <Button label="Parse MG" class="w-full" @click="submitGrammar"/>
                            <Button label="Cancel" severity="secondary" outlined class="w-full" @click="clearGrammarTextBox"/>
                        </div>
                    </div>

                </div>


            </div>

            <div class="flex flex-column">
                <div class="flex flex-row" style="justify-content: center;">
                    <Panel v-if="mgSize" class="flex flex-column gap-4 mt-5 px-4" style="padding-top: 5%;">
                        <div>
                            <h2 class="block font-semibold mb-2">MG => JSON</h2>
                            <Textarea v-model="mgAsRawJson" rows="30" cols="50" class="w-full" style="resize: none;" />
                        </div>
                    </Panel>
                    <Panel v-if="mgSize" class="flex flex-column gap-4 mt-5 px-4" style="padding-top: 5%;">
                        <h2 class="block text-md font-semibold mb-2">Metrics & Pathways</h2>
                        <br>
                        <DataTable :value="mdl_metrics" tableStyle="min-width: 50rem">
                            <Column field="metric" header="Metric"></Column>
                            <Column field="value" header="Value"></Column>
                        </DataTable>
                        <br>

                        <div class="p-grid p-gap-4">
                            <h2 class="block text-md font-semibold mb-2">All Pathways</h2>
                            <br>
                            <div class="p-col-12 p-md-6" v-if="all_pathways && all_pathways.length">
                                <ul class="p-m-0 p-pl-3">
                                    <li v-for="(item, index) in all_pathways" :key="'all-' + index">
                                        {{ path_start_node }} => {{ item }} => {{ path_end_node }}
                                    </li>
                                </ul>
                            </div>

                            <h2 class="block text-md font-semibold mb-2">Shortest Pathways</h2>
                            <br>
                            <div class="p-col-12 p-md-6" v-if="shortest_pathways && shortest_pathways.length">
                                <ul class="p-m-0 p-pl-3">
                                    <li v-for="(item, index) in shortest_pathways" :key="'shortest-' + index">
                                        {{ path_start_node }} => {{ item }} => {{ path_end_node }}
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </Panel>
                </div>
                <div class="flex flex-row gap-4 mt-5 px-4" style="justify-content: center;">
                    <Panel>
                        <label class="block text-md font-semibold mb-2">Visualisation</label>
                        <GraphVis ref="graph_vis" />
                    </Panel>
                </div>

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
                            <!-- State A & B Inputs -->
                            <div class="p-fluid flex flex-row grid gap-2 mb-4">
                                <div class="">
                                    <InputText v-model="state_a_combine" placeholder="State A" />
                                </div>
                                <div class="">
                                    <InputText v-model="state_b_combine" placeholder="State B" />
                                </div>
                            </div>
                            <Button class="col-1" @click="onCombineStates" type="submit" severity="secondary" label="Combine States" />
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
