<template>
  <div>
    <h1>Neo4j Graph Visualization</h1>
    <h1>{{ status }}</h1>
    <div id="viz"></div>
    <Button @click="reload">Reload</Button>

  </div>
</template>

<script setup lang="ts">
// import NeoVis from 'neovis.js'
import dotenv from 'dotenv'
import {ref} from 'vue'
import NeoVis from 'neovis.js/dist/neovis.js';

// dotenv.config()

const passwordEnvVariable = "almonds1" // process.env.PASSWORD;

const containerId = "graph-container"
const serverUsername = "neo4j"
const serverPassword = passwordEnvVariable
const serverURL = "bolt://localhost:7687/"

const status = ref()

function reload() {
  try {
    const vis = new NeoVis({
        containerId: "viz",
        neo4j: {
            serverUrl: serverURL,
            serverUser: serverUsername,
            serverPassword: serverPassword
        },
        labels: {
          State: {
              label: "name"
          }
        },
        relationships: {
            MERGE: {
              label: "li"
                // value: "weight"
            }
        }
    })
    // vis.render()
    vis.renderWithCypher("MATCH (n)-[r:MERGE]->(m) RETURN *;")
    // vis.renderWithCypher("MATCH (n) RETURN n;")
  status.value = "Success"
  } catch (error) {
    status.value = error
  }
}
</script>

<style scoped>
#viz {
  width: 900px;
  height: 700px;
  border: 1px solid lightgray;
  font: 22pt arial;
}
</style>
