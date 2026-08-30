<script lang="ts">
	import { fetchAuditEvents } from '$lib/api/audit';
	import { AuditEventType, type AuditEvent, type AuditFilters } from '$lib/domain/audit';
	import { t } from 'svelte-intl-precompile';
	import { toast } from 'svelte-sonner';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import * as Table from '$lib/components/ui/table';
	import * as Pagination from '$lib/components/ui/pagination';
	import { RangeCalendar } from '$lib/components/ui/range-calendar';
	import * as Popover from '$lib/components/ui/popover';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { createSvelteTable, FlexRender } from '$lib/components/ui/data-table';
	import { getCoreRowModel, type ColumnDef, type CellContext } from '@tanstack/table-core';
	import { type DateValue } from '@internationalized/date';
	import { CalendarIcon } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import type { PageItem } from 'bits-ui';

	let events = $state<AuditEvent[]>([]);
	let loading = $state(true);
	let totalCount = $state(0);
	let currentPage = $state(1);
	let perPage = $state(20);
	let totalPages = $state(0);

	let filters = $state<AuditFilters>({});
	let eventTypeFilter = $state<string>('');
	let urlNameSearch = $state('');
	let usernameSearch = $state('');
	let dateRange = $state<{ start: DateValue | undefined; end: DateValue | undefined } | undefined>(
		undefined
	);

	$effect(() => {
		loadAuditEvents();
	});

	async function loadAuditEvents() {
		loading = true;
		try {
			const response = await fetchAuditEvents(currentPage, perPage, filters);
			events = response.events;
			totalCount = response.totalCount;
			totalPages = response.totalPages;
		} catch (e) {
			console.error('Failed to load audit events:', e);
			toast.error($t('adminPage.audit.errors.loadFailed'));
		} finally {
			loading = false;
		}
	}

	function applyFilters() {
		const newFilters: AuditFilters = {};

		if (eventTypeFilter && eventTypeFilter !== '') {
			newFilters.eventType = eventTypeFilter as AuditEventType;
		}
		if (urlNameSearch.trim()) {
			newFilters.urlName = urlNameSearch.trim();
		}
		if (usernameSearch.trim()) {
			newFilters.username = usernameSearch.trim();
		}
		if (dateRange?.start) {
			const date = new Date(dateRange.start.year, dateRange.start.month - 1, dateRange.start.day);
			newFilters.dateFrom = Math.floor(date.getTime() / 1000);
		}
		if (dateRange?.end) {
			const date = new Date(
				dateRange.end.year,
				dateRange.end.month - 1,
				dateRange.end.day,
				23,
				59,
				59
			);
			newFilters.dateTo = Math.floor(date.getTime() / 1000);
		}

		filters = newFilters;
		currentPage = 1;
		loadAuditEvents();
	}

	function clearFilters() {
		eventTypeFilter = '';
		urlNameSearch = '';
		usernameSearch = '';
		dateRange = undefined;
		filters = {};
		currentPage = 1;
		loadAuditEvents();
	}

	function formatTimestamp(timestamp: number): string {
		return new Date(timestamp * 1000).toLocaleString();
	}

	function formatUsername(username: string, userId: number): string {
		return `${username} (id: ${userId})`;
	}

	function formatUserColumn(event: AuditEvent): string {
		if (event.actorUserId === event.targetUserId) {
			return formatUsername(event.actorUsername, event.actorUserId);
		}
		return `${formatUsername(event.actorUsername, event.actorUserId)} → ${formatUsername(event.targetUsername, event.targetUserId)}`;
	}

	function formatDateRange(): string {
		if (!dateRange?.start && !dateRange?.end) {
			return $t('adminPage.audit.filters.selectDateRange');
		}
		if (dateRange?.start && dateRange?.end) {
			return `${dateRange.start.day}/${dateRange.start.month}/${dateRange.start.year} - ${dateRange.end.day}/${dateRange.end.month}/${dateRange.end.year}`;
		}
		if (dateRange?.start) {
			return `${$t('adminPage.audit.filters.dateFrom')}: ${dateRange.start.day}/${dateRange.start.month}/${dateRange.start.year}`;
		}
		return '';
	}

	function handlePageChange(newPage: number) {
		currentPage = newPage;
		loadAuditEvents();
	}

	const columns: ColumnDef<AuditEvent>[] = [
		{
			accessorKey: 'eventType',
			header: () => $t('adminPage.audit.table.eventType'),
			cell: (info: CellContext<AuditEvent, unknown>) =>
				$t(`adminPage.audit.eventTypes.${info.getValue()}`)
		},
		{
			accessorKey: 'actorUsername',
			header: () => $t('adminPage.audit.table.user'),
			cell: (info: CellContext<AuditEvent, unknown>) => formatUserColumn(info.row.original)
		},
		{
			accessorKey: 'urlName',
			header: () => $t('adminPage.audit.table.urlName'),
			cell: (info: CellContext<AuditEvent, unknown>) => info.getValue() || '—'
		},
		{
			accessorKey: 'createdAt',
			header: () => $t('adminPage.audit.table.timestamp'),
			cell: (info: CellContext<AuditEvent, unknown>) => formatTimestamp(info.getValue() as number)
		}
	];

	const table = createSvelteTable({
		get data() {
			return events;
		},
		columns,
		getCoreRowModel: getCoreRowModel()
	});
</script>

<svelte:head>
	<title>{$t('adminPage.title')} - {$t('adminPage.tabs.audit')}</title>
</svelte:head>

<div class="space-y-6">
	<!-- Filters Section -->
	<div class="border-muted rounded-lg border p-4">
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
			<!-- Event Type Filter -->
			<div>
				<Label for="eventType">{$t('adminPage.audit.filters.eventType')}</Label>
				<Select.Root bind:value={eventTypeFilter}>
					<Select.Trigger size="sm" class="mt-2 w-full">
						{#if eventTypeFilter === ''}
							{$t('adminPage.audit.filters.allEventTypes')}
						{:else if eventTypeFilter === AuditEventType.CreateUrl}
							{$t('adminPage.audit.eventTypes.create_url')}
						{:else if eventTypeFilter === AuditEventType.DeleteUrl}
							{$t('adminPage.audit.eventTypes.delete_url')}
						{:else if eventTypeFilter === AuditEventType.UserLogin}
							{$t('adminPage.audit.eventTypes.user_login')}
						{:else if eventTypeFilter === AuditEventType.UserLogout}
							{$t('adminPage.audit.eventTypes.user_logout')}
						{:else if eventTypeFilter === AuditEventType.UserQuotaUpdate}
							{$t('adminPage.audit.eventTypes.user_quota_update')}
						{:else if eventTypeFilter === AuditEventType.PasskeyRegister}
							{$t('adminPage.audit.eventTypes.passkey_register')}
						{:else if eventTypeFilter === AuditEventType.PasskeyDelete}
							{$t('adminPage.audit.eventTypes.passkey_delete')}
						{/if}
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="">
							{$t('adminPage.audit.filters.allEventTypes')}
						</Select.Item>
						<Select.Item value={AuditEventType.CreateUrl}>
							{$t('adminPage.audit.eventTypes.create_url')}
						</Select.Item>
						<Select.Item value={AuditEventType.DeleteUrl}>
							{$t('adminPage.audit.eventTypes.delete_url')}
						</Select.Item>
						<Select.Item value={AuditEventType.UserLogin}>
							{$t('adminPage.audit.eventTypes.user_login')}
						</Select.Item>
						<Select.Item value={AuditEventType.UserLogout}>
							{$t('adminPage.audit.eventTypes.user_logout')}
						</Select.Item>
						<Select.Item value={AuditEventType.UserQuotaUpdate}>
							{$t('adminPage.audit.eventTypes.user_quota_update')}
						</Select.Item>
						<Select.Item value={AuditEventType.PasskeyRegister}>
							{$t('adminPage.audit.eventTypes.passkey_register')}
						</Select.Item>
						<Select.Item value={AuditEventType.PasskeyDelete}>
							{$t('adminPage.audit.eventTypes.passkey_delete')}
						</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>

			<!-- URL Name Search -->
			<div>
				<Label for="urlName">{$t('adminPage.audit.filters.urlName')}</Label>
				<Input
					id="urlName"
					type="text"
					bind:value={urlNameSearch}
					placeholder={$t('adminPage.audit.filters.urlNamePlaceholder')}
					class="mt-2 h-10"
				/>
			</div>

			<!-- Username Search -->
			<div>
				<Label for="username">{$t('adminPage.audit.filters.username')}</Label>
				<Input
					id="username"
					type="text"
					bind:value={usernameSearch}
					placeholder={$t('adminPage.audit.filters.usernamePlaceholder')}
					class="mt-2 h-10"
				/>
			</div>

			<!-- Date Range Picker -->
			<div>
				<Label>{$t('adminPage.audit.filters.dateRange')}</Label>
				<Popover.Root>
					<Popover.Trigger class={buttonVariants({ size: 'sm', variant: 'outline' })}>
						{#snippet child({ props })}
							<button
								{...props}
								type="button"
								class={cn(
									'border-input focus-visible:border-primary dark:bg-input/30 dark:hover:bg-input/50 flex h-10 w-full items-center justify-start gap-2 rounded-sm border-2 bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none select-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
									'mt-2',
									!dateRange && 'text-muted-foreground'
								)}
							>
								<CalendarIcon class="pointer-events-none size-4 shrink-0 opacity-50" />
								<span class="truncate">{formatDateRange()}</span>
							</button>
						{/snippet}
					</Popover.Trigger>
					<Popover.Content class="w-auto p-0" align="start">
						<RangeCalendar bind:value={dateRange} />
					</Popover.Content>
				</Popover.Root>
			</div>
		</div>

		<div class="mt-4 flex gap-2">
			<Button size="sm" onclick={applyFilters}>{$t('adminPage.audit.filters.apply')}</Button>
			<Button size="sm" variant="outline" onclick={clearFilters}>
				{$t('adminPage.audit.filters.clear')}
			</Button>
		</div>
	</div>

	<!-- Results Count -->
	{#if !loading}
		<div class="text-muted-foreground ps-1 text-sm">
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			{@html $t('adminPage.audit.showing', { values: { count: events.length, total: totalCount } })}
		</div>
	{/if}

	<!-- Table -->
	{#if loading}
		<div class="py-12 text-center">
			<p class="text-muted-foreground">{$t('common.loading')}</p>
		</div>
	{:else if events.length === 0}
		<div class="py-12 text-center">
			<p class="text-muted-foreground text-lg">
				{$t('adminPage.audit.noEvents')}
			</p>
		</div>
	{:else}
		<Card>
			<CardContent>
				<Table.Root>
					<Table.Header>
						{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
							<Table.Row>
								{#each headerGroup.headers as header (header.id)}
									<Table.Head>
										{#if !header.isPlaceholder}
											<FlexRender
												content={header.column.columnDef.header}
												context={header.getContext()}
											/>
										{/if}
									</Table.Head>
								{/each}
							</Table.Row>
						{/each}
					</Table.Header>
					<Table.Body>
						{#each table.getRowModel().rows as row (row.id)}
							<Table.Row>
								{#each row.getVisibleCells() as cell (cell.id)}
									<Table.Cell>
										<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
									</Table.Cell>
								{/each}
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</CardContent>
		</Card>

		<!-- Pagination -->
		{#if totalPages > 1}
			<div class="flex justify-center">
				<Pagination.Root count={totalCount} {perPage} page={currentPage}>
					{#snippet children({ pages }: { pages: PageItem[] })}
						<Pagination.Content>
							<Pagination.Item>
								<Pagination.Previous
									onclick={() => {
										if (currentPage > 1) handlePageChange(currentPage - 1);
									}}
								/>
							</Pagination.Item>
							{#each pages as page (page.key)}
								{#if page.type === 'ellipsis'}
									<Pagination.Item>
										<Pagination.Ellipsis />
									</Pagination.Item>
								{:else}
									<Pagination.Item>
										<Pagination.Link
											{page}
											isActive={currentPage == page.value}
											onclick={() => handlePageChange(page.value)}
										>
											{page.value}
										</Pagination.Link>
									</Pagination.Item>
								{/if}
							{/each}
							<Pagination.Item>
								<Pagination.Next
									onclick={() => {
										if (currentPage < totalPages) handlePageChange(currentPage + 1);
									}}
								/>
							</Pagination.Item>
						</Pagination.Content>
					{/snippet}
				</Pagination.Root>
			</div>
		{/if}
	{/if}
</div>
