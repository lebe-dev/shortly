<script lang="ts">
	import { t } from 'svelte-intl-precompile';
	import { Button } from './ui/button';
	import { Card, CardHeader, CardTitle, CardContent } from './ui/card';
	import Trash2 from 'lucide-svelte/icons/trash-2';
	import Copy from 'lucide-svelte/icons/copy';
	import CornerDownRight from 'lucide-svelte/icons/corner-down-right';
	import { formatCreatedDate } from '$lib/date';

	interface Props {
		id: string;
		url: string;
		originalUrl: string;
		created: number;
		ttl: number;
		customName?: string | null;
		username?: string | null;
		userId?: number | null;
		lastAccessed?: number | null;
		onDelete: (id: string) => Promise<void>;
		onCopy: (url: string) => Promise<void>;
		showUserInfo?: boolean;
	}

	let {
		id,
		url,
		originalUrl,
		created,
		ttl,
		customName = null,
		username = null,
		userId = null,
		lastAccessed = null,
		onDelete,
		onCopy,
		showUserInfo = false
	}: Props = $props();

	function getShortUrlDisplay(): string {
		return customName || id;
	}

	function formatExpires(): string {
		if (ttl === 0) {
			return '∞';
		}

		const now = Math.floor(Date.now() / 1000);
		const expiresAt = created + ttl;
		const remaining = expiresAt - now;

		if (remaining <= 0) {
			return $t('linksPage.expired');
		}

		const days = Math.floor(remaining / 86400);
		const hours = Math.floor((remaining % 86400) / 3600);
		const minutes = Math.floor((remaining % 3600) / 60);

		if (days > 0) {
			return `${days}${$t('linksPage.timeRemaining.dayShort')} ${hours}${$t('linksPage.timeRemaining.hourShort')}`;
		} else if (hours > 0) {
			return `${hours}${$t('linksPage.timeRemaining.hourShort')} ${minutes}${$t('linksPage.timeRemaining.minuteShort')}`;
		} else {
			return `${minutes}${$t('linksPage.timeRemaining.minuteShort')}`;
		}
	}

	function isExpired(): boolean {
		if (ttl === 0) return false;
		const now = Math.floor(Date.now() / 1000);
		const expiresAt = created + ttl;
		return expiresAt < now;
	}
</script>

<Card class={customName ? 'border-blue-200 dark:border-blue-900' : ''}>
	<CardHeader>
		<CardTitle>
			<div class="flex items-center justify-between gap-2">
				<div class="flex items-center gap-2">
					<a
						href={url}
						class="text-blue-600 hover:underline dark:text-blue-400"
						target="_blank"
						rel="noopener noreferrer"
						title={url}
					>
						{getShortUrlDisplay()}
					</a>
					<Button
						variant="ghost"
						size="sm"
						class="h-6 w-6 p-0 text-gray-400 hover:cursor-pointer"
						title={$t('common.buttons.copy')}
						onclick={() => onCopy(url)}
					>
						<Copy class="h-2 w-2" />
					</Button>
				</div>
				<Button
					variant="ghost"
					class="h-6 w-6 text-gray-400  hover:cursor-pointer hover:bg-red-100 hover:text-red-600 dark:text-gray-500 dark:hover:bg-red-900/20 dark:hover:text-red-400"
					size="sm"
					onclick={() => onDelete(id)}
				>
					<Trash2 class="h-3 w-3" />
				</Button>
			</div>
		</CardTitle>
	</CardHeader>
	<CardContent>
		<div class="space-y-3">
			<!-- Original URL -->
			<div class="space-y-1">
				<div class="text-muted-foreground text-xs font-medium">
					{$t('linksPage.table.originalUrl')}
				</div>
				<div class="flex items-center gap-1">
					<CornerDownRight class="text-muted-foreground h-3 w-3 shrink-0" />
					<a
						href={originalUrl}
						class="truncate text-xs text-blue-600 hover:underline dark:text-blue-400"
						target="_blank"
						rel="noopener noreferrer"
						title={originalUrl}
					>
						{originalUrl}
					</a>
					<Button
						variant="ghost"
						size="sm"
						class="h-6 w-6 shrink-0 p-0 text-gray-400 hover:cursor-pointer"
						title={$t('common.buttons.copy')}
						onclick={() => onCopy(originalUrl)}
					>
						<Copy class="h-2 w-2" />
					</Button>
				</div>
			</div>

			<!-- User Info (only for admin) -->
			{#if showUserInfo && (username || userId !== null)}
				<div class="space-y-1">
					<div class="text-muted-foreground text-xs font-medium">
						{$t('adminPage.table.user')}
					</div>
					<div class="text-muted-foreground font-mono text-xs">
						{#if username}
							{username} (id: {userId})
						{:else if userId !== null}
							{userId}
						{/if}
					</div>
				</div>
			{/if}

			<!-- Created date and Expires -->
			<div
				class="flex items-start justify-between border-t border-gray-200 pt-3 dark:border-gray-700"
			>
				<div class="space-y-1">
					<div class="text-muted-foreground text-xs font-medium">
						{$t('linksPage.table.created')}
					</div>
					<div class="text-muted-foreground text-sm">
						{formatCreatedDate(created)}
					</div>
				</div>
				<div class="space-y-1 text-right">
					<div class="text-muted-foreground text-xs font-medium">
						{$t('linksPage.table.expires')}
					</div>
					<div
						class="text-muted-foreground text-sm"
						class:text-red-600={isExpired()}
						class:dark:text-red-400={isExpired()}
					>
						{formatExpires()}
					</div>
				</div>
			</div>

			<!-- Last Accessed -->
			{#if lastAccessed}
				<div class="space-y-1 border-t border-gray-200 pt-3 dark:border-gray-700">
					<div class="text-muted-foreground text-xs font-medium">
						{$t('linksPage.table.lastAccessed')}
					</div>
					<div class="text-muted-foreground text-sm">
						{formatCreatedDate(lastAccessed)}
					</div>
				</div>
			{/if}
		</div>
	</CardContent>
</Card>
