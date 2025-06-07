<script setup lang="ts">
import { ref } from "vue";
import Card from 'primevue/card';

const grammars = ref([
  {
    name: "Auxiliaries",
    language: "English",
    mg:
        [
            "Mary :: x -k;",
            "laugh :: =x x;",
            "jump :: =x x;",
            "-s :: =>x +k t;",
            "-ed :: =>x +k t;"
        ]
  },
  { 
    name: "Auxiliaries", 
    language: "English", 
    mg: 
        [
            "Mary :: d -k;",
            "laughs :: =d +k t;",
            "laughed :: =d +k t;", 
            "jumps :: =d +k t;", 
            "jumped :: =d +k t;",  
        ]
  },
  { 
    name: "Auxiliaries", 
    language: "English", 
    mg: 
        [
            "Mary :: d -k;",
            "laugh :: =d v;",
            "jump :: =d v;",
            "-s :: =>v +k t;",
            "-ed :: =>v +k t;"
        ]
  },
  {
    name: "Agglutination I",
    language: "Kazakh",
    mg:
        [
            "Sayle :: d -k;",
            "",

            "jazyp :: d= v;",    // write CVB
            "juzip :: d= v;",    // swim CVB
            "",

            "jatyr :: v= +k t;", // PROG-3
            "tur :: v= +k t;",   // STAT-3
            "jur :: v= +k t;"    // ITER-3
        ]
  },
  { 
    name: "Agglutination II", 
    language: "Kazakh", 
    mg: 
        [
            "Sayle :: d -k;",
            "hat :: d -k;",
            "",

            "{TP} :: v= +k TP;",                  // subject pulled up
            "-dy :: AspP= +k TP;",              // subject introduced. object pulled up.
            "-ady :: AspP= +k TP;",
            "-yp :: AspP= +k TP;",
            "",

            "{AspP} :: VoiceP= AspP;",
            "otyr :: VoiceP= AspP;",             // right now
            "jatyr :: VoiceP= AspP;",            // ongoing over time
            "tur :: VoiceP= AspP;",              // repeated
            "",

            "-yl :: vP= VoiceP;",               // PASSIVE
            "{VoiceP} :: vP= VoiceP;",          // ACTIVE
            "",

            "{vP} :: VP= vP;",                   // no ext arg for passive constructions
            "{vP} :: VP= d= vP;",                // ext arg for active constructions
            "",

            "jaz :: d= VP;"
        ]
  },
  { 
    name: "Auxiliaries", 
    language: "Korean", 
    mg: 
        [
            "yumi :: d;",
            "",

            "usɔyo :: d= s;",
            "usɔsɔyo :: d= s;",
            "usəl :: v;",
            "usgo :: GER;",
            "",

            "kɔyayo :: v= d= s;",
            "kɔyayo :: AUX= GER= d= s;",
            "",

            "issɔyo :: GER= d= s;",
            "isəl :: GER= AUX;"
        ]
  },
  { 
    name: "Smiling", 
    language: "Norwegian", 
    mg: 
        [
        "Jeg :: d;",
        "",
        "smiler :: d= s;",
        "smilte :: d= s;",
        "smilt :: perf;",
        "smile :: y;",
        "",
        "har :: =perf d= s;",
        "hadde :: =perf d= s;",
        "ha :: =perf y;",
        "",
        "skal :: =y d= s;",
        "vil :: =y d= s;"
        ]
  },
]);
</script>

<template>
    <div class="examples">
        <Panel header="MG Examples">
            <p class="flex-wrap font-medium m-0">
                This page contains examples of minimalist grammars.<br> 
                The grammars are not complete and are only meant to illustrate the basic concepts of minimalist grammars.<br>
                A special thank you to Samantha Zielinski, Mara Koshold, and Mira Ahmedovic for their contributions.
            </p>
            <Divider />
        </Panel>

        <Divider />
        <div class="p-4 grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <Card v-for="(grammar, index) in grammars" :key="index" 
                class="transition duration-200 hover:shadow-xl hover:bg-gray-100 cursor-pointer shadow-lg rounded-2xl">
            <template #title>
                <div class="text-xl font-semibold">{{ grammar.name }}</div>
            </template>

            <template #subtitle>
                <div class="text-sm text-gray-500">{{ grammar.language }}</div>
            </template>

            <template #content>
                <div v-for="(line, i) in grammar.mg" :key="i" class="text-sm text-gray-500">{{ line }} <br> </div>
            </template>
            </Card>
        </div>
    </div>
</template>

