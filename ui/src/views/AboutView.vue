<template>
  <div class="about">
    <h3>Frontend</h3>
    <p v-for="(val, name) in aboutUi">{{ name }}: {{ val }}</p>
    <br />
    <h3>Server</h3>
    <p v-for="(val, name) in aboutApi">{{ name }}: {{ val }}</p>
  </div>
</template>

<script setup lang="ts">
import { name, version } from "~/../package.json";
import axios from "axios";
import { ref, onMounted, UnwrapRef, Ref } from "vue";
import { About } from "~/models";

const aboutUi = {
  version,
  name,
  software: ["npm", "vue", "element-plus"],
};

const aboutApi: Ref<UnwrapRef<About | null>> = ref(null);
onMounted(() => {
  axios.get("/about").then((res) => {
    aboutApi.value = About.fromJSON(res.data);
  });
});
</script>
<style>
.about {
  margin: auto;
  width: 100%;
  height: 100%;
  justify-content: center;
}
</style>
