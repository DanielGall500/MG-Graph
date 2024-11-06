<template>
  <div id="app" class="container">
    <h1 class="display-4 text-primary mt-5">Minimalist Grammar Size Calculator</h1>
    <form @submit.prevent="submitGrammar" class="mt-4 p-4 rounded shadow-sm bg-light">
      <div class="form-group">
        <textarea
          v-model="grammar"
          class="form-control"
          placeholder="Enter your grammar here..."
          rows="8"
        ></textarea>
      </div>
      <button type="submit" class="btn btn-primary btn-block">Calculate Size</button>
    </form>

    <div v-if="size !== null" class="mt-4 p-3 bg-success text-white rounded shadow-sm">
      <h2>Grammar Size: <span class="font-weight-bold">{{ size }}</span></h2>
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
        const response = await fetch('http://127.0.0.1:8000/calculate', { // Adjust the URL as necessary
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

<style scoped>
#app {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
}
h1 {
  font-size: 2rem;
  text-align: center;
  margin-bottom: 20px;
}
textarea {
  width: 100%;
  resize: vertical;
  padding: 10px;
}
button {
  font-size: 1.1rem;
}
</style>
