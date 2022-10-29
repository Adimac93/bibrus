<template>
    <div class="wrapper">
        <h1>Zaloguj się</h1>
        <form @submit.prevent="handleSubmit" class="login-form">
            <div class="form-item"><label>Login</label><input type="text" required v-model="login"/></div>
            <div class="form-item"><label>Hasło</label><input type="password" required v-model="password"/></div>

            <button class="form-button">Zaloguj się</button>
        </form>
    </div>
</template>

<script setup lang="ts">
const login = ref("");
const password = ref("");

function handleSubmit() {
    alert("This button's functionality is blocked on API docs.");
}
</script>

<style scoped lang="scss">

.wrapper {
    display: grid;
    place-items: center;
}
.login-form {
    display: flex;
    flex-flow: column nowrap;
    gap: 8px;
}

.form-button {
    display: block;
    width: 100%;
}

.form-item {
    display: flex;
    flex-flow: column nowrap;
}
</style>
