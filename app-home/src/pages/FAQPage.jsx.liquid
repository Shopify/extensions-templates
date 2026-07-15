import {useState, useEffect, useRef} from 'preact/hooks';
import {useLocation} from 'preact-iso';

import {createFAQ, deleteFAQ, EMPTY_FAQ, getFAQ, updateFAQ} from '../models/faqs.js';

export default function FAQPage({id}) {
  const {route} = useLocation();
  const isNew = !id || id === 'new';
  const [faq, setFaq] = useState({...EMPTY_FAQ});
  const snapshot = useRef({...EMPTY_FAQ});
  const [status, setStatus] = useState(isNew ? 'idle' : 'loading');
  const [error, setError] = useState(null);
  const [fieldErrors, setFieldErrors] = useState(emptyFieldErrors());

  useEffect(() => {
    if (isNew) {
      const emptyFAQ = {...EMPTY_FAQ};
      snapshot.current = emptyFAQ;
      setFaq(emptyFAQ);
      setStatus('idle');
      setError(null);
      setFieldErrors(emptyFieldErrors());
      return;
    }

    loadFAQ();
  }, [id]);

  async function loadFAQ() {
    setStatus('loading');
    setError(null);

    try {
      const existingFAQ = await getFAQ(id);

      if (!existingFAQ) {
        setStatus('not-found');
        return;
      }

      const nextFAQ = {
        id: existingFAQ.id,
        question: existingFAQ.question || '',
        answer: existingFAQ.answer || '',
      };
      snapshot.current = nextFAQ;
      setFaq(nextFAQ);
      setFieldErrors(emptyFieldErrors());
      setStatus('idle');
    } catch (err) {
      console.error('Failed to load FAQ:', err);
      setError('Failed to load FAQ');
      setStatus('idle');
    }
  }

  function setFaqField(key, value) {
    setFaq((currentFAQ) => ({...currentFAQ, [key]: value}));
    setFieldErrors((errors) => ({...errors, [key]: undefined}));
  }

  async function saveFAQ() {
    const question = faq.question.trim();
    const answer = faq.answer.trim();
    const nextFieldErrors = emptyFieldErrors();

    if (!question) nextFieldErrors.question = 'Question is required';
    if (!answer) nextFieldErrors.answer = 'Answer is required';

    if (nextFieldErrors.question || nextFieldErrors.answer) {
      setFieldErrors(nextFieldErrors);
      throw new Error('Validation failed');
    }

    setStatus('saving');
    setError(null);

    try {
      const savedFAQ = isNew
        ? await createFAQ({question, answer})
        : await updateFAQ(id, {question, answer});

      snapshot.current = savedFAQ;
      setFaq(savedFAQ);
      setFieldErrors(emptyFieldErrors());
      setStatus('idle');
      return savedFAQ;
    } catch (err) {
      console.error('Failed to save FAQ:', err);
      setError(isNew ? 'Failed to save FAQ' : 'Failed to update FAQ');
      setStatus('idle');
      throw err;
    }
  }

  async function handleSubmit(event) {
    const promise = saveFAQ();
    event?.waitUntil?.(promise);
    await promise;

    if (isNew) {
      route('/');
    }
  }

  function handleReset() {
    setFaq({...snapshot.current});
    setFieldErrors(emptyFieldErrors());
  }

  async function handleDelete() {
    if (isNew) return;

    setStatus('deleting');
    setError(null);

    try {
      await deleteFAQ(id);
      route('/');
    } catch (err) {
      console.error('Failed to delete FAQ:', err);
      setError('Failed to delete FAQ');
      setStatus('idle');
    }
  }

  if (status === 'not-found') {
    return (
      <s-page heading="FAQ not found">
        <s-link slot="breadcrumb-actions" href="/">
          FAQs
        </s-link>
        <s-section>
          <s-grid gap="base" justifyItems="center" paddingBlock="large-400">
            <s-paragraph>The FAQ you're looking for doesn't exist.</s-paragraph>
            <s-button href="/">Go home</s-button>
          </s-grid>
        </s-section>
      </s-page>
    );
  }

  const isLoading = status === 'loading';
  const heading = isNew ? 'Add New FAQ' : isLoading ? 'Loading FAQ' : faq.question || 'Edit FAQ';

  return (
    <s-page heading={heading} inlineSize="small">
      <s-link slot="breadcrumb-actions" href="/">
        FAQs
      </s-link>

      {!isNew && (
        <s-button
          slot="secondary-actions"
          onClick={handleDelete}
          loading={status === 'deleting'}
          disabled={isLoading}
          tone="critical"
        >
          Delete
        </s-button>
      )}

      {error && (
        <s-section>
          <s-banner tone="critical">{error}</s-banner>
        </s-section>
      )}

      {isLoading && (
        <s-section>
          <s-paragraph>Loading...</s-paragraph>
        </s-section>
      )}

      {!isLoading && (
        <s-form id="faq-form" onSubmit={handleSubmit} onReset={handleReset}>
          <s-section heading="FAQ details">
            <s-stack gap="base">
              <s-text-field
                label="Question"
                name="question"
                value={faq.question}
                onInput={(event) => setFaqField('question', inputValue(event))}
                error={fieldErrors.question}
                required
              />

              <s-text-area
                label="Answer"
                name="answer"
                value={faq.answer}
                onInput={(event) => setFaqField('answer', inputValue(event))}
                error={fieldErrors.answer}
                rows={4}
                required
              />
            </s-stack>
          </s-section>
        </s-form>
      )}
    </s-page>
  );
}

function inputValue(event) {
  const target = event.target;
  return target && 'value' in target ? String(target.value) : '';
}

function emptyFieldErrors() {
  return {question: undefined, answer: undefined};
}
