<script lang="ts" setup>
import { toggleDark } from "~/composables";
import { useAuthStore } from "~/stores";
</script>

<template>
  <el-menu class="el-menu-demo" mode="horizontal">
    <el-menu-item index="1"><RouterLink to="/">Home</RouterLink></el-menu-item>
    <el-menu-item index="5"
      ><RouterLink to="/about">About</RouterLink></el-menu-item
    >
    <el-menu-item index="6"
      ><RouterLink to="/files">Files</RouterLink></el-menu-item
    >
    <el-menu-item index="7"
      ><RouterLink to="/profile">Profile</RouterLink></el-menu-item
    >
    <el-menu-item index="8" v-if="useAuthStore().user?.isAdmin()"
      ><RouterLink to="/users">Users</RouterLink></el-menu-item
    >

    <!--
    <el-sub-menu index="2">
      <template #title>Workspace</template>
      <el-menu-item index="2-1">item one</el-menu-item>
      <el-menu-item index="2-2">item two</el-menu-item>
      <el-menu-item index="2-3">item three</el-menu-item>
      <el-sub-menu index="2-4">
        <template #title>item four</template>
        <el-menu-item index="2-4-1">item one</el-menu-item>
        <el-menu-item index="2-4-2">item two</el-menu-item>
        <el-menu-item index="2-4-3">item three</el-menu-item>
      </el-sub-menu>
    </el-sub-menu>
    <el-menu-item index="3" disabled>Info</el-menu-item>
    <el-menu-item index="4">Orders</el-menu-item>
      -->

    <el-menu-item h="full" @click="toggleDark()">
      <button
        class="border-none w-full bg-transparent cursor-pointer"
        style="height: var(--ep-menu-item-height)"
      >
        <i inline-flex i="dark:ep-moon ep-sunny" />
      </button>
    </el-menu-item>
    <div class="flex-grow" />
    <el-menu-item @click="useAuthStore().logout()">Logout</el-menu-item>
  </el-menu>
</template>
