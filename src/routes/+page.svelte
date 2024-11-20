<script lang="ts">
    import { onMount } from 'svelte';
    import init, { verify_api } from '../../wasm-lib/pkg/wasm_lib'; // Adjust the path as necessary

    let SlackCode: string | null;
    let GitHubCode: string | null;
    let verificationResult: string | null = null;
    let isLoading: boolean = false;
    let errorMessage: string | null = null;

    onMount(async () => {
        await init(); // Initialize the Wasm module
        SlackCode = localStorage.getItem('slackCode');
        GitHubCode = localStorage.getItem('githubCode');

        if (!SlackCode) {
            console.log('You need to sign in to Slack');
        }

        if (!GitHubCode) {
            console.log('You need to sign in to GitHub');
        }
    });

    async function handleContinue() {
        if (SlackCode && GitHubCode) {
            isLoading = true;
            errorMessage = null;
            try {
                const result = await verify_api(SlackCode, GitHubCode);
                verificationResult = result;
                console.log('Verification Result:', result);
                localStorage.removeItem('slackCode');
                localStorage.removeItem('githubCode');

                window.location.href = result;
            } catch (error) {
                if (error instanceof Error) {
                    errorMessage = 'Verification failed: ' + error.message;
                } else {
                    errorMessage = 'Verification failed: An unknown error occurred';
                }
                console.error('Verification failed:', error);
                localStorage.removeItem('slackCode');
                localStorage.removeItem('githubCode');
            } finally {
                isLoading = false;
            }
        }
    }
</script>

<div class="min-h-screen flex flex-col bg-gradient-to-r from-purple-400 via-pink-500 to-red-500">
    <div class="flex-grow flex items-center justify-center">
        <div class="bg-white shadow-lg rounded-lg p-8 max-w-lg w-full">
            <h1 class="text-2xl font-bold text-center mb-4">OnBoard OAuth Verification</h1>
            <div class="flex justify-center gap-4 mb-4">
                <button
                    class={SlackCode ? "btn-success" : "btn-primary"}
                    on:click={() => {
                        if (!SlackCode) {
                            window.location.href =
                                'https://slack.com/oauth/v2/authorize?client_id=2210535565.6498112565286&scope=&user_scope=openid,email,profile&redirect_uri=https://verify.onboard.hackclub.com/auth/callback/slack';
                        }
                    }}
                    disabled={!!SlackCode}
                    title={SlackCode ? "Slack authentication complete" : ""}
                >
                    Login to Slack
                </button>

                <button
                    class={GitHubCode ? "btn-success" : "btn-primary"}
                    on:click={() => {
                        if (!GitHubCode) {
                            window.location.href =
                                'https://github.com/login/oauth/authorize?client_id=Ov23li7FU4IQ7x89NQMn&redirect_uri=https://verify.onboard.hackclub.com/auth/callback/github';
                        }
                    }}
                    disabled={!!GitHubCode}
                    title={GitHubCode ? "GitHub authentication complete" : ""}
                >
                    Login to GitHub
                </button>
            </div>

            <div class="flex justify-center mb-4">
                <button
                    class="btn-secondary"
                    on:click={handleContinue}
                    disabled={!SlackCode || !GitHubCode || isLoading}
                >
                    {#if isLoading}
                        Loading...
                    {:else}
                        Continue
                    {/if}
                </button>
            </div>

            {#if verificationResult}
                <p class="text-center mt-4 text-green-500">Verification Result: {verificationResult}</p>
            {/if}

            {#if errorMessage}
                <p class="text-center mt-4 text-red-500">Error: {errorMessage}</p>
            {/if}
        </div>
    </div>
    <footer class="text-center text-white py-4">
        <p>Made by LimesKey</p>
    </footer>
</div>