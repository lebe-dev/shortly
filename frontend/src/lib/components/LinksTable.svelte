<script lang="ts" module>
	export interface LinkRow {
		id: string;
		shortUrl: string;
		originalUrl: string;
		created: number;
		ttl: number;
		lastAccessed?: number | null;
		username?: string | null;
		userId?: number | null;
	}
</script>

<script lang="ts">
	import { t } from 'svelte-intl-precompile';
	import { format } from 'date-fns';
	import Copy from 'lucide-svelte/icons/copy';
	import Trash2 from 'lucide-svelte/icons/trash-2';
	import { Button } from './ui/button';
	import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from './ui/table';
	import { formatRemainingTime } from '$lib/date';
	import { splitShortUrl, stripProtocol } from '$lib/display';

	interface Props {
		rows: LinkRow[];
		showUser?: boolean;
		showLastAccessed?: boolean;
		onCopy: (url: string) => void | Promise<void>;
		onDelete: (id: string) => void | Promise<void>;
	}

	let { rows, showUser = false, showLastAccessed = true, onCopy, onDelete }: Props = $props();

	function formatDate(seconds: number): string {
		return format(new Date(seconds * 1000), 'dd.MM.yyyy, HH:mm');
	}

	function expiry(row: LinkRow): { label: string; tone: 'never' | 'active' | 'expired' } {
		if (row.ttl === 0) {
			return { label: $t('common.never'), tone: 'never' };
		}
		const nowSeconds = Math.floor(Date.now() / 1000);
		if (row.created + row.ttl <= nowSeconds) {
			return { label: $t('linksPage.expired'), tone: 'expired' };
		}
		return { label: formatRemainingTime(row.ttl, row.created, $t, 'short'), tone: 'active' };
	}

	const toneText = {
		never: 'text-muted-foreground',
		active: 'text-foreground',
		expired: 'text-destructive'
	};

	const toneDot = {
		never: 'bg-input',
		active: 'bg-primary',
		expired: 'bg-destructive'
	};
</script>

<div class="surface-card overflow-hidden">
	<Table class={showUser ? 'min-w-[960px]' : 'min-w-[820px]'}>
		<TableHeader>
			<TableRow class="hover:[&,&>svelte-css-wrapper]:[&>th,td]:bg-muted">
				<TableHead>{$t('linksPage.table.shortUrl')}</TableHead>
				<TableHead>{$t('linksPage.table.originalUrl')}</TableHead>
				{#if showUser}
					<TableHead>{$t('adminPage.table.user')}</TableHead>
				{/if}
				<TableHead>{$t('linksPage.table.created')}</TableHead>
				<TableHead>{$t('linksPage.table.expires')}</TableHead>
				{#if showLastAccessed}
					<TableHead>{$t('linksPage.table.lastAccessed')}</TableHead>
				{/if}
				<TableHead><span class="sr-only">{$t('linksPage.table.actions')}</span></TableHead>
			</TableRow>
		</TableHeader>
		<TableBody>
			{#each rows as row (row.id)}
				{@const parts = splitShortUrl(row.shortUrl)}
				{@const exp = expiry(row)}
				<TableRow>
					<TableCell>
						<!-- eslint-disable svelte/no-navigation-without-resolve -->
						<a
							href={row.shortUrl}
							target="_blank"
							rel="noopener noreferrer"
							title={row.shortUrl}
							class="text-foreground hover:text-link font-mono text-[13px] font-medium transition-colors"
						>
							<span class="text-muted-foreground">/</span>{parts.slug}
						</a>
					</TableCell>
					<TableCell class="w-full max-w-0">
						<a
							href={row.originalUrl}
							target="_blank"
							rel="noopener noreferrer"
							title={row.originalUrl}
							class="text-muted-foreground hover:text-foreground block truncate font-mono text-[13px] transition-colors"
						>
							{stripProtocol(row.originalUrl)}
						</a>
						<!-- eslint-enable svelte/no-navigation-without-resolve -->
					</TableCell>
					{#if showUser}
						<TableCell>
							{#if row.username}
								<div class="flex items-center gap-2.5">
									<span
										class="bg-secondary text-foreground flex size-[26px] items-center justify-center rounded-full text-[11px] font-semibold uppercase"
										aria-hidden="true">{row.username.slice(0, 2)}</span
									>
									<span class="text-[13px]">{row.username}</span>
									{#if row.userId !== null && row.userId !== undefined}
										<span class="text-muted-foreground font-mono text-[11px]">#{row.userId}</span>
									{/if}
								</div>
							{:else if row.userId !== null && row.userId !== undefined}
								<span class="text-muted-foreground font-mono text-[13px]">#{row.userId}</span>
							{:else}
								<span class="text-muted-foreground">—</span>
							{/if}
						</TableCell>
					{/if}
					<TableCell class="text-muted-foreground text-[13px] tabular-nums"
						>{formatDate(row.created)}</TableCell
					>
					<TableCell>
						<span class="flex items-center gap-2 text-[13px] tabular-nums {toneText[exp.tone]}">
							<span class="size-1.5 rounded-full {toneDot[exp.tone]}" aria-hidden="true"></span>
							{exp.label}
						</span>
					</TableCell>
					{#if showLastAccessed}
						<TableCell class="text-muted-foreground text-[13px] tabular-nums">
							{row.lastAccessed ? formatDate(row.lastAccessed) : '—'}
						</TableCell>
					{/if}
					<TableCell class="w-px text-right">
						<div class="inline-flex gap-0.5">
							<Button
								variant="ghost"
								size="icon"
								title={$t('common.buttons.copy')}
								aria-label={$t('common.buttons.copy')}
								onclick={() => onCopy(row.shortUrl)}
							>
								<Copy />
							</Button>
							<Button
								variant="ghost"
								size="icon"
								class="hover:bg-destructive-soft hover:text-destructive"
								title={$t('common.buttons.delete')}
								aria-label={$t('common.buttons.delete')}
								onclick={() => onDelete(row.id)}
							>
								<Trash2 />
							</Button>
						</div>
					</TableCell>
				</TableRow>
			{/each}
		</TableBody>
	</Table>
</div>
