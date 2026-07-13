import {useState, useEffect} from 'preact/hooks';

import AboutTemplate from '../components/AboutTemplate.jsx';
import {deleteFAQ, listFAQs} from '../models/faqs.js';

export default function HomePage() {
  const [faqs, setFaqs] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadFAQs();
  }, []);

  async function loadFAQs() {
    setLoading(true);
    try {
      setFaqs(await listFAQs());
    } catch (err) {
      console.error('Failed to load FAQs:', err);
      setFaqs([]);
    } finally {
      setLoading(false);
    }
  }

  async function handleDelete(id) {
    try {
      await deleteFAQ(id);
      setFaqs(await listFAQs());
    } catch (err) {
      console.error('Failed to delete FAQ:', err);
    }
  }

  const hasFaqs = faqs.length > 0;

  return (
    <s-page heading="FAQ Manager">
      <s-button slot="primary-action" variant="primary" href="/faq/new">
        Add FAQ
      </s-button>

      <AboutTemplate />

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
                <s-button accessibilityLabel="Add a new FAQ" href="/faq/new">
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
                    <s-link href={`/faq/${faq.id}`}>
                      <s-text type="strong">{faq.question || 'Untitled'}</s-text>
                    </s-link>
                  </s-table-cell>
                  <s-table-cell>
                    <s-text>{faq.answer}</s-text>
                  </s-table-cell>
                  <s-table-cell>
                    <s-button href={`/faq/${faq.id}`} variant="tertiary">
                      Edit
                    </s-button>
                    <s-button
                      onClick={() => handleDelete(faq.id)}
                      variant="tertiary"
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
