import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import test from 'node:test';

const faqModel = await readFile(
  new URL('../../app-home/src/models/faqs.js.liquid', import.meta.url),
  'utf8',
);

test('FAQ model persists structured values with the App Home Storage API', async () => {
  const values = new Map();
  const storage = {
    async get(key) {
      const value = values.get(key);
      return value === undefined ? undefined : JSON.parse(value);
    },
    async set(key, value) {
      values.set(key, JSON.stringify(value));
    },
  };

  globalThis.shopify = {storage};

  try {
    const moduleUrl =
      'data:text/javascript;base64,' +
      Buffer.from(faqModel).toString('base64');
    const faqs = await import(moduleUrl);

    assert.deepEqual(await faqs.listFAQs(), []);

    const created = await faqs.createFAQ({
      question: 'Where is my order?',
      answer: 'Check your order confirmation email.',
    });

    assert.deepEqual(await storage.get('faqs'), [created]);
    assert.deepEqual(await faqs.listFAQs(), [created]);
    assert.deepEqual(await faqs.getFAQ(created.id), created);

    const updated = await faqs.updateFAQ(created.id, {
      question: 'How do I track my order?',
      answer: 'Use the tracking link in your confirmation email.',
    });

    assert.deepEqual(await storage.get('faqs'), [updated]);
    assert.deepEqual(await faqs.listFAQs(), [updated]);

    await faqs.deleteFAQ(updated.id);

    assert.deepEqual(await storage.get('faqs'), []);
    assert.deepEqual(await faqs.listFAQs(), []);
  } finally {
    delete globalThis.shopify;
  }
});
