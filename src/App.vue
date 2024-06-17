<template>
  <div id="app">
    <h1>Thread Manager</h1>
    <div>
      <label> Rate: <input type="number" v-model="rate" /></label>
      <button @click="addThread">Add Thread</button>
    </div>
    <div class="container">
      <div class="threads-column">
        <h2>Threads</h2>
        <div v-for="id in threads" :key="id" class="thread-item">
          <p>Thread ID: {{ id }}</p>
          <p>Counter: {{ counters[id] }}</p>
          <button @click="stopThread(id)">Stop Thread</button>
        </div>
      </div>
      <div class="fifo-column">
        <h2>FIFO queue</h2>
        <div v-for="(message) in fifo"  class="fifo-item">
          <p>threadID: {{ message.id }}</p>
          <p>Message: {{ message.count }}</p>
        </div>
      </div>
      <div class="collection-column">
        <h2>Collection</h2>
        <div v-for="(message) in collection"  class="collection-item">
          <p>threadID: {{ message[0] }}</p>
          <p>Message: {{ message[1] }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export default {
  name: 'App',
  data() {
    return {
      rate: 1,
      threads: [],
      counters: {},
      collection: [],
      fifo: []
    };
  },
  methods: {
    async fetchThreads() {
      try {
        const res = await invoke('get_thread_ids');
        console.log(`List of IDs: ${res}`);
        this.threads = res;
      } catch (e) {
        console.error(e);
      }
      this.threads.forEach(id => {
        listen(`thread-${id}`, (event) => {
          this.counters[id] = event.payload;
        })
          .then(() => {
            console.log(`Successfully set up Tauri event listener for thread ${id}`);
          })
          .catch((error) => {
            console.error(`Error setting up Tauri event listener for thread ${id}:`, error);
          });
      });

      listen('hashmap', (event) => {
        console.log('hashmap', event);
        this.collection = event.payload;
      }).catch((error) => {
        console.error('Error setting up Tauri event listener for hashmap:', error);
      });
      listen('fifo', (event) => {
        console.log('fifo', event);
        this.fifo = event.payload;
      }).catch((error) => {
        console.error('Error setting up Tauri event listener for hashmap:', error);
      });
    },
    async addThread() {
      invoke('add_thread', { rate: this.rate })
        .then((res) =>
          console.log(`Created thread ID: ${res}`)
        ).catch((e) => console.error(e));
      this.fetchThreads();
    },
    async stopThread(id) {
      invoke('stop_thread', { id })
        .then((res) => {
          console.log("Thread stopped with ID", res);
        }).catch((e) => console.error(e));
      this.fetchThreads();
    },
  },
  mounted() {
    this.fetchThreads();
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

.container {
  display: flex;
  justify-content: space-between;
}

.threads-column, .fifo-column, .collection-column {
  flex: 1;
  padding: 10px;
}

.thread-item {
  background-color: #f9f9f9;
  border: 1px solid #ddd;
  margin-bottom: 10px;
  padding: 10px;
}

.fifo-item {
  background-color: #e9f7ef;
  border: 1px solid #a9dfbf;
  margin-bottom: 10px;
  padding: 10px;
}

.collection-item {
  background-color: #fce5cd;
  border: 1px solid #f5cba7;
  margin-bottom: 10px;
  padding: 10px;
}
</style>