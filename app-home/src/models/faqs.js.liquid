export const EMPTY_FAQ = {question: '', answer: ''};

const FAQ_STORAGE_KEY = 'faqs';

export async function listFAQs() {
  const stored = await shopify.storage.getItem(FAQ_STORAGE_KEY);
  const parsedFaqs = stored ? JSON.parse(stored) : [];
  return Array.isArray(parsedFaqs) ? parsedFaqs : [];
}

export async function getFAQ(id) {
  const faqs = await listFAQs();
  return faqs.find((faq) => String(faq.id) === String(id));
}

export async function createFAQ(faq) {
  const faqs = await listFAQs();
  const savedFAQ = {
    id: Date.now(),
    question: faq.question,
    answer: faq.answer,
  };
  await writeFAQs([...faqs, savedFAQ]);
  return savedFAQ;
}

export async function updateFAQ(id, faq) {
  const faqs = await listFAQs();
  const savedFAQ = {
    id,
    question: faq.question,
    answer: faq.answer,
  };
  await writeFAQs(
    faqs.map((storedFAQ) =>
      String(storedFAQ.id) === String(id) ? savedFAQ : storedFAQ,
    ),
  );
  return savedFAQ;
}

export async function deleteFAQ(id) {
  const faqs = await listFAQs();
  await writeFAQs(faqs.filter((faq) => String(faq.id) !== String(id)));
}

async function writeFAQs(faqs) {
  await shopify.storage.setItem(FAQ_STORAGE_KEY, JSON.stringify(faqs));
}
