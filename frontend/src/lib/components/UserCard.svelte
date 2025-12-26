<script lang="ts">
	import { t } from 'svelte-intl-precompile';
	import { Card, CardContent, CardHeader, CardTitle } from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import ShieldCheck from 'lucide-svelte/icons/shield-check';
	import Zap from 'lucide-svelte/icons/zap';
	import Pencil from 'lucide-svelte/icons/pencil';
	import Check from 'lucide-svelte/icons/check';
	import X from 'lucide-svelte/icons/x';
	import { toast } from 'svelte-sonner';
	import { updateUserQuotas } from '$lib/api/user';
	import type { AdminUserDto } from '$lib/domain/config';

	interface Props {
		user: AdminUserDto;
		onQuotasUpdated?: (updatedUser: { maxUrlsPerUser: number; maxUrlsPerDay: number }) => void;
	}

	let { user, onQuotasUpdated }: Props = $props();

	let isEditing = $state(false);
	let editMaxPerUser = $state(user.maxUrlsPerUser);
	let editMaxPerDay = $state(user.maxUrlsPerDay);
	let isSaving = $state(false);

	function formatDate(timestamp: number): string {
		const date = new Date(timestamp * 1000);
		const day = String(date.getDate()).padStart(2, '0');
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const year = date.getFullYear();
		return `${day}.${month}.${year}`;
	}

	function getInitials(username: string): string {
		return username.substring(0, 2).toUpperCase();
	}

	function startEditing() {
		editMaxPerUser = user.maxUrlsPerUser;
		editMaxPerDay = user.maxUrlsPerDay;
		isEditing = true;
	}

	function cancelEditing() {
		isEditing = false;
		editMaxPerUser = user.maxUrlsPerUser;
		editMaxPerDay = user.maxUrlsPerDay;
	}

	async function saveQuotas() {
		if (editMaxPerUser < 0 || editMaxPerDay < 0) {
			toast.error($t('adminPage.users.quotasMustBeNonNegative'));
			return;
		}

		if (editMaxPerUser === user.maxUrlsPerUser && editMaxPerDay === user.maxUrlsPerDay) {
			isEditing = false;
			return;
		}

		isSaving = true;
		try {
			const updated = await updateUserQuotas(user.id, {
				maxUrlsPerUser: editMaxPerUser,
				maxUrlsPerDay: editMaxPerDay
			});

			toast.success($t('adminPage.users.quotasUpdated'));
			isEditing = false;

			if (onQuotasUpdated) {
				onQuotasUpdated({
					maxUrlsPerUser: updated.maxUrlsPerUser,
					maxUrlsPerDay: updated.maxUrlsPerDay
				});
			}
		} catch (error) {
			console.error('Failed to update quotas:', error);
			toast.error($t('adminPage.users.quotasUpdateFailed'));
		} finally {
			isSaving = false;
		}
	}
</script>

<Card>
	<CardHeader>
		<div class="flex items-center gap-3">
			<!-- Avatar or Initials -->
			{#if user.avatarUrl}
				<img src={user.avatarUrl} alt={user.username} class="h-12 w-12 rounded-full" />
			{:else}
				<div class="flex h-12 w-12 items-center justify-center rounded-full bg-blue-500 text-white">
					<span class="text-sm font-semibold">{getInitials(user.username)}</span>
				</div>
			{/if}

			<div class="flex-1 overflow-hidden">
				<div class="flex items-center gap-2">
					<CardTitle class={`truncate text-lg ${user.isAdmin ? 'text-primary' : ''}`}>
						{user.username}
					</CardTitle>
					{#if user.isAdmin}
						<Zap class="text-primary h-4 w-4 flex-shrink-0" />
					{/if}
				</div>
				{#if user.email}
					<p class="text-muted-foreground truncate text-xs">{user.email}</p>
				{/if}
			</div>
		</div>
	</CardHeader>

	<CardContent>
		<div class="space-y-3">
			<!-- User ID and Created -->
			<div class="flex items-center justify-between text-sm">
				<span class="text-muted-foreground">{$t('adminPage.users.userId')}</span>
				<span class="font-medium">#{user.id}</span>
			</div>

			<div class="flex items-center justify-between text-sm">
				<span class="text-muted-foreground">{$t('adminPage.users.created')}</span>
				<span class="font-medium">{formatDate(user.createdAt)}</span>
			</div>

			<div class="flex items-center justify-between text-sm">
				<span class="text-muted-foreground">{$t('adminPage.users.urlCount')}</span>
				<span class="font-medium">{user.urlCount}</span>
			</div>

			<!-- Quotas -->
			<div class="border-t pt-3">
				<div class="mb-2 flex items-center justify-between">
					<div class="flex items-center gap-2">
						<ShieldCheck class="text-muted-foreground h-4 w-4" />
						<span class="text-muted-foreground text-xs font-medium"
							>{$t('adminPage.users.quotas')}</span
						>
					</div>
					{#if !isEditing}
						<Button variant="ghost" size="sm" class="h-6 w-6 p-0" onclick={startEditing}>
							<Pencil class="h-3 w-3" />
						</Button>
					{/if}
				</div>

				{#if isEditing}
					<div class="space-y-2">
						<div class="flex items-center gap-2">
							<span class="text-muted-foreground w-20 text-xs"
								>{$t('adminPage.users.maxPerUser')}</span
							>
							<Input
								type="number"
								bind:value={editMaxPerUser}
								min="0"
								max="99999"
								class="h-7 w-20 text-xs"
								disabled={isSaving}
							/>
						</div>
						<div class="flex items-center gap-2">
							<span class="text-muted-foreground w-20 text-xs"
								>{$t('adminPage.users.maxPerDay')}</span
							>
							<Input
								type="number"
								bind:value={editMaxPerDay}
								min="0"
								max="99999"
								class="h-7 w-20 text-xs"
								disabled={isSaving}
							/>
						</div>
						<div class="flex justify-end gap-1 pt-1">
							<Button
								variant="ghost"
								size="sm"
								class="h-7 px-2"
								onclick={cancelEditing}
								disabled={isSaving}
							>
								<X class="h-3 w-3" />
							</Button>
							<Button
								variant="default"
								size="sm"
								class="h-7 px-2"
								onclick={saveQuotas}
								disabled={isSaving}
							>
								<Check class="h-3 w-3" />
							</Button>
						</div>
					</div>
				{:else}
					<div class="space-y-1">
						<div class="flex items-center justify-between text-xs">
							<span class="text-muted-foreground">{$t('adminPage.users.maxPerUser')}</span>
							<span class="font-medium">{user.maxUrlsPerUser}</span>
						</div>
						<div class="flex items-center justify-between text-xs">
							<span class="text-muted-foreground">{$t('adminPage.users.maxPerDay')}</span>
							<span class="font-medium">{user.maxUrlsPerDay}</span>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</CardContent>
</Card>
