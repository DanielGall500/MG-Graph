<template>
  <div style="height: 20vh; width: 20vw;">
    <Card :id="containerId" class="visualisation"></Card>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue'
import NeoVis from 'neovis.js/dist/neovis.js';

const usernameEnvVariable = import.meta.env.VITE_NEO_USER;
const passwordEnvVariable = import.meta.env.VITE_NEO_PW; 

const containerId = "graph-vis-box";
const serverUsername = usernameEnvVariable;
const serverPassword = passwordEnvVariable;
const serverURL = "bolt://localhost:7687/";

const status = ref()

function reload_vis() {
  try {
    const vis = new NeoVis({
        containerId: containerId,
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
            }
        }
    })
    vis.renderWithCypher("MATCH (n)-[r]->(m) RETURN *;")
    vis.registerOnEvent("completed", () => {
      console.log("Completed rendering");
      console.log("Nodes:", vis.network.body.data.nodes);
      console.log("Edges:", vis.network.body.data.edges);
    });

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
.visualisation {
  width: 900px;
  height: 700px;
  font: 22pt arial;
}
</style>
