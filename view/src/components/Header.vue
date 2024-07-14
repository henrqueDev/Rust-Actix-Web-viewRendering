<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useSidebarStore } from '@/stores/sidebar';
import { useCartStore } from '@/stores/cart';
import Hamburger from './icons/Hamburger.vue';
import Search from "./icons/Search.vue";
import Favourite from "./icons/Favourite.vue";
import Cart from "./icons/Cart.vue";
import User from "./icons/User.vue";
import { ref, onMounted } from "vue";

const sideBarstore = useSidebarStore();
const cartStore = useCartStore();
const cartElement = ref<HTMLDivElement | null>(null);

cartStore.$onAction(function () {
    cartElement.value!.animate([
        { transform: "scale(1)" },
        { transform: "scale(2)" },
        { transform: "scale(1)" }
    ], { duration: 200 })
});

const num = ref(0);

onMounted( () => {
    fetch("pacientes/list?per_page=10&page=1").then((res) => res.json())
        .then(response => response)
        .then(data => {
            num.value = data.data;
        // What if data.result is neither null nor SUCCESS?  <--------
        });
});

</script>

<template>
    <header class="position-fixed">
        <div id="navbar-top">
            <div class="d-flex justify-content-between align-items-center w-75 m-auto h-100">
                <span class="fw-500 text-light">Welcome to Our Store Hiscraves</span>
                <span class="fw-500 text-light">Call Us: 1234567890</span>
            </div>
        </div>
        <nav id="navbar" class="bg-light">
            <div class="d-flex justify-content-between align-items-center w-75 m-auto h-100">
                <div>
                    <button class="border-0 bg-transparent" @click="sideBarstore.toggleSidebar">
                        <Hamburger></Hamburger>
                    </button>
                </div>
                <h1 class="h3 ms-5 mt-2">{{ num }}</h1>
                <!-- logo -->
                <div class="d-flex align-items-center justify-content-center" style="gap: 20px;">
                    <button class="border-0 bg-transparent">
                        <Search/>
                    </button>
                    <button class="border-0 bg-transparent">
                        <Favourite/>
                    </button>
                    <button class="border-0 bg-transparent position-relative">
                        <div ref="cartElement">
                            <Cart />
                        </div>
                        <span class="badge text-bg-danger position-absolute top-0 start-100 translate-middle rounded-pill">{{ cartStore.itemsInCart }}</span>
                    </button>
                    <button class="border-0 bg-transparent">
                        <User/>
                    </button>
                </div>
            </div>
        </nav>
    </header>
</template>

<style scoped>
    header {
       z-index: 1000;
        top: 0;
        left: 0;
        right: 0;
    }

    #navbar-top {
        height: 48px;
        background-color: #1f1f1f;
    }

    #navbar {
        height: 88px;
        box-shadow: 0px 8px 21px 0px rgba(0, 0, 0, 0.16);
    }
</style>@/stores/sidebar