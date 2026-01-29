<script setup lang="ts">
</script>

<template>
  <nav class="navbar">
    <h2>
      <RouterLink to="/">Home</RouterLink>
    </h2>
    <h2>
      <RouterLink to="/board/0">Tierlista</RouterLink>
    </h2>
    <h2>
      <RouterLink to="/">Some other</RouterLink>
    </h2>
    <h2>
      <RouterLink to="/">fun</RouterLink>
    </h2>
    <h2>
      <RouterLink to="/">stuff</RouterLink>
    </h2>
  </nav>
  <RouterView></RouterView>
</template>

<style scoped>
.navbar {
  display: flex;
  flex-flow: nowrap row;
  width: 100vw;
  border-bottom: 2px gray solid;
}

.navbar h2 {
  margin: 20px;
}
h2 a {
  all: unset;
  cursor: pointer; 
  text-decoration: none;
}
</style>
