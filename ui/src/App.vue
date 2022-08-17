<template>
  <div class="common-layout">
    <el-config-provider namespace="ep">
      <el-container>
        <el-header v-if="authenticated()">
          <BaseHeader />
        </el-header>
        <el-container>
          <el-aside v-if="authenticated()">
            <BaseSide />
          </el-aside>
          <el-main>
            <main>
              <RouterView />
            </main>
          </el-main>
        </el-container>
      </el-container>
    </el-config-provider>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from "./stores";

const authenticated = useAuthStore().authenticated;
useAuthStore().start_check();
</script>
<style>
#app {
  text-align: center;
  color: var(--ep-text-color-primary);
}

.element-plus-logo {
  width: 50%;
}
</style>
