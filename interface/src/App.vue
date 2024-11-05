<template>
  <div id="app">
    <h1>Minimalist Grammar Input</h1>
    <form @submit.prevent="submitGrammar">
      <textarea v-model="grammar" placeholder="Enter your grammar here..." rows="10" cols="30"></textarea>
      <br />
      <button type="submit">Calculate Size</button>
    </form>
    <div v-if="size !== null">
      <h2>Grammar Size: {{ size }}</h2>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      grammar: '', // Store the user's grammar input
      size: null, // Store the calculated size
    };
  },
  methods: {
    async submitGrammar() {
      try {
        console.log("SUBMITTTINGGG")
        const response = await fetch('http://localhost:8080/calculate', { // Adjust the URL as necessary
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ grammar: this.grammar }), // Send the grammar to the backend
        });
        const data = await response.json();
        this.size = data.size; // Assuming the response has a size field
      } catch (error) {
        console.error('Error:', error);
      }
    },
  },
};
</script>

<style>
/* Add some basic styles */
#app {
  max-width: 600px;
  margin: 0 auto;
  text-align: center;
}
textarea {
  width: 100%;
  margin-bottom: 10px;
}
</style>
