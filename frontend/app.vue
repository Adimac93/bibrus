<template>
    <div>
        {{ data }} {{ error }}
    </div>
</template>

<script setup lang="ts">
import { apiURL } from "&/api-url";

const { data, error } = await useFetch(apiURL("/"));
</script>

