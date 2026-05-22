import {render} from 'preact';
import {useState, useEffect} from 'preact/hooks';

export default async () => {
  render(<App />, document.body);
};

const FAQ_STORAGE_KEY = 'faqs';

function App() {
  const [currentView, setCurrentView] = useState('list');
  const [faqs, setFaqs] = useState([]);
  const [newQuestion, setNewQuestion] = useState('');
  const [newAnswer, setNewAnswer] = useState('');
  const [loading, setLoading] = useState(true);

  // Load FAQs from storage on mount
  useEffect(() => {
    loadFAQs();
  }, []);

  async function readFAQs() {
    const stored = await shopify.storage.getItem(FAQ_STORAGE_KEY);
    const parsedFaqs = stored ? JSON.parse(stored) : [];
    return Array.isArray(parsedFaqs) ? parsedFaqs : [];
  }

  async function loadFAQs() {
    setLoading(true);
    try {
      setFaqs(await readFAQs());
    } catch (err) {
      console.error('Failed to load FAQs:', err);
      setFaqs([]);
    } finally {
      setLoading(false);
    }
  }

  async function saveFAQ() {
    const question = newQuestion.trim();
    const answer = newAnswer.trim();

    if (!question || !answer) {
      return;
    }

    const newFaq = {
      id: Date.now(),
      question,
      answer,
    };

    try {
      const latestFaqs = await readFAQs();
      const updatedFaqs = [...latestFaqs, newFaq];
      await shopify.storage.setItem(FAQ_STORAGE_KEY, JSON.stringify(updatedFaqs));
      setFaqs(updatedFaqs);
      setNewQuestion('');
      setNewAnswer('');
      setCurrentView('list');
    } catch (err) {
      console.error('Failed to save FAQ:', err);
      throw err;
    }
  }

  async function deleteFAQ(id) {
    try {
      const latestFaqs = await readFAQs();
      const updatedFaqs = latestFaqs.filter((faq) => faq.id !== id);
      await shopify.storage.setItem(FAQ_STORAGE_KEY, JSON.stringify(updatedFaqs));
      setFaqs(updatedFaqs);
    } catch (err) {
      console.error('Failed to delete FAQ:', err);
    }
  }

  // Add FAQ view
  if (currentView === 'add') {
    const handleSubmit = (event) => {
      const promise = saveFAQ();
      event?.waitUntil?.(promise);
      return promise;
    };

    const handleReset = () => {
      setNewQuestion('');
      setNewAnswer('');
      setCurrentView('list');
    };

    return (
      <s-page heading="Add New FAQ">
        <s-form id="add-faq-form" onSubmit={handleSubmit} onReset={handleReset}>
          <s-section>
            <s-stack gap="base">
              <s-text-field
                label="Question"
                name="question"
                value={newQuestion}
                oninput={(e) => setNewQuestion(e.target.value)}
              />

              <s-text-area
                label="Answer"
                name="answer"
                value={newAnswer}
                oninput={(e) => setNewAnswer(e.target.value)}
                rows={4}
              />
            </s-stack>
          </s-section>
        </s-form>
      </s-page>
    );
  }

  // List view (default)
  const hasFaqs = faqs.length > 0;

  return (
    <s-page heading="FAQ Manager">
      <s-button
        slot="primary-action"
        variant="primary"
        onclick={() => setCurrentView('add')}
      >
        Add FAQ
      </s-button>

      <s-section slot="aside" heading="About this template">
        <s-paragraph>
          <s-text>{shopify.i18n.translate('welcome', {target: shopify.extension.target})}</s-text>
        </s-paragraph>
        <s-stack direction="inline" gap="small" alignItems="center">
          <s-paragraph>
            <s-text>Framework: </s-text>
            <s-link href="https://preactjs.com/" target="_blank">
              Preact
            </s-link>
          </s-paragraph>
          <s-link href="https://preactjs.com/" target="_blank">
            <s-box maxInlineSize="24px" maxBlockSize="24px">
              <s-image src="./preact-logo.png" alt="Preact logo" />
            </s-box>
          </s-link>
        </s-stack>
        <s-stack direction="inline" gap="small" alignItems="center">
          <s-paragraph>
            <s-text>Interface: </s-text>
            <s-link
              href="https://shopify.dev/docs/api/app-home/web-components"
              target="_blank"
            >
              Polaris web components
            </s-link>
          </s-paragraph>
          <s-link
            href="https://shopify.dev/docs/api/app-home/web-components"
            target="_blank"
          >
            <s-box maxInlineSize="32px" maxBlockSize="32px">
              <s-image src="./polaris-icon.png" alt="Polaris logo" />
            </s-box>
          </s-link>
        </s-stack>
        <s-stack direction="inline" gap="small" alignItems="center">
          <s-paragraph>
            <s-text>API: </s-text>
            <s-link
              href="https://shopify.dev/docs/api/admin-graphql"
              target="_blank"
            >
              GraphQL Admin API
            </s-link>
          </s-paragraph>
          <s-link
            href="https://shopify.dev/docs/api/admin-graphql"
            target="_blank"
          >
            <s-box maxInlineSize="24px" maxBlockSize="24px">
              <s-image src="./graphql-logo.png" alt="GraphQL logo" />
            </s-box>
          </s-link>
        </s-stack>
        <s-paragraph>
          <s-text>Database: </s-text>
          <s-link
            href="https://shopify.dev/docs/api/admin-extensions/2026-04-rc/target-apis/core-apis/standard-api#standardapi-propertydetail-storage"
            target="_blank"
          >
            Storage API
          </s-link>
        </s-paragraph>
      </s-section>

      {loading && (
        <s-section>
          <s-paragraph>Loading...</s-paragraph>
        </s-section>
      )}

      {!loading && !hasFaqs && (
        <s-section accessibilityLabel="Empty state section">
          <s-grid gap="base" justifyItems="center" paddingBlock="large-400">
            <s-box maxInlineSize="200px" maxBlockSize="200px">
              <s-image
                aspectRatio="1/0.5"
                src="https://cdn.shopify.com/static/images/polaris/patterns/callout.png"
                alt="Illustration of FAQ creation"
              />
            </s-box>
            <s-grid justifyItems="center" maxInlineSize="450px" gap="base">
              <s-stack alignItems="center">
                <s-heading>Start creating FAQs</s-heading>
                <s-paragraph>
                  Create frequently asked questions using the Storage API.
                </s-paragraph>
              </s-stack>
              <s-button-group>
                <s-button
                  accessibilityLabel="Add a new FAQ"
                  onclick={() => setCurrentView('add')}
                >
                  Add FAQ
                </s-button>
              </s-button-group>
            </s-grid>
          </s-grid>
        </s-section>
      )}

      {!loading && hasFaqs && (
        <s-section padding="none" accessibilityLabel="FAQs table section">
          <s-table>
            <s-table-header-row>
              <s-table-header listSlot="primary">Question</s-table-header>
              <s-table-header>Answer</s-table-header>
              <s-table-header>Actions</s-table-header>
            </s-table-header-row>
            <s-table-body>
              {faqs.map((faq) => (
                <s-table-row key={faq.id}>
                  <s-table-cell>
                    <s-text weight="bold">{faq.question}</s-text>
                  </s-table-cell>
                  <s-table-cell>
                    <s-text>{faq.answer}</s-text>
                  </s-table-cell>
                  <s-table-cell>
                    <s-button
                      onclick={() => deleteFAQ(faq.id)}
                      variant="plain"
                      tone="critical"
                    >
                      Delete
                    </s-button>
                  </s-table-cell>
                </s-table-row>
              ))}
            </s-table-body>
          </s-table>
        </s-section>
      )}
    </s-page>
  );
}
