import { describe, it, expect } from 'vitest';
import { isUrlValid } from './validator';

describe('isUrlValid', () => {
	describe('valid URLs with standard protocols', () => {
		it('should accept http URLs', () => {
			expect(isUrlValid('http://example.com')).toBe(true);
			expect(isUrlValid('http://example.com/path')).toBe(true);
			expect(isUrlValid('http://example.com:8080/path?query=value')).toBe(true);
		});

		it('should accept https URLs', () => {
			expect(isUrlValid('https://example.com')).toBe(true);
			expect(isUrlValid('https://example.com/path')).toBe(true);
			expect(isUrlValid('https://subdomain.example.com')).toBe(true);
		});

		it('should accept ftp URLs', () => {
			expect(isUrlValid('ftp://ftp.example.com')).toBe(true);
			expect(isUrlValid('ftp://ftp.example.com/file.txt')).toBe(true);
		});

		it('should accept gopher URLs', () => {
			expect(isUrlValid('gopher://example.com')).toBe(true);
			expect(isUrlValid('gopher://gopher.example.com/path')).toBe(true);
		});

		it('should accept file URLs', () => {
			expect(isUrlValid('file:///path/to/file.txt')).toBe(true);
			expect(isUrlValid('file:///C:/Users/file.txt')).toBe(true);
		});

		it('should accept WebSocket URLs', () => {
			expect(isUrlValid('ws://example.com')).toBe(true);
			expect(isUrlValid('wss://example.com/socket')).toBe(true);
		});

		it('should accept mailto URLs', () => {
			expect(isUrlValid('mailto:user@example.com')).toBe(true);
			expect(isUrlValid('mailto:user@example.com?subject=Hello')).toBe(true);
		});

		it('should accept data URLs', () => {
			expect(isUrlValid('data:text/plain;base64,SGVsbG8=')).toBe(true);
			expect(isUrlValid('data:image/png;base64,iVBORw0KGgo=')).toBe(true);
		});

		it('should accept tel URLs', () => {
			expect(isUrlValid('tel:+1234567890')).toBe(true);
			expect(isUrlValid('tel:123-456-7890')).toBe(true);
		});
	});

	describe('URLs with spaces and special characters', () => {
		it('should accept URLs with spaces', () => {
			expect(isUrlValid('http://example.com/path with spaces')).toBe(true);
			expect(isUrlValid('https://example.com/my file.pdf')).toBe(true);
		});

		it('should accept URLs with Cyrillic characters', () => {
			expect(isUrlValid('https://пример.рф')).toBe(true);
			expect(isUrlValid('https://example.com/путь')).toBe(true);
			expect(isUrlValid('http://пример.рф/страница')).toBe(true);
		});

		it('should accept URLs with other special characters', () => {
			expect(isUrlValid('https://example.com/path?query=hello world')).toBe(true);
			expect(isUrlValid('http://example.com/文档')).toBe(true);
			expect(isUrlValid('https://example.com/ñoño')).toBe(true);
		});

		it('should accept URLs with brackets and parentheses', () => {
			expect(isUrlValid('https://example.com/file(1).txt')).toBe(true);
			expect(isUrlValid('http://example.com/[test]')).toBe(true);
		});
	});

	describe('invalid URLs', () => {
		it('should reject empty strings', () => {
			expect(isUrlValid('')).toBe(false);
			expect(isUrlValid('   ')).toBe(false);
			expect(isUrlValid('\t\n')).toBe(false);
		});

		it('should reject URLs without protocol', () => {
			expect(isUrlValid('example.com')).toBe(false);
			expect(isUrlValid('www.example.com')).toBe(false);
			expect(isUrlValid('//example.com')).toBe(false);
		});

		it('should reject malformed URLs', () => {
			expect(isUrlValid('not a url')).toBe(false);
			expect(isUrlValid('http://')).toBe(false);
			expect(isUrlValid('http:/')).toBe(false);
			expect(isUrlValid('http:')).toBe(false);
		});

		it('should reject invalid protocols', () => {
			expect(isUrlValid('ht!tp://example.com')).toBe(false);
			expect(isUrlValid('123://example.com')).toBe(false);
		});

		it('should reject URLs with only whitespace after protocol', () => {
			expect(isUrlValid('http://   ')).toBe(false);
		});
	});

	describe('edge cases', () => {
		it('should accept URLs with IPv4 addresses', () => {
			expect(isUrlValid('http://192.168.1.1')).toBe(true);
			expect(isUrlValid('https://127.0.0.1:8080')).toBe(true);
		});

		it('should accept URLs with IPv6 addresses', () => {
			expect(isUrlValid('http://[2001:db8::1]')).toBe(true);
			expect(isUrlValid('https://[::1]:8080')).toBe(true);
		});

		it('should accept URLs with authentication', () => {
			expect(isUrlValid('http://user:pass@example.com')).toBe(true);
			expect(isUrlValid('ftp://anonymous@ftp.example.com')).toBe(true);
		});

		it('should accept URLs with fragments', () => {
			expect(isUrlValid('https://example.com#section')).toBe(true);
			expect(isUrlValid('http://example.com/page#fragment with spaces')).toBe(true);
		});

		it('should accept URLs with query parameters', () => {
			expect(isUrlValid('https://example.com?key=value&foo=bar')).toBe(true);
			expect(isUrlValid('http://example.com?search=hello world')).toBe(true);
		});

		it('should accept URLs with various TLDs', () => {
			expect(isUrlValid('https://example.co.uk')).toBe(true);
			expect(isUrlValid('http://example.museum')).toBe(true);
			expect(isUrlValid('https://example.xn--p1ai')).toBe(true); // Punycode TLD
		});
	});
});
