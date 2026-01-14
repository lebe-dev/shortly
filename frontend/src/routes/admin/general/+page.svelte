<script lang="ts">
	import { configStore } from '$lib/stores/config';
	import { t } from 'svelte-intl-precompile';
	import { Card, CardHeader, CardTitle, CardContent } from '$lib/components/ui/card';
	import Settings from 'lucide-svelte/icons/settings';
	import Link from 'lucide-svelte/icons/link';
	import FileText from 'lucide-svelte/icons/file-text';
	import ShieldCheck from 'lucide-svelte/icons/shield-check';
	import Calendar from 'lucide-svelte/icons/calendar';
	import BarChart from 'lucide-svelte/icons/bar-chart';

	function formatReservedNames(names: string[]): string {
		return names.join(', ');
	}
</script>

<svelte:head>
	<title>{$t('adminPage.title')} - {$t('adminPage.tabs.general')}</title>
</svelte:head>

{#if !$configStore}
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
							>{$configStore.baseUrl}</span
						>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.shortUrlTtl')}:</span
						>
						<span class="text-sm font-medium"
							>{$t('adminPage.general.values.hours', {
								values: { count: $configStore.shortUrlTtl }
							})}</span
						>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxUrlLength')}:</span
						>
						<span class="text-sm font-medium"
							>{$t('adminPage.general.values.characters', {
								values: { count: $configStore.maxUrlLength }
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
							class:text-green-600={$configStore.features.createUrl.enabled}
							class:dark:text-blue-300={$configStore.features.createUrl.enabled}
							class:text-red-600={!$configStore.features.createUrl.enabled}
							class:dark:text-red-400={!$configStore.features.createUrl.enabled}
						>
							{$configStore.features.createUrl.enabled
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
							class:text-green-600={$configStore.features.createUrl.authOnly}
							class:dark:text-blue-300={$configStore.features.createUrl.authOnly}
							class:text-red-600={!$configStore.features.createUrl.authOnly}
							class:dark:text-red-400={!$configStore.features.createUrl.authOnly}
						>
							{$configStore.features.createUrl.authOnly
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxPerUser')}:</span
						>
						<span class="text-sm font-medium">{$configStore.features.createUrl.maxPerUser}</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxPerDay')}:</span
						>
						<span class="text-sm font-medium">{$configStore.features.createUrl.maxPerDay}</span>
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
							class:text-green-600={$configStore.features.namedUrls.enabled}
							class:dark:text-blue-300={$configStore.features.namedUrls.enabled}
							class:text-red-600={!$configStore.features.namedUrls.enabled}
							class:dark:text-red-400={!$configStore.features.namedUrls.enabled}
						>
							{$configStore.features.namedUrls.enabled
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.minLength')}:</span
						>
						<span class="text-sm font-medium">{$configStore.features.namedUrls.minLength}</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.maxLength')}:</span
						>
						<span class="text-sm font-medium">{$configStore.features.namedUrls.maxLength}</span>
					</div>
					<div class="flex flex-col gap-1">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.reservedNames')}:</span
						>
						<span class="text-sm font-medium">
							{formatReservedNames($configStore.features.namedUrls.reservedNames)}
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
							class:text-green-600={$configStore.auth.enabled}
							class:dark:text-blue-300={$configStore.auth.enabled}
							class:text-red-600={!$configStore.auth.enabled}
							class:dark:text-red-400={!$configStore.auth.enabled}
						>
							{$configStore.auth.enabled
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm"
							>{$t('adminPage.general.fields.authType')}:</span
						>
						<span class="text-sm font-medium">{$configStore.auth.authType}</span>
					</div>
					{#if $configStore.auth.gitlab}
						<div class="flex items-start justify-between">
							<span class="text-muted-foreground text-sm"
								>{$t('adminPage.general.fields.gitlabBaseUrl')}:</span
							>
							<span class="max-w-[60%] text-right text-sm font-medium break-all">
								{$configStore.auth.gitlab.baseUrl}
							</span>
						</div>
						<div class="flex items-start justify-between">
							<span class="text-muted-foreground text-sm"
								>{$t('adminPage.general.fields.gitlabAppId')}:</span
							>
							<span
								class="max-w-[60%] overflow-hidden text-right text-sm font-medium text-ellipsis whitespace-nowrap"
								title={$configStore.auth.gitlab.applicationId}
							>
								{$configStore.auth.gitlab.applicationId}
							</span>
						</div>
					{/if}
					<div class="flex items-start justify-between">
						<span class="text-muted-foreground text-sm">{$t('adminPage.general.fields.note')}:</span
						>
						<span class="max-w-[60%] text-right text-sm font-medium">
							{$configStore.auth.note || $t('adminPage.general.values.none')}
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
						<span class="font-mono text-sm font-medium"
							>{$configStore.scheduler.cleanupExpiredUrls}</span
						>
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
							class:text-green-600={$configStore.metrics.enabled}
							class:dark:text-blue-300={$configStore.metrics.enabled}
							class:text-red-600={!$configStore.metrics.enabled}
							class:dark:text-red-400={!$configStore.metrics.enabled}
						>
							{$configStore.metrics.enabled
								? $t('adminPage.general.values.yes')
								: $t('adminPage.general.values.no')}
						</span>
					</div>
					{#if $configStore.metrics.enabled}
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
