<script setup lang="ts">
import Dialog from 'primevue/dialog';
import { ref } from 'vue';

const visible = ref(false);
</script>

<template>
  <div class="flex flex-wrap">
  <Toast />
  <div class="sticky top-0 bg-white z-50 shadow-md card flex justify-content-left" style="z-index: 50; background-color: white;width: 100vh;">
          <div class="flex flex-row h-full align-items-center justify-content-center">
              <div class="flex align-items-center justify-content-between px-4 pt-3 flex-shrink-0">
                  <span class="inline-flex align-items-center gap-2">
                      <span class="font-semibold text-2xl text-primary">MG-Graph</span>
                  </span>
              </div>
              <div class="overflow-y-auto">
                    <ul class="flex flex-row list-none p-0 m-0 overflow-hidden">
                        <li>
                            <a href='/' v-ripple class="no-underline flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                <i class="pi pi-home mr-2"></i>
                                <span class="font-medium">Dashboard</span>
                            </a>
                        </li>
                        <li>
                            <a href='/mygrammars' v-ripple class="no-underline flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                <i class="pi pi-star mr-2"></i>
                                <span class="font-medium">My MGs</span>
                            </a>
                        </li>
                        <li>
                            <a href='/examples' v-ripple class="no-underline flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                <i class="pi pi-bookmark mr-2"></i>
                                <span class="font-medium">Examples</span>
                            </a>
                        </li>
                        <li>
                            <a href='/settings' v-ripple class="no-underline flex align-items-center cursor-pointer p-3 border-round text-700 hover:surface-100 transition-duration-150 transition-colors p-ripple">
                                <i class="pi pi-user mr-2"></i>
                                <span class="font-medium">Settings</span>
                            </a>
                        </li>
                    </ul>
              </div>
              <div class="mt-auto">
                  <hr class="mb-3 mx-3 border-top-1 border-none surface-border" />
              </div>
        </div>
      </div>
        <Divider layout="horizontal"/>

    <RouterView />

    </div>
  
</template>

