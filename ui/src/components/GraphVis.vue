<template>
  <div>
    <Card :id="containerId" class="visualisation" style="justify-content: center"></Card>
  </div>
</template>

<script setup lang="ts">
import {ref, onMounted } from 'vue'
import NeoVis from 'neovis.js';
import { useToast } from 'primevue/usetoast';

const containerId = "graph-vis-box";

const db_username = ref("");
const db_password = ref("");
const db_addr = ref("");

const toast = useToast();
const status = ref()

onMounted(() => {
    update_settings();
    reload_vis();
});

const vizContainer = ref(null)
const canvasRef = ref(null)

function showMessage(summary: string, detail: string, is_error: boolean) {
    const sev = is_error ? "error" : "success";
    toast.add({
        severity: sev,  
        summary: summary,
        detail: detail,
        life: 3000 
    });
};

async function update_settings() {
    const response = await fetch('http://127.0.0.1:8000/get-settings', { // Adjust the URL as necessary
        method: 'GET',
        headers: {
        'Content-Type': 'application/json',
        }
    });

    if (response.ok) {
      const response_json = await response.json();
      db_username.value = response_json.username;
      db_password.value = response_json.password;
      db_addr.value = response_json.db_addr;
    }
    else {
      showMessage("Visualisation Setup Unsuccessful", "Unable to connect to Neo4J for visualisation.", true);
    }
}

async function reload_vis() {
  await update_settings();
  try {
    const vis = new NeoVis({
        containerId: containerId,
        neo4j: {
            serverUrl: db_addr.value,
            serverUser: db_username.value,
            serverPassword: db_password.value,
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
                gravitationalConstant: -150, // Change gravitational constant to pull nodes further apart
                centralGravity: 0.003, // Adjust the central gravity to move nodes away from the center
                springLength: 250, // Increase spring length to make nodes more spread out
                springConstant: 0.03, // Decrease spring constant for less attractive force
                damping: 1, // Adjust the damping to slow the node movement
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
          "State": {
            label: "name",
           [NeoVis.NEOVIS_ADVANCED_CONFIG]: {
                static: {
                    color: {
                        border: "#ffffff",
                        background: "#ffffff",
                        highlight: {
                            border: "#ffffff",
                            background: "#000000"
                        }
                    },
                    shape: "dot",
                    size: 50,
                    borderWidth: "1",
                    font: {
                        "background": "none",
                        "strokeWidth": "0",
                        "size": 30,
                        "color": "#464646"
                    }
                },
                function: {
                    color: {
                        background: (node) => node.properties.CVE_ID ? "#bd6962" : node.properties.CWE_ID ? "#a4cbfa" : node.properties.PUBLISH_DATE ? "#f7ce46" : "#a4cb9d",
                    },
                    label: (node) => NeoVis.objectToTitleString(node, ["name","move"]).trim() // Remove leading/trailing newlines
                        .split('\n') // Split by line
                        .map(line => line.split(':')[1]?.trim() || '') // Get content after ':', or empty string
                        .join('\n'), // Join back into multi-line string
                }
            }
          },
          "Interm": {
            label: "name",
           [NeoVis.NEOVIS_ADVANCED_CONFIG]: {
                static: {
                    color: {
                        border: "#ffffff",
                        background: "#ffffff",
                        highlight: {
                            border: "#ffffff",
                            background: "#000000"
                        }
                    },
                    shape: "dot",
                    size: 50,
                    borderWidth: "1",
                    font: {
                        "background": "none",
                        "strokeWidth": "0",
                        "size": 30,
                        "color": "#464646"
                    }
                },
                function: {
                    color: {
                        background: (node) => node.properties.CVE_ID ? "#b833ff" : node.properties.CWE_ID ? "#b833ff" : node.properties.PUBLISH_DATE ? "#b833ff" : "#b833ff",
                        // background: (node) => node.properties.move == "-k" ? "#f7ce46" : "#a4cb9d",
                    },
                    label: (node) => NeoVis.objectToTitleString(node, ["name","move"])
                        .trim()
                        .split('\n') // Split by line
                        .map(line => line.split(':')[1]?.trim() || '') // Get content after ':', or empty string
                        .join('\n'), // Join back into multi-line string
                }
            }
          }
        },
        relationships: {
        'Merge': {
            [NeoVis.NEOVIS_ADVANCED_CONFIG]: {
                static: {
                    color: "#64b687",
                    font: {
                        background: "none",
                        strokeWidth: "0",
                        color: "#000000"
                    }
                },
                function: {
                    label: (edge) => NeoVis.objectToTitleString(edge, ["li", "move"])
                        .trim()
                        .split('\n') // Split by line
                        .map(line => line.split(':')[1]?.trim() || '') // Get content after ':', or empty string
                        .join('\n'), // Join back into multi-line string
                }
            }
        }
    }
    })
    vis.renderWithCypher("MATCH (n)-[r]->(m) RETURN *;")

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
  width: 80vw;
  height: 60vh;
  font: 22pt arial;
}
</style>
