/**
 * QSV Skill Loader
 * Loads and manages skill definitions from JSON files
 * Includes BM25 search integration for intelligent tool discovery
 */

import { readdir, readFile } from "fs/promises";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import { existsSync } from "fs";
import type { QsvSkill, SkillCategory } from "./types.js";
import { ToolSearchIndex } from "./bm25-search.js";
import { getErrorMessage } from "./utils.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

/** Type guard for parsed JSON that has the minimum required QsvSkill shape. */
function isValidSkillShape(parsed: unknown): parsed is QsvSkill {
  if (typeof parsed !== "object" || parsed === null) return false;
  const obj = parsed as Record<string, unknown>;

  // Basic required string fields
  if (typeof obj.name !== "string" || typeof obj.description !== "string") return false;
  if (typeof obj.version !== "string") return false;
  if (typeof obj.category !== "string") return false;

  // Command must be an object with at least a string `subcommand` property,
  // since code accesses `skill.command.subcommand` in multiple places.
  const command = obj.command;
  if (typeof command !== "object" || command === null) return false;
  const cmd = command as Record<string, unknown>;
  if (typeof cmd.subcommand !== "string") return false;

  // Examples, if present, should be an array (optional in practice)
  if (obj.examples !== undefined && !Array.isArray(obj.examples)) return false;

  return true;
}

export class SkillLoader {
  private skills: Map<string, QsvSkill> = new Map();
  private skillsDir: string;
  private allSkillsLoaded: boolean = false;
  private bm25Index: ToolSearchIndex | null = null;
  private loadingPromise: Promise<Map<string, QsvSkill>> | null = null;

  constructor(skillsDir?: string) {
    if (skillsDir) {
      this.skillsDir = skillsDir;
    } else {
      // Try multiple possible locations for the qsv directory
      // 1. When built for production: dist/loader.js -> ../qsv
      // 2. When built for tests: dist/src/loader.js -> ../../qsv
      const productionPath = join(__dirname, "../qsv");
      const testPath = join(__dirname, "../../qsv");

      if (existsSync(productionPath)) {
        this.skillsDir = productionPath;
      } else if (existsSync(testPath)) {
        this.skillsDir = testPath;
      } else {
        // Fallback to production path (will fail with clearer error)
        this.skillsDir = productionPath;
      }
    }
  }

  /**
   * Load all skills from the directory
   * Returns cached skills if already loaded
   * Also builds BM25 search index after loading
   */
  async loadAll(): Promise<Map<string, QsvSkill>> {
    if (this.allSkillsLoaded) {
      return this.skills;
    }

    // Prevent concurrent loads from causing duplicate BM25 indexing
    if (this.loadingPromise) {
      return this.loadingPromise;
    }

    this.loadingPromise = this.doLoadAll();
    try {
      return await this.loadingPromise;
    } finally {
      this.loadingPromise = null;
    }
  }

  /**
   * Internal implementation of loadAll (called once, guarded by loadingPromise)
   * Uses parallel I/O for faster loading of all skill files
   */
  private async doLoadAll(): Promise<Map<string, QsvSkill>> {
    const files = await readdir(this.skillsDir);
    const jsonFiles = files.filter((f) => f.endsWith(".json"));

    // Load all skill files in parallel
    const loadResults = await Promise.all(
      jsonFiles.map(async (file) => {
        const skillPath = join(this.skillsDir, file);
        const content = await readFile(skillPath, "utf-8");
        const parsed: unknown = JSON.parse(content);
        if (!isValidSkillShape(parsed)) {
          console.warn(`[Loader] Skipping invalid skill file ${file}: missing required fields`);
          return null;
        }
        return parsed;
      }),
    );

    for (const skill of loadResults) {
      if (skill) this.skills.set(skill.name, skill);
    }

    // Build BM25 index after loading all skills
    this.buildBM25Index();

    this.allSkillsLoaded = true;
    return this.skills;
  }

  /**
   * Build or rebuild the BM25 search index
   * Called automatically after loadAll()
   */
  private buildBM25Index(): void {
    if (!this.bm25Index) {
      this.bm25Index = new ToolSearchIndex();
    } else {
      this.bm25Index.reset();
    }

    const skillArray = Array.from(this.skills.values());
    this.bm25Index.indexTools(skillArray);

    console.error(
      `[Loader] Built BM25 index with ${this.bm25Index.getIndexedCount()} skills`,
    );
  }

  /**
   * Check if all skills have been loaded
   */
  isAllLoaded(): boolean {
    return this.allSkillsLoaded;
  }

  /**
   * Load a specific skill by name
   */
  async load(skillName: string): Promise<QsvSkill | null> {
    if (this.skills.has(skillName)) {
      return this.skills.get(skillName)!;
    }

    // Try loading from file
    const skillPath = join(this.skillsDir, `${skillName}.json`);
    try {
      const content = await readFile(skillPath, "utf-8");
      const parsed: unknown = JSON.parse(content);
      if (!isValidSkillShape(parsed)) {
        console.warn(`[Loader] Invalid skill file for ${skillName}: missing required fields`);
        return null;
      }
      const skill = parsed;
      this.skills.set(skillName, skill);
      return skill;
    } catch (error: unknown) {
      console.warn(`[Loader] Failed to load skill ${skillName}:`, getErrorMessage(error));
      return null;
    }
  }

  /**
   * Load multiple skills by name (batch loading with parallel I/O)
   * Returns Map of successfully loaded skills
   */
  async loadByNames(skillNames: string[]): Promise<Map<string, QsvSkill>> {
    const results = new Map<string, QsvSkill>();

    // Load all skills in parallel for better performance
    const loadPromises = skillNames.map(async (name) => {
      const skill = await this.load(name);
      return { name, skill };
    });

    const loadedResults = await Promise.all(loadPromises);

    for (const { name, skill } of loadedResults) {
      if (skill) {
        results.set(name, skill);
      }
    }

    return results;
  }

  /**
   * Get all loaded skills
   */
  getAll(): QsvSkill[] {
    return Array.from(this.skills.values());
  }

  /**
   * Search skills by query using BM25 relevance ranking
   * Falls back to substring search if BM25 index not available
   *
   * @param query - Search query string
   * @param limit - Maximum results (default: 10)
   * @returns Array of matching skills sorted by relevance
   */
  search(query: string, limit = 10): QsvSkill[] {
    // Use BM25 if index is available and built
    if (this.bm25Index?.isIndexed()) {
      return this.bm25Index.search(query, limit);
    }

    // Fallback to substring search
    return this.substringSearch(query, limit);
  }

  /**
   * Fallback substring-based search
   * Used when BM25 index is not yet built
   */
  private substringSearch(query: string, limit: number): QsvSkill[] {
    const lowerQuery = query.toLowerCase();

    return this.getAll()
      .filter(
        (skill) =>
          skill.name.toLowerCase().includes(lowerQuery) ||
          skill.description.toLowerCase().includes(lowerQuery) ||
          skill.category.toLowerCase().includes(lowerQuery) ||
          (skill.examples &&
            skill.examples.some((ex) =>
              ex.description.toLowerCase().includes(lowerQuery),
            )),
      )
      .slice(0, limit);
  }

  /**
   * Check if BM25 index is available and built
   */
  isBM25Indexed(): boolean {
    return this.bm25Index?.isIndexed() ?? false;
  }

  /**
   * Get skills by category
   */
  getByCategory(category: SkillCategory): QsvSkill[] {
    return this.getAll().filter((skill) => skill.category === category);
  }

  /**
   * Get all categories
   */
  getCategories(): SkillCategory[] {
    const categories = new Set<SkillCategory>();
    this.getAll().forEach((skill) => {
      categories.add(skill.category as SkillCategory);
    });
    return Array.from(categories);
  }

  /**
   * Get skill statistics (single-pass O(N) computation)
   */
  getStats() {
    const skills = this.getAll();
    const byCategory: Record<string, number> = {};
    let totalExamples = 0;
    let totalOptions = 0;
    let totalArgs = 0;

    for (const skill of skills) {
      byCategory[skill.category] = (byCategory[skill.category] || 0) + 1;
      totalExamples += skill.examples?.length || 0;
      totalOptions += skill.command.options?.length || 0;
      totalArgs += skill.command.args?.length || 0;
    }

    return {
      total: skills.length,
      byCategory,
      totalExamples,
      totalOptions,
      totalArgs,
    };
  }
}
