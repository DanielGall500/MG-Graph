<template>
  <div>
    <div id="graph-vis-box"></div>
  </div>
</template>

<script setup lang="ts">
// import NeoVis from 'neovis.js'
import dotenv from 'dotenv'
import {ref} from 'vue'
import NeoVis from 'neovis.js/dist/neovis.js';

// dotenv.config()

const passwordEnvVariable = "almonds1" // process.env.PASSWORD;

const containerId = "graph-vis-box"
const serverUsername = "neo4j"
const serverPassword = passwordEnvVariable
const serverURL = "bolt://localhost:7687/"

const status = ref()

function reload_vis() {
  try {
    const vis = new NeoVis({
        containerId: "graph-vis-box",
        neo4j: {
            serverUrl: serverURL,
            serverUser: serverUsername,
            serverPassword: serverPassword
        },
        visConfig: {
          nodes: {
              shape: 'circle',
              size: 100
          },
          edges: {
              arrows: {
                  to: {enabled: true}
              }
          },
          physics: {
            enabled: true, // Enable physics simulation
            solver: 'forceAtlas2Based', // Use the forceAtlas2 solver for better node spacing
            forceAtlas2Based: {
                gravitationalConstant: -100, // Change gravitational constant to pull nodes further apart
                centralGravity: 0.01, // Adjust the central gravity to move nodes away from the center
                springLength: 250, // Increase spring length to make nodes more spread out
                springConstant: 0.05, // Decrease spring constant for less attractive force
                damping: 0.4, // Adjust the damping to slow the node movement
            },
            barnesHut: {
                theta: 0.5, // This is relevant if you're using barnesHut solver; controls accuracy vs. performance
            },
            stabilization: {
                iterations: 1000, // Number of iterations for stabilization before the physics stop
                updateInterval: 25, // Interval for updates in milliseconds
            }
          },
        },
        labels: {
          State: {
              label: "name",
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

defineExpose({
  reload_vis
});
</script>

<style scoped>
#graph-vis-box {
  width: 900px;
  height: 700px;
  border: 1px solid lightgray;
  font: 22pt arial;
}
</style>
