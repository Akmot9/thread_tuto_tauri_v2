<template>
  <div id="app">
    <h1>Thread Manager</h1>
    <div>
      <label>
        Rate:
        <input type="number" v-model="rate" />
      </label>
      <button @click="addThread">Add Thread</button>
    </div>
    <div class="threads-list">
      <div v-for="id in threads" :key="id" class="thread-box">
        <p>Thread ID: {{ id }}</p>
        <button @click="stopThread(id)">Stop Thread</button>
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
    };
  },
  methods: {
    async fetchThreads() {
      try {
        const res = await invoke('get_thread_ids');
        console.log(`liste des IDs: ${res}`);
        this.threads = res;
      } catch (e) {
        console.error(e);
      }
      this.threads.forEach(id => {
        listen(`thread-${id}`, (event) => {
          console.log('Événement reçu:', event);
          // Mettre à jour l'état du composant ou effectuer d'autres actions
          //this.$set(this.counters, id, event.payload);
        })
          .then(() => {
            console.log(`Écouteur d'événements Tauri configuré avec succès pour le thread ${id}`);
          })
          .catch((error) => {
            console.error(`Erreur lors de la configuration de l'écouteur d'événements Tauri pour le thread ${id}:`, error);
          });
      });
      
    },

    async addThread() {
      invoke('add_thread', { rate: this.rate })
        .then((res) =>
          console.log(`Numero du thread cree: ${res}`)
        ).catch((e) => console.error(e));
        this.fetchThreads();
    },

    async stopThread(id) {
      invoke('stop_thread', { id })
      .then((res) => {
          console.log("Thread stopped with id ",res);
        }).catch((e) => console.error(e));
      this.fetchThreads();
    },

    mounted() {
      this.fetchThreads();
      
    },
    
  }
};
</script>

<style>
#app {
  font-family: sans-serif;
  text-align: center;
}

.threads-list {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 20px;
  margin-top: 20px;
}

.thread-box {
  border: 1px solid #ccc;
  padding: 10px;
  width: 200px;
  text-align: left;
}

.thread-box p {
  margin: 5px 0;
}

.thread-box button {
  display: block;
  margin: 10px 0 0;
}
</style>
