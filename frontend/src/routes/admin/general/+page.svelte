<script lang="ts">
	import { fetchConfig } from '$lib/api/config';
	import { t } from 'svelte-intl-precompile';
	import { Card, CardHeader, CardTitle, CardContent } from '$lib/components/ui/card';
	import type { AppConfig } from '$lib/domain/config';
	import Settings from 'lucide-svelte/icons/settings';
	import Link from 'lucide-svelte/icons/link';
	import FileText from 'lucide-svelte/icons/file-text';
	import ShieldCheck from 'lucide-svelte/icons/shield-check';
	import Calendar from 'lucide-svelte/icons/calendar';
	import BarChart from 'lucide-svelte/icons/bar-chart';

	let config: AppConfig | null = $state(null);
	let loading = $state(true);

	$effect(() => {
		if (loading) {
			loadConfig();
		}
	});

	async function loadConfig() {
		loading = true;
		try {
			config = await fetchConfig();
		} catch (e) {
			console.error('Failed to load config:', e);
		} finally {
			loading = false;
		}
	}

	function formatReservedNames(names: string[]): string {
		return names.join(', ');
	}
</script>

<svelte:head>
	<title>{$t('adminPage.title')} - {$t('adminPage.tabs.general')}</title>
</svelte:head>

{#if loading}
	<div class="py-12 text-center">
		<p class="text-muted-foreground">{$t('common.loading')}</p>
	</div>
{:else if !config}
	<div class="py-12 text-center">
		<p class="text-muted-foreground text-lg">{$t('adminPage.errors.notAuthorized')}</p>
	</div>
{:else}
	<div class="grid grid-cols-1 gap-6 md:grid-cols-2">
		<!-- Service Settings Card -->
		<Card>
			<CardHeader>
				<div class="flex items-center gap-2">
					<Settings class="h-5 w-5" />
					<CardTitle>{$t('adminPage.general.sections.service')}</CardTitle>
				</div>
			</CardHeader>
			<CardContent>
				<div class="space-y-3">
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.baseUrl')}:</span
						>
						<span class="max-w-[60%] text-right text-sm font-medium break-all"
							>{config.baseUrl}</span
						>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.shortUrlTtl')}:</span
						>
						<span class="text-sm font-medium"
							>{$t('adminPage.general.values.hours', {
								values: { count: config.shortUrlTtl }
							})}</span
						>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxUrlLength')}:</span
						>
						<span class="text-sm font-medium"
							>{$t('adminPage.general.values.characters', {
								values: { count: config.maxUrlLength }
							})}</span
						>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- URL Creation Card -->
		<Card>
			<CardHeader>
				<div class="flex items-center gap-2">
					<Link class="h-5 w-5" />
					<CardTitle>{$t('adminPage.general.sections.createUrl')}</CardTitle>
				</div>
			</CardHeader>
			<CardContent>
				<div class="space-y-3">
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.enabled')}:</span
						>
						<span
							class="text-sm font-medium"
							class:text-green-600={config.features.createUrl.enabled}
							class:dark:text-green-400={config.features.createUrl.enabled}
							class:text-red-600={!config.features.createUrl.enabled}
							class:dark:text-red-400={!config.features.createUrl.enabled}
						>
							{config.features.createUrl.enabled
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.authOnly')}:</span
						>
						<span
							class="text-sm font-medium"
							class:text-green-600={config.features.createUrl.authOnly}
							class:dark:text-green-400={config.features.createUrl.authOnly}
							class:text-red-600={!config.features.createUrl.authOnly}
							class:dark:text-red-400={!config.features.createUrl.authOnly}
						>
							{config.features.createUrl.authOnly
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxPerUser')}:</span
						>
						<span class="text-sm font-medium">{config.features.createUrl.maxPerUser}</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxPerDay')}:</span
						>
						<span class="text-sm font-medium">{config.features.createUrl.maxPerDay}</span>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Named URLs Card -->
		<Card>
			<CardHeader>
				<div class="flex items-center gap-2">
					<FileText class="h-5 w-5" />
					<CardTitle>{$t('adminPage.general.sections.namedUrls')}</CardTitle>
				</div>
			</CardHeader>
			<CardContent>
				<div class="space-y-3">
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.enabled')}:</span
						>
						<span
							class="text-sm font-medium"
							class:text-green-600={config.features.namedUrls.enabled}
							class:dark:text-green-400={config.features.namedUrls.enabled}
							class:text-red-600={!config.features.namedUrls.enabled}
							class:dark:text-red-400={!config.features.namedUrls.enabled}
						>
							{config.features.namedUrls.enabled
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.minLength')}:</span
						>
						<span class="text-sm font-medium">{config.features.namedUrls.minLength}</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxLength')}:</span
						>
						<span class="text-sm font-medium">{config.features.namedUrls.maxLength}</span>
					</div>
					<div class="flex flex-col gap-1">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.reservedNames')}:</span
						>
						<span class="text-sm font-medium">
							{formatReservedNames(config.features.namedUrls.reservedNames)}
						</span>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Authentication Card -->
		<Card>
			<CardHeader>
				<div class="flex items-center gap-2">
					<ShieldCheck class="h-5 w-5" />
					<CardTitle>{$t('adminPage.general.sections.auth')}</CardTitle>
				</div>
			</CardHeader>
			<CardContent>
				<div class="space-y-3">
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.enabled')}:</span
						>
						<span
							class="text-sm font-medium"
							class:text-green-600={config.auth.enabled}
							class:dark:text-green-400={config.auth.enabled}
							class:text-red-600={!config.auth.enabled}
							class:dark:text-red-400={!config.auth.enabled}
						>
							{config.auth.enabled
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.authType')}:</span
						>
						<span class="text-sm font-medium">{config.auth.authType}</span>
					</div>
					{#if config.auth.gitlab}
						<div class="flex items-start justify-between">
							<span class="text-muted-foreground text-sm"
								>{$t('adminPage.general.fields.gitlabBaseUrl')}:</span
							>
							<span class="max-w-[60%] text-right text-sm font-medium break-all">
								{config.auth.gitlab.baseUrl}
							</span>
						</div>
						<div class="flex items-start justify-between">
							<span class="text-muted-foreground text-sm"
								>{$t('adminPage.general.fields.gitlabAppId')}:</span
							>
							<span
								class="max-w-[60%] overflow-hidden text-right text-sm font-medium text-ellipsis whitespace-nowrap"
								title={config.auth.gitlab.applicationId}
							>
								{config.auth.gitlab.applicationId}
							</span>
						</div>
					{/if}
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm">{$t('adminPage.general.fields.note')}:</span
						>
						<span class="max-w-[60%] text-right text-sm font-medium">
							{config.auth.note || $t('adminPage.general.values.none')}
						</span>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Scheduler Card -->
		<Card>
			<CardHeader>
				<div class="flex items-center gap-2">
					<Calendar class="h-5 w-5" />
					<CardTitle>{$t('adminPage.general.sections.scheduler')}</CardTitle>
				</div>
			</CardHeader>
			<CardContent>
				<div class="space-y-3">
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.cleanupCron')}:</span
						>
						<span class="font-mono text-sm font-medium">{config.scheduler.cleanupExpiredUrls}</span>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Metrics Card -->
		<Card>
			<CardHeader>
				<div class="flex items-center gap-2">
					<BarChart class="h-5 w-5" />
					<CardTitle>{$t('adminPage.general.sections.metrics')}</CardTitle>
				</div>
			</CardHeader>
			<CardContent>
				<div class="space-y-3">
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.enabled')}:</span
						>
						<span
							class="text-sm font-medium"
							class:text-green-600={config.metrics.enabled}
							class:dark:text-green-400={config.metrics.enabled}
							class:text-red-600={!config.metrics.enabled}
							class:dark:text-red-400={!config.metrics.enabled}
						>
							{config.metrics.enabled
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					{#if config.metrics.enabled}
						<div class="flex items-start justify-between">
							<span class="text-muted-foreground text-sm"
								>{$t('adminPage.general.fields.metricsEndpoint')}:</span
							>
							<a
								href="/api/metrics"
								target="_blank"
								class="text-sm font-medium text-blue-600 hover:underline dark:text-blue-400"
							>
								/api/metrics
							</a>
						</div>
					{/if}
				</div>
			</CardContent>
		</Card>
	</div>
{/if}
