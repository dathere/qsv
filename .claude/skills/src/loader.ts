/**
 * QSV Skill Loader
 * Loads and manages skill definitions from JSON files
 */

import { readdir, readFile } from 'fs/promises';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import { existsSync } from 'fs';
import type { QsvSkill, SkillCategory } from './types.js';

const __dirname = dirname(fileURLToPath(import.meta.url));

export class SkillLoader {
  private skills: Map<string, QsvSkill> = new Map();
  private skillsDir: string;

  constructor(skillsDir?: string) {
    if (skillsDir) {
      this.skillsDir = skillsDir;
    } else {
      // Try multiple possible locations for the qsv directory
      // 1. When built for production: dist/loader.js -> ../qsv
      // 2. When built for tests: dist/src/loader.js -> ../../qsv
      const productionPath = join(__dirname, '../qsv');
      const testPath = join(__dirname, '../../qsv');

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
   */
  async loadAll(): Promise<Map<string, QsvSkill>> {
    const files = await readdir(this.skillsDir);

    for (const file of files) {
      if (!file.endsWith('.json')) continue;

      const skillPath = join(this.skillsDir, file);
      const content = await readFile(skillPath, 'utf-8');
      const skill: QsvSkill = JSON.parse(content);

      this.skills.set(skill.name, skill);
    }

    return this.skills;
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
      const content = await readFile(skillPath, 'utf-8');
      const skill: QsvSkill = JSON.parse(content);
      this.skills.set(skillName, skill);
      return skill;
    } catch {
      return null;
    }
  }

  /**
   * Get all loaded skills
   */
  getAll(): QsvSkill[] {
    return Array.from(this.skills.values());
  }

  /**
   * Search skills by query (matches name, description, category)
   */
  search(query: string): QsvSkill[] {
    const lowerQuery = query.toLowerCase();

    return this.getAll().filter(skill =>
      skill.name.toLowerCase().includes(lowerQuery) ||
      skill.description.toLowerCase().includes(lowerQuery) ||
      skill.category.toLowerCase().includes(lowerQuery) ||
      skill.examples.some(ex =>
        ex.description.toLowerCase().includes(lowerQuery)
      )
    );
  }

  /**
   * Get skills by category
   */
  getByCategory(category: SkillCategory): QsvSkill[] {
    return this.getAll().filter(skill => skill.category === category);
  }

  /**
   * Get all categories
   */
  getCategories(): SkillCategory[] {
    const categories = new Set<SkillCategory>();
    this.getAll().forEach(skill => {
      categories.add(skill.category as SkillCategory);
    });
    return Array.from(categories);
  }

  /**
   * Get skill statistics
   */
  getStats() {
    const skills = this.getAll();

    return {
      total: skills.length,
      byCategory: this.getCategories().reduce((acc, cat) => {
        acc[cat] = this.getByCategory(cat).length;
        return acc;
      }, {} as Record<string, number>),
      totalExamples: skills.reduce((sum, s) => sum + (s.examples?.length || 0), 0),
      totalOptions: skills.reduce((sum, s) => sum + (s.command.options?.length || 0), 0),
      totalArgs: skills.reduce((sum, s) => sum + (s.command.args?.length || 0), 0)
    };
  }

}
