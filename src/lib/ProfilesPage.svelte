<script>
    import { invoke } from "@tauri-apps/api";
    import ProfilePageItem from "./ProfilePageItem.svelte";

    export let profiles;

    function newProfile() {
        invoke("create_profile")
    }
</script>

<main>
    {#each profiles as profile}
        <ProfilePageItem profile={profile}/>
    {/each}
    <button class="new" on:click={newProfile}>New Profile</button>
</main>

<style>
    .new {
        position:fixed;
        right: 1rem;
        bottom: 5rem;
        padding: .5rem;
    }
</style>