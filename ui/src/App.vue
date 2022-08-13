<template>
  <div class="common-layout">
    <el-config-provider namespace="ep">
      <el-container>
        <el-header>
          <BaseHeader />
        </el-header>
        <el-container>
          <el-aside>
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

<style>
#app {
  text-align: center;
  color: var(--ep-text-color-primary);
}

.element-plus-logo {
  width: 50%;
}
</style>
