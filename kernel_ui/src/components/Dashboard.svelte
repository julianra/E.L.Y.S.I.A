<!-- ======================================================================
📍 FILE: src/components/Dashboard.svelte
📝 ROLE:
  Eerste scherm na succesvolle login.
====================================================================== -->

<script lang="ts">
  import { authToken } from "../stores/kernel";
</script>

<h2>Welcome, Admin</h2>

<p>You are successfully logged in.</p>

<pre>
Token:
{$authToken}
</pre>
