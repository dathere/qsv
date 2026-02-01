/**
 * Type declarations for wink-bm25-text-search
 */

declare module "wink-bm25-text-search" {
  /**
   * Text preprocessing function that takes a string and returns tokens
   */
  type PrepFunction = (text: string) => string[];

  interface FieldWeights {
    [field: string]: number;
  }

  interface BM25Config {
    fldWeights?: FieldWeights;
    bm25Params?: {
      k1?: number;
      b?: number;
    };
  }

  interface BM25SearchEngine {
    /**
     * Define the configuration for the BM25 search engine
     */
    defineConfig(config: BM25Config): void;

    /**
     * Define preprocessing tasks for text
     * Each function takes a string and returns an array of tokens
     */
    definePrepTasks(tasks: PrepFunction[]): void;

    /**
     * Add a document to the index
     * @param doc - Document object with field values
     * @param id - Unique identifier for the document
     */
    addDoc(doc: Record<string, string>, id: number): void;

    /**
     * Consolidate the index for searching
     * Must be called after all documents are added
     */
    consolidate(): void;

    /**
     * Search the index
     * @param query - Search query string
     * @param limit - Maximum number of results
     * @returns Array of [docId, score] tuples
     */
    search(query: string, limit?: number): Array<[number, number]>;

    /**
     * Export the BM25 model as JSON
     */
    exportJSON(): string;

    /**
     * Import a BM25 model from JSON
     */
    importJSON(json: string): void;

    /**
     * Reset the search engine
     */
    reset(): void;

    /**
     * Get the total number of documents
     */
    getTotalDocs(): number;
  }

  /**
   * Create a new BM25 search engine instance
   */
  function bm25(): BM25SearchEngine;

  export = bm25;
}
