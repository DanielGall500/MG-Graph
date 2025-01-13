<template>
  <div>
    <h1>Neo4j Graph Visualization</h1>
    <div id="graph-container" style="width: 100%; height: 600px;"></div>
  </div>
</template>

<script>
import NeoVis from 'neovis.js'

const containerId = "graph-container"
const serverUsername = "neo4j"
const serverPassword = "almonds1"  // DEAR GOD REMOVE BEFORE GIT
const serverURL = "http://localhost:7474/"

const vis = new NeoVis({
    container_id: containerId,
    server_url: serverURL,
    server_user: serverUsername,
    server_password: serverPassword,
    initial_cypher: "MATCH (n)-[r]->(m) RETURN n,r,m"
})
vis.render()

viz.registerOnEvent("completed", () => {
    console.log("Graph rendering complete.");
});
</script>

<style scoped>
</style>
