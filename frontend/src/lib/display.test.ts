import { describe, it, expect } from 'vitest';
import { limitLevel, limitPercent, splitShortUrl, stripProtocol } from './display';

describe('splitShortUrl', () => {
	it('splits host and slug and drops the protocol', () => {
		expect(splitShortUrl('https://s.example.com/onboarding')).toEqual({
			prefix: 's.example.com/',
			slug: 'onboarding'
		});
	});

	it('keeps nested base paths in the prefix', () => {
		expect(splitShortUrl('http://example.com/s/k7Pq2x')).toEqual({
			prefix: 'example.com/s/',
			slug: 'k7Pq2x'
		});
	});

	it('returns the whole value as slug when there is no slash', () => {
		expect(splitShortUrl('k7Pq2x')).toEqual({ prefix: '', slug: 'k7Pq2x' });
	});

	it('ignores a trailing slash', () => {
		expect(splitShortUrl('https://s.example.com/onboarding/')).toEqual({
			prefix: 's.example.com/',
			slug: 'onboarding'
		});
	});
});

describe('stripProtocol', () => {
	it('removes http and https schemes', () => {
		expect(stripProtocol('https://docs.example.com/a?b=1')).toBe('docs.example.com/a?b=1');
		expect(stripProtocol('http://example.com')).toBe('example.com');
	});

	it('keeps other schemes untouched', () => {
		expect(stripProtocol('ftp://example.com')).toBe('ftp://example.com');
	});
});

describe('limitLevel', () => {
	it('stays calm while usage is low', () => {
		expect(limitLevel(0, 20)).toBe('calm');
		expect(limitLevel(15, 20)).toBe('calm');
	});

	it('becomes near at 80 percent', () => {
		expect(limitLevel(16, 20)).toBe('near');
		expect(limitLevel(99, 100)).toBe('near');
	});

	it('is reached at or above the limit', () => {
		expect(limitLevel(20, 20)).toBe('reached');
		expect(limitLevel(25, 20)).toBe('reached');
	});

	it('treats a zero limit as reached', () => {
		expect(limitLevel(0, 0)).toBe('reached');
	});
});

describe('limitPercent', () => {
	it('rounds and clamps usage', () => {
		expect(limitPercent(37, 100)).toBe(37);
		expect(limitPercent(1, 3)).toBe(33);
		expect(limitPercent(30, 20)).toBe(100);
		expect(limitPercent(-1, 20)).toBe(0);
		expect(limitPercent(1, 0)).toBe(100);
	});
});
