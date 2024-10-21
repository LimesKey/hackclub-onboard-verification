<script lang="ts">
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";

    onMount(() => {
        const urlParams = new URL(window.location.href).searchParams;
    let retrievedCode = urlParams.get('code') ?? '';

        if (retrievedCode) {
            localStorage.setItem('githubCode', retrievedCode);
        }
        goto('/');
    });
</script>