<template>
  <div id="app">
    <h1>Thread Manager</h1>
    <div class="control-panel">
      <label>
        Rate:
        <input type="number" v-model="rate" />
      </label>
      <button @click="addThread">Add Thread</button>
    </div>
    <transition-group name="thread" class="threads-list">
      <div v-for="id in threads" :key="id" class="thread-box">
        <p>Thread ID: {{ id }}</p>
        <p>Counter: {{ counters[id] }}</p>
        <button @click="stopThread(id)" class="stop-button">Stop Thread</button>
      </div>
      <div>Collection </div>
      <div v-for="messages in collection" :key="id">
        <p>Thread ID: {{ id }}</p>
        <p>Counter: {{ counters[id] }}</p>
        <button @click="stopThread(id)" class="stop-button">Stop Thread</button>
      </div>
    </transition-group>
  </div>
</template>

<script>
import { event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export default {
  name: 'App',
  data() {
    return {
      rate: 1,
      threads: [],
      counters: {},
      collection: {},
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
          //console.log('Événement reçu:', event);
          // Mettre à jour l'état du composant ou effectuer d'autres actions
          this.counters[id] = event.payload;
        })
          .then(() => {
            console.log(`Écouteur d'événements Tauri configuré avec succès pour le thread ${id}`);
          })
          .catch((error) => {
            console.error(`Erreur lors de la configuration de l'écouteur d'événements Tauri pour le thread ${id}:`, error);
          });
      });

      listen('hashmap', (event) => {
        console.log('hashmap', event);
        this.collection = event.payload;
      }).catch((error) => {
            console.error(`Erreur lors de la configuration de l'écouteur d'événements Tauri pour le thread ${id}:`, error);
          })
      
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
  text-align: center;
  background-color: #f9f9f9;
  color: #333;
  padding: 20px;
}

h1 {
  margin-bottom: 20px;
}

.control-panel {
  margin-bottom: 20px;
}

.control-panel label {
  margin-right: 10px;
}

button {
  background-color: #4CAF50;
  color: white;
  border: none;
  padding: 10px 20px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
  margin: 4px 2px;
  cursor: pointer;
  border-radius: 5px;
  transition: background-color 0.3s ease;
}

button:hover {
  background-color: #45a049;
}

.stop-button {
  background-color: #f44336;
}

.stop-button:hover {
  background-color: #e53935;
}

.threads-list {
  display: flex;
  flex-wrap: nowrap;
  overflow-x: auto;
  gap: 20px;
  margin-top: 20px;
}

.thread-box {
  border: 1px solid #ccc;
  padding: 10px;
  width: 200px;
  text-align: left;
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.thread-box p {
  margin: 5px 0;
}

.thread-box button {
  display: block;
  margin: 10px 0 0;
}

.thread-enter-active, .thread-leave-active {
  transition: all 0.5s ease;
}

.thread-enter, .thread-leave-to /* .thread-leave-active in <2.1.8 */ {
  transform: translateX(-100%);
  opacity: 0;
}

.thread-enter-to {
  transform: translateX(0);
}

.thread-leave {
  transform: translateX(100%);
}
</style>