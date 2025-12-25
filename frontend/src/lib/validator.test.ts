import { describe, it, expect } from 'vitest';
import { isUrlValid } from './validator';

const MAX_LENGTH = 2048;

describe('isUrlValid', () => {
	describe('valid URLs with standard protocols', () => {
		it('should accept http URLs', () => {
			expect(isUrlValid('http://example.com', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://example.com/path', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://example.com:8080/path?query=value', MAX_LENGTH)).toBe(true);
		});

		it('should accept https URLs', () => {
			expect(isUrlValid('https://example.com', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://example.com/path', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://subdomain.example.com', MAX_LENGTH)).toBe(true);
		});

		it('should accept ftp URLs', () => {
			expect(isUrlValid('ftp://ftp.example.com', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('ftp://ftp.example.com/file.txt', MAX_LENGTH)).toBe(true);
		});

		it('should accept gopher URLs', () => {
			expect(isUrlValid('gopher://example.com', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('gopher://gopher.example.com/path', MAX_LENGTH)).toBe(true);
		});

		it('should accept file URLs', () => {
			expect(isUrlValid('file:///path/to/file.txt', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('file:///C:/Users/file.txt', MAX_LENGTH)).toBe(true);
		});

		it('should accept WebSocket URLs', () => {
			expect(isUrlValid('ws://example.com', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('wss://example.com/socket', MAX_LENGTH)).toBe(true);
		});

		it('should accept mailto URLs', () => {
			expect(isUrlValid('mailto:user@example.com', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('mailto:user@example.com?subject=Hello', MAX_LENGTH)).toBe(true);
		});

		it('should accept data URLs', () => {
			expect(isUrlValid('data:text/plain;base64,SGVsbG8=', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('data:image/png;base64,iVBORw0KGgo=', MAX_LENGTH)).toBe(true);
		});

		it('should accept tel URLs', () => {
			expect(isUrlValid('tel:+1234567890', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('tel:123-456-7890', MAX_LENGTH)).toBe(true);
		});
	});

	describe('URLs with spaces and special characters', () => {
		it('should accept URLs with spaces', () => {
			expect(isUrlValid('http://example.com/path with spaces', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://example.com/my file.pdf', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with Cyrillic characters', () => {
			expect(isUrlValid('https://пример.рф', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://example.com/путь', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://пример.рф/страница', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with other special characters', () => {
			expect(isUrlValid('https://example.com/path?query=hello world', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://example.com/文档', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://example.com/ñoño', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with brackets and parentheses', () => {
			expect(isUrlValid('https://example.com/file(1).txt', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://example.com/[test]', MAX_LENGTH)).toBe(true);
		});
	});

	describe('invalid URLs', () => {
		it('should reject empty strings', () => {
			expect(isUrlValid('', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('   ', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('\t\n', MAX_LENGTH)).toBe(false);
		});

		it('should reject URLs without protocol', () => {
			expect(isUrlValid('example.com', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('www.example.com', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('//example.com', MAX_LENGTH)).toBe(false);
		});

		it('should reject malformed URLs', () => {
			expect(isUrlValid('not a url', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('http://', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('http:/', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('http:', MAX_LENGTH)).toBe(false);
		});

		it('should reject invalid protocols', () => {
			expect(isUrlValid('ht!tp://example.com', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('123://example.com', MAX_LENGTH)).toBe(false);
		});

		it('should reject URLs with only whitespace after protocol', () => {
			expect(isUrlValid('http://   ', MAX_LENGTH)).toBe(false);
		});

		it('should reject http/https URLs without proper domain', () => {
			expect(isUrlValid('https://adasdadas', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('http://test', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('https://singleword', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('http://no-dot-here', MAX_LENGTH)).toBe(false);
		});

		it('should reject http/https URLs with invalid domain structure', () => {
			expect(isUrlValid('https://example.', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('http://.example.com', MAX_LENGTH)).toBe(false);
			expect(isUrlValid('https://example..com', MAX_LENGTH)).toBe(false);
		});
	});

	describe('edge cases', () => {
		it('should accept localhost URLs', () => {
			expect(isUrlValid('http://localhost', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://localhost:3000', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://localhost/path', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with IPv4 addresses', () => {
			expect(isUrlValid('http://192.168.1.1', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://127.0.0.1:8080', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://10.0.0.1', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with IPv6 addresses', () => {
			expect(isUrlValid('http://[2001:db8::1]', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://[::1]:8080', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://[fe80::1]', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with authentication', () => {
			expect(isUrlValid('http://user:pass@example.com', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('ftp://anonymous@ftp.example.com', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with fragments', () => {
			expect(isUrlValid('https://example.com#section', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://example.com/page#fragment with spaces', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with query parameters', () => {
			expect(isUrlValid('https://example.com?key=value&foo=bar', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://example.com?search=hello world', MAX_LENGTH)).toBe(true);
		});

		it('should accept URLs with various TLDs', () => {
			expect(isUrlValid('https://example.co.uk', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('http://example.museum', MAX_LENGTH)).toBe(true);
			expect(isUrlValid('https://example.xn--p1ai', MAX_LENGTH)).toBe(true); // Punycode TLD
		});
	});

	describe('self-referencing URL prevention', () => {
		const BASE_URL = 'https://short.ly';

		it('should reject URLs that start with baseUrl', () => {
			expect(isUrlValid('https://short.ly/abc123', MAX_LENGTH, BASE_URL)).toBe(false);
			expect(isUrlValid('https://short.ly/', MAX_LENGTH, BASE_URL)).toBe(false);
			expect(isUrlValid('https://short.ly', MAX_LENGTH, BASE_URL)).toBe(false);
		});

		it('should reject URLs with different case', () => {
			expect(isUrlValid('HTTPS://SHORT.LY/abc123', MAX_LENGTH, BASE_URL)).toBe(false);
			expect(isUrlValid('https://Short.Ly/test', MAX_LENGTH, BASE_URL)).toBe(false);
		});

		it('should handle baseUrl with trailing slash', () => {
			const baseUrlWithSlash = 'https://short.ly/';
			expect(isUrlValid('https://short.ly/abc', MAX_LENGTH, baseUrlWithSlash)).toBe(false);
			expect(isUrlValid('https://short.ly', MAX_LENGTH, baseUrlWithSlash)).toBe(false);
		});

		it('should accept URLs from different domains', () => {
			expect(isUrlValid('https://example.com/path', MAX_LENGTH, BASE_URL)).toBe(true);
			expect(isUrlValid('https://short.ly.evil.com', MAX_LENGTH, BASE_URL)).toBe(true);
		});
	});
});
