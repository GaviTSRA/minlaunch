<script>
    import ProfilePageItem from "./ProfilePageItem.svelte";

    export let profiles;
</script>

<main>
    {#each profiles as profile}
        <ProfilePageItem profile={profile}/>
    {/each}
</main>