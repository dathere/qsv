/**
 * MCP Resource Provider for QSV Test Examples
 *
 * Exposes 1,279 test examples as browsable MCP resources
 */

import type { SkillLoader } from './loader.js';
import type { TestExamples, McpResource, McpResourceContent } from './types.js';

export class ExampleResourceProvider {
  private loader: SkillLoader;
  private cache: Map<string, TestExamples> = new Map();

  constructor(loader: SkillLoader) {
    this.loader = loader;
  }

  /**
   * List all available example resources
   * Supports filtering by query string
   */
  async listResources(query?: string): Promise<McpResource[]> {
    const resources: McpResource[] = [];

    // Get all skills
    const skills = await this.loader.loadAll();

    for (const skill of skills.values()) {
      // Skip skills without test examples
      if (!skill.examples_ref) {
        continue;
      }

      try {
        // Load test examples for this skill
        const examples = await this.loadExamplesForSkill(skill.name);

        if (!examples || examples.examples.length === 0) {
          continue;
        }

        // Add each example as a resource
        for (const example of examples.examples) {
          const uri = this.buildUri(skill.name, example.name);
          const name = `${skill.name}: ${example.description}`;

          // Apply query filter if provided
          if (query) {
            const searchText = `${name} ${example.description} ${example.tags.join(' ')}`.toLowerCase();
            if (!searchText.includes(query.toLowerCase())) {
              continue;
            }
          }

          resources.push({
            uri,
            name,
            description: `${example.description} [${example.tags.join(', ')}]`,
            mimeType: 'application/json',
          });
        }
      } catch (error) {
        // Skip skills with errors loading examples
        console.warn(`Failed to load examples for ${skill.name}:`, error instanceof Error ? error.message : String(error));
        continue;
      }
    }

    return resources;
  }

  /**
   * Get a specific example resource by URI
   */
  async getResource(uri: string): Promise<McpResourceContent | null> {
    try {
      const { skillName, exampleName } = this.parseUri(uri);

      // Load examples for this skill
      const examples = await this.loadExamplesForSkill(skillName);

      if (!examples) {
        return null;
      }

      // Find the specific example
      const example = examples.examples.find(ex => ex.name === exampleName);

      if (!example) {
        return null;
      }

      // Format example as JSON with helpful structure
      const content = {
        name: example.name,
        description: example.description,
        tags: example.tags,
        command: example.command,
        args: example.args,
        options: example.options,
        input: example.input ? {
          filename: example.input.filename,
          data: example.input.data,
          preview: example.input.data && Array.isArray(example.input.data)
            ? example.input.data.slice(0, 10).map(row => row.join(',')).join('\n')
            : undefined,
        } : undefined,
        expected: example.expected ? {
          data: example.expected.data,
          preview: example.expected.data && Array.isArray(example.expected.data)
            ? example.expected.data.slice(0, 10).map(row => row.join(',')).join('\n')
            : undefined,
        } : undefined,
        usage: {
          description: 'How to use this example',
          steps: [
            '1. Create input CSV file with the provided data',
            '2. Run the command shown above',
            '3. Compare output with expected results',
          ],
        },
      };

      return {
        uri,
        mimeType: 'application/json',
        text: JSON.stringify(content, null, 2),
      };
    } catch (error) {
      return null;
    }
  }

  /**
   * List resources for a specific skill
   */
  async listResourcesForSkill(skillName: string): Promise<McpResource[]> {
    const examples = await this.loadExamplesForSkill(skillName);

    if (!examples) {
      return [];
    }

    return examples.examples.map(example => ({
      uri: this.buildUri(skillName, example.name),
      name: `${skillName}: ${example.description}`,
      description: `${example.description} [${example.tags.join(', ')}]`,
      mimeType: 'application/json',
    }));
  }

  /**
   * List resources by tag
   */
  async listResourcesByTag(tag: string): Promise<McpResource[]> {
    const allResources = await this.listResources();

    return allResources.filter(resource => {
      const descMatch = resource.description?.toLowerCase().includes(`[${tag.toLowerCase()}`);
      return descMatch;
    });
  }

  /**
   * Get statistics about available examples
   */
  async getStatistics(): Promise<{
    totalSkills: number;
    skillsWithExamples: number;
    totalExamples: number;
    examplesBySkill: Record<string, number>;
  }> {
    const skills = await this.loader.loadAll();
    let skillsWithExamples = 0;
    let totalExamples = 0;
    const examplesBySkill: Record<string, number> = {};

    for (const skill of skills.values()) {
      if (!skill.examples_ref) {
        continue;
      }

      try {
        const examples = await this.loadExamplesForSkill(skill.name);

        if (examples && examples.examples.length > 0) {
          skillsWithExamples++;
          totalExamples += examples.examples.length;
          examplesBySkill[skill.name] = examples.examples.length;
        }
      } catch (error) {
        // Skip on error
        console.warn(`Failed to load examples for ${skill.name} during statistics:`, error instanceof Error ? error.message : String(error));
      }
    }

    return {
      totalSkills: skills.size,
      skillsWithExamples,
      totalExamples,
      examplesBySkill,
    };
  }

  /**
   * Load test examples for a skill (with caching)
   */
  private async loadExamplesForSkill(skillName: string): Promise<TestExamples | null> {
    // Check cache first
    if (this.cache.has(skillName)) {
      return this.cache.get(skillName)!;
    }

    // Load from disk
    const examples = await this.loader.loadTestExamples(skillName);

    if (examples) {
      this.cache.set(skillName, examples);
    }

    return examples;
  }

  /**
   * Build resource URI for an example
   */
  private buildUri(skillName: string, exampleName: string): string {
    // Remove qsv- prefix for cleaner URIs
    const command = skillName.replace('qsv-', '');
    return `qsv://examples/${command}/${exampleName}`;
  }

  /**
   * Parse resource URI to extract skill and example names
   */
  private parseUri(uri: string): { skillName: string; exampleName: string } {
    // Expected format: qsv://examples/{command}/{example_name}
    const match = uri.match(/^qsv:\/\/examples\/([^/]+)\/(.+)$/);

    if (!match) {
      throw new Error(`Invalid resource URI: ${uri}`);
    }

    const command = match[1];
    const exampleName = match[2];

    // Validate that captured groups are not empty
    if (!command || !exampleName || command.trim() === '' || exampleName.trim() === '') {
      throw new Error(`Invalid resource URI: command and example name must not be empty in ${uri}`);
    }

    // Add qsv- prefix back to get skill name
    const skillName = `qsv-${command}`;

    return { skillName, exampleName };
  }
}
