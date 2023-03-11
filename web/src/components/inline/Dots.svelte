<script lang="ts">
    import type { INode } from "../../utils/types";

    export let node: INode = {tag: "dots", value:`...`, children: null};
</script>

<span>{`${node.value} `}</span>
