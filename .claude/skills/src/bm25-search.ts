/**
 * BM25 Search Module for qsv Tool Discovery
 *
 * Implements BM25 text search for finding relevant qsv tools.
 * Uses field boosting to prioritize name matches over descriptions.
 */

import bm25 from "wink-bm25-text-search";
import nlp from "wink-nlp-utils";
import type { QsvSkill } from "./types.js";

/**
 * BM25 search result type [docIndex, score]
 */
type BM25Result = [number, number];

/**
 * Tool search index using BM25 algorithm
 *
 * Provides relevance-ranked search across tool definitions with
 * field boosting for better tool discovery.
 */
export class ToolSearchIndex {
  private engine: ReturnType<typeof bm25>;
  private skills: QsvSkill[] = [];
  private indexed = false;

  constructor() {
    this.engine = bm25();

    // Configure field weights:
    // - name: highest priority (exact command name matches)
    // - category: high priority (categorical discovery)
    // - description: medium priority (capability matching)
    // - examples: lower priority (supplementary context)
    this.engine.defineConfig({
      fldWeights: {
        name: 3,
        category: 2,
        description: 1,
        examples: 0.5,
      },
    });

    // Configure text preprocessing pipeline using wink-nlp-utils functions
    // These transform raw text into an array of normalized tokens
    this.engine.definePrepTasks([
      nlp.string.lowerCase,
      nlp.string.removeExtraSpaces,
      nlp.string.tokenize0,
      nlp.tokens.propagateNegations,
      nlp.tokens.stem,
    ]);
  }

  /**
   * Index all skills for search
   * Should be called after loading skills
   *
   * @param skills - Array of QsvSkill definitions to index
   */
  indexTools(skills: QsvSkill[]): void {
    this.skills = skills;

    skills.forEach((skill, idx) => {
      // Build searchable document from skill
      const doc = {
        name: skill.name.replace("qsv-", "").replace(/_/g, " "),
        category: skill.category,
        description: skill.description,
        examples:
          skill.examples?.map((e) => e.description).join(" ") || "",
      };

      this.engine.addDoc(doc, idx);
    });

    // Consolidate index for searching
    this.engine.consolidate();
    this.indexed = true;
  }

  /**
   * Search for tools matching a query
   *
   * @param query - Search query string
   * @param limit - Maximum number of results (default: 10)
   * @returns Array of matching QsvSkill objects sorted by relevance
   */
  search(query: string, limit = 10): QsvSkill[] {
    if (!this.indexed) {
      console.error("[BM25] Warning: search called before index was built");
      return [];
    }

    // Perform BM25 search
    const results = this.engine.search(query, limit) as BM25Result[];

    // Map results back to skills
    return results
      .map(([docIdx]) => this.skills[docIdx])
      .filter((skill): skill is QsvSkill => skill !== undefined);
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
    return this.skills.length;
  }

  /**
   * Reset the index (for testing or reloading)
   */
  reset(): void {
    this.engine = bm25();
    this.engine.defineConfig({
      fldWeights: {
        name: 3,
        category: 2,
        description: 1,
        examples: 0.5,
      },
    });
    this.engine.definePrepTasks([
      nlp.string.lowerCase,
      nlp.string.removeExtraSpaces,
      nlp.string.tokenize0,
      nlp.tokens.propagateNegations,
      nlp.tokens.stem,
    ]);
    this.skills = [];
    this.indexed = false;
  }
}
