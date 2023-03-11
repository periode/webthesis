<script lang="ts">
    export let date: string;
    let d = new Date(parseInt(date) * 1000);
    const day = d.getDate() < 10 ? "0" + d.getDate() : d.getDate();
    const month =
        d.getMonth() + 1 < 10 ? "0" + (d.getMonth() + 1) : d.getMonth() + 1;
    const year = d.getFullYear();
    const hour = d.getHours() < 10 ? "0" + d.getHours() : d.getHours();
    const minute = d.getMinutes() < 10 ? "0" + d.getMinutes() : d.getMinutes();
</script>

<time datetime={`${year}`} class="text-sm">
    last updated on: {`${day}.${month}.${year}`} at {`${hour}:${minute}`}
</time>
