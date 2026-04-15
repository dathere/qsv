/**
 * BM25 Search Module for qsv Tool Discovery
 *
 * Implements BM25 text search for finding relevant qsv tools.
 * Uses field boosting to prioritize name matches over descriptions.
 *
 * Uses wink-bm25-text-search (MIT licensed) for BM25 scoring
 * and wink-nlp-utils for tokenization and stemming.
 */

import bm25 from "wink-bm25-text-search";
import nlp from "wink-nlp-utils";
import type { QsvSkill } from "./types.js";

/**
 * Field weights for BM25 scoring
 * Higher weights mean matches in that field contribute more to relevance
 */
const FIELD_WEIGHTS = {
  name: 3,
  category: 2,
  description: 1,
  examples: 0.5,
};

/**
 * Tool search index using BM25 algorithm via wink-bm25-text-search
 *
 * Provides relevance-ranked search across tool definitions with
 * field boosting for better tool discovery.
 */
export class ToolSearchIndex {
  private engine: ReturnType<typeof bm25>;
  private skills: Map<number, QsvSkill> = new Map();
  private indexed = false;
  private indexedCount = 0;

  constructor() {
    this.engine = bm25();
    this.initializeEngine();
  }

  /**
   * Initialize the BM25 engine with proper configuration
   */
  private initializeEngine(): void {
    // Define how to prepare text for indexing/searching
    // Uses wink-nlp-utils pipeline: lowercase → tokenize → stem → propagate negations → remove stopwords
    const prepareText = (text: string): string[] => {
      const tokens = nlp.string.tokenize0(nlp.string.lowerCase(text));
      const stemmed = nlp.tokens.stem(tokens);
      const withNegation = nlp.tokens.propagateNegations(stemmed);
      return nlp.tokens.removeWords(withNegation);
    };

    // Configure the engine
    this.engine.defineConfig({ fldWeights: FIELD_WEIGHTS });
    this.engine.definePrepTasks([prepareText]);
  }

  /**
   * Index all skills for search
   * Should be called after loading skills
   *
   * @param skills - Array of QsvSkill definitions to index
   */
  indexTools(skills: QsvSkill[]): void {
    // Reset state
    this.engine = bm25();
    this.initializeEngine();
    this.skills = new Map();
    this.indexed = false;
    this.indexedCount = 0;

    // Handle empty skills array - skip consolidation to avoid BM25 error
    if (skills.length === 0) {
      return;
    }

    // Index each skill as a document with weighted fields
    for (let i = 0; i < skills.length; i++) {
      const skill = skills[i];
      this.skills.set(i, skill);

      // Prepare field text
      const nameText = skill.name.replace("qsv-", "").replace(/_/g, " ");
      const categoryText = skill.category;
      const descriptionText = skill.description;
      const examplesText =
        skill.examples?.map((e) => e.description).join(" ") || "";

      // Add document with all fields
      this.engine.addDoc(
        {
          name: nameText,
          category: categoryText,
          description: descriptionText,
          examples: examplesText,
        },
        i, // Use index as document ID
      );
    }

    // Consolidate the index (required before searching)
    this.engine.consolidate();

    this.indexed = true;
    this.indexedCount = skills.length;
  }

  /**
   * Search for tools matching a query
   *
   * @param query - Search query string
   * @param limit - Maximum number of results (default: 10)
   * @returns Array of matching QsvSkill objects sorted by relevance
   */
  search(query: string, limit = 10): QsvSkill[] {
    if (!this.indexed || this.indexedCount === 0) {
      console.error("[BM25] Warning: search called before index was built");
      return [];
    }

    if (!query || query.trim().length === 0) {
      return [];
    }

    // Perform BM25 search
    const results = this.engine.search(query);

    // Convert results back to QsvSkill objects
    // wink-bm25 returns array of [docId, score] tuples where docId is a string
    const matchedSkills: QsvSkill[] = [];
    for (let i = 0; i < Math.min(results.length, limit); i++) {
      const [docId] = results[i];
      // Convert string docId back to number for map lookup
      const numericId =
        typeof docId === "string" ? parseInt(docId, 10) : (docId as number);
      const skill = this.skills.get(numericId);
      if (skill) {
        matchedSkills.push(skill);
      }
    }

    return matchedSkills;
  }

  /**
   * Check if the index has been built
   */
  isIndexed(): boolean {
    return this.indexed;
  }

  /**
   * Get the number of indexed documents
   */
  getIndexedCount(): number {
    return this.indexedCount;
  }

  /**
   * Reset the index (for testing or reloading)
   */
  reset(): void {
    this.engine = bm25();
    this.initializeEngine();
    this.skills = new Map();
    this.indexed = false;
    this.indexedCount = 0;
  }
}
