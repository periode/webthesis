<script lang="ts">
    export let tag: string;
</script>

<div class="border-red-600 border text-black p-1">
    could not render node <b>{tag}</b>
    <slot></slot>
</div>



