<script setup lang="ts">
import { ref } from "vue";
import GraphVis from '@/components/GraphVis.vue'
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

function setGrammarTextBox(grammar: string) {
    mgTextValue.value = grammar;
}

function clearGrammarTextBox() {
    mgTextValue.value = "";
}

function setMGSize(size: number) {
    mgSize.value = size;
}

function switchTab(tab: number) {
    activeTab.value = tab;
}

function reload() {
    graph_vis.value.reload_vis();
}

const submitGrammar = async (): Promise<string> => {
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
        switchTab(2);
        reload();

        // set the updated values for grammar
        setMGSize(data.size);
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
        showMessage("Success!", `Decomposition of ${affix} Successful.`, false);
        switchTab(2);
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
        showMessage("Success!", `Combination Successful.`, false);
        switchTab(2);
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
  <div class="flex flex-wrap">
  <Toast />
  <div class="card flex justify-content-left">
          <div class="flex flex-column h-full">
              <div class="flex align-items-center justify-content-between px-4 pt-3 flex-shrink-0">
                  <span class="inline-flex align-items-center gap-2">
                      <svg width="35" height="40" viewBox="0 0 35 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                          <path
                              d="M25.87 18.05L23.16 17.45L25.27 20.46V29.78L32.49 23.76V13.53L29.18 14.73L25.87 18.04V18.05ZM25.27 35.49L29.18 31.58V27.67L25.27 30.98V35.49ZM20.16 17.14H20.03H20.17H20.16ZM30.1 5.19L34.89 4.81L33.08 12.33L24.1 15.67L30.08 5.2L30.1 5.19ZM5.72 14.74L2.41 13.54V23.77L9.63 29.79V20.47L11.74 17.46L9.03 18.06L5.72 14.75V14.74ZM9.63 30.98L5.72 27.67V31.58L9.63 35.49V30.98ZM4.8 5.2L10.78 15.67L1.81 12.33L0 4.81L4.79 5.19L4.8 5.2ZM24.37 21.05V34.59L22.56 37.29L20.46 39.4H14.44L12.34 37.29L10.53 34.59V21.05L12.42 18.23L17.45 26.8L22.48 18.23L24.37 21.05ZM22.85 0L22.57 0.69L17.45 13.08L12.33 0.69L12.05 0H22.85Z"
                              fill="var(--primary-color)"
                          />
                          <path
                              d="M30.69 4.21L24.37 4.81L22.57 0.69L22.86 0H26.48L30.69 4.21ZM23.75 5.67L22.66 3.08L18.05 14.24V17.14H19.7H20.03H20.16H20.2L24.1 15.7L30.11 5.19L23.75 5.67ZM4.21002 4.21L10.53 4.81L12.33 0.69L12.05 0H8.43002L4.22002 4.21H4.21002ZM21.9 17.4L20.6 18.2H14.3L13 17.4L12.4 18.2L12.42 18.23L17.45 26.8L22.48 18.23L22.5 18.2L21.9 17.4ZM4.79002 5.19L10.8 15.7L14.7 17.14H14.74H15.2H16.85V14.24L12.24 3.09L11.15 5.68L4.79002 5.2V5.19Z"
                              fill="var(--text-color)"
                          />
                      </svg>
                      <span class="font-semibold text-2xl text-primary">MG-Graph</span>
                  </span>
                  <span>
                      <Button type="button" @click="visible = false" icon="pi pi-times" rounded outlined class="h-2rem w-2rem"></Button>
                  </span>
              </div>
              <div class="overflow-y-auto">
                  <ul class="list-none p-3 m-0">
                      <li>
                          <ul class="list-none p-0 m-0 overflow-hidden">
                              <li>
                                  <a v-ripple class="flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                      <i class="pi pi-home mr-2"></i>
                                      <span class="font-medium">Dashboard</span>
                                  </a>
                              </li>
                              <li>
                                  <a v-ripple class="flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                      <i class="pi pi-bookmark mr-2"></i>
                                      <span class="font-medium">Examples</span>
                                  </a>
                              </li>
                              <li>
                                  <a v-ripple class="flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                      <i class="pi pi-users mr-2"></i>
                                      <span class="font-medium">Saved Minimalist Grammars</span>
                                  </a>
                              </li>
                              <li>
                                  <a v-ripple class="flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                      <i class="pi pi-calendar mr-2"></i>
                                      <span class="font-medium">About</span>
                                  </a>
                              </li>
                          </ul>
                      </li>
                  </ul>
              </div>
              <div class="mt-auto">
                  <hr class="mb-3 mx-3 border-top-1 border-none surface-border" />
                  <a v-ripple class="m-3 flex align-items-center cursor-pointer p-3 gap-2 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                      <Avatar image="https://primefaces.org/cdn/primevue/images/avatar/amyelsner.png" shape="circle" />
                      <span class="font-bold">Daniel Gallagher</span>
                  </a>
              </div>
        </div>
        <Divider layout="vertical"/>
      </div>

    <TabView @tab-change="reload" :active-index="activeTab">
        <!-- Editor Tab -->
        <TabPanel header="Editor" :activeIndex="activeTab">
            <div class="flex justify-content-right">
                <Card style="width: 80rem; overflow: hidden">
                    <template #title>Minimalist Grammar Editor</template>
                    <template #content>
                    <p class="m-0">
                        Input your grammar below and submit in order to generate its di-graph representation. 
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
                        <br><br>
                        You can additionally choose the algorithm you would like to use to calculate the grammar size.
                    </p>

                    <Divider />

                    <div class="flex justify-content-center" style="flex-direction: column; gap: 25px;">
                        <div>
                            <Dropdown v-model="selectedCity" :options="sizeAlgorithms" optionLabel="name" placeholder="Size Algorithm" checkmark :highlightOnSelect="false" class="w-full md:w-14rem" />
                        </div>
                            <Textarea v-model="mgTextValue" autoResize rows="20" cols="10" />
                        <div>
                        </div>
                    </div>
                    </template>
                    <template #footer>
                    <div class="flex gap-3 mt-1" style="width: 30em;">
                        <Button label="Cancel" severity="secondary" outlined class="w-full" @click="clearGrammarTextBox"/>
                        <Button label="Submit" class="w-full" @click="submitGrammar"/>
                        <p>{{ responseNotification }}</p>
                    </div>
                    </template>
                </Card>
            </div>
        </TabPanel>

        <TabPanel header="Decomposition" :activeIndex="activeTab">
            <div class="flex justify-content-right">
                <Card style="width: 80rem; overflow: hidden">
                    <template #title>Perform Lexical Decomposition</template>
                    <template #content>
                    <Divider />
                    <h1>Your items are here...</h1>
                    <div v-if="loading_decomp_suggestions"><p>Loading...</p></div>
                    <p>Suggestions:</p>
                    <div v-for="(li_vec, affix) in decomp_suggestions" :key="affix" >
                        <Button class="btn btn-light" @click="decompose($event, affix, li_vec)">{{ affix }}</Button>
                        <ul>
                            <li v-for="li in li_vec" :key="li">{{ li }}</li>
                        </ul>
                    </div>
                    <input v-model="state_a_combine" placeholder="State A" />
                    <input v-model="state_b_combine" placeholder="State B" />
                    <Button @click="onCombineStates" type="submit" severity="secondary" label="Submit" />
                    <Form v-slot="$form" @submit="onCombineStates" class="flex flex-col gap-4 w-full sm:w-56">
                        <div class="flex flex-col gap-1">
                            <InputText name="username" type="text" placeholder="Username" fluid />
                            <Message v-if="$form.username?.invalid" severity="error" size="small" variant="simple">{{ $form.username.error?.message }}</Message>
                        </div>
                        <Button type="submit" severity="secondary" label="Submit" />
                    </Form>
                    </template>
                    <template #footer>
                    <div class="flex gap-3 mt-1" style="width: 30em;">
                        <Button label="Cancel" severity="secondary" outlined class="w-full" />
                        <Button label="Calculate Decomposition Suggestions" class="w-full" @click="get_suggestions"/>
                    </div>
                    </template>
                </Card>
            </div>
        </TabPanel>

        <!-- Visualisation Tab -->
        <TabPanel header="Visualisation" :activeIndex="activeTab">
            <h1>MG-Graph Visualisation</h1>
            <h2>Size: {{  Math.round(mgSize)  }}</h2>
            <GraphVis ref="graph_vis"/>
        </TabPanel>
    </TabView>

    </div>
</template>
