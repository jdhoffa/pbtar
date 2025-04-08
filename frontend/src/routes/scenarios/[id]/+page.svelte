<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { scenariosApi } from '$lib/api';
  import type { ScenarioDetail } from '$lib/types';
  
  let scenario: ScenarioDetail | null = null;
  let loading = true;
  let error: string | null = null;
  
  // Get scenario ID from route params
  $: scenarioId = Number($page.params.id);
  
  // Helper function to get emoji based on scenario type
  function getScenarioEmoji(type: string): string {
    return type.toLowerCase() === 'normative' ? '🎯' : '🔍';
  }
  
  // Format date for display
  function formatDate(dateString?: string): string {
    if (!dateString) return 'N/A';
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { 
      year: 'numeric', 
      month: 'short', 
      day: 'numeric' 
    });
  }
  
  // Load scenario data
  async function loadScenarioDetail() {
    loading = true;
    error = null;
    
    try {
      scenario = await scenariosApi.getScenarioById(scenarioId);
    } catch (err) {
      console.error('Error loading scenario:', err);
      error = 'Failed to load scenario details. Please try again later.';
      scenario = null;
    } finally {
      loading = false;
    }
  }
  
  // Initialize component
  onMount(() => {
    if (scenarioId) {
      loadScenarioDetail();
    }
  });
</script>

<svelte:head>
  {#if scenario}
    <title>{scenario.title} - Climate Scenarios Database</title>
  {:else}
    <title>Scenario Details - Climate Scenarios Database</title>
  {/if}
</svelte:head>

<div class="scenario-detail-page">
  <div class="back-link">
    <a href="/">&larr; Back to All Scenarios</a>
  </div>
  
  {#if loading}
    <div class="loading">Loading scenario details...</div>
  {:else if error}
    <div class="error card">
      <p>{error}</p>
      <button class="btn" on:click={loadScenarioDetail}>Try Again</button>
    </div>
  {:else if scenario}
    <div class="scenario-header">
      <div class="scenario-type">
        <span class="emoji">{getScenarioEmoji(scenario.type_name)}</span>
        <span class="badge badge--{scenario.type_name.toLowerCase()}">{scenario.type_name}</span>
        {#if scenario.temperature_target}
          <span class="badge badge--temperature">{scenario.temperature_target}</span>
        {/if}
      </div>
      <h1>{scenario.title}</h1>
    </div>
    
    <div class="scenario-content card">
      <div class="scenario-meta">
        {#if scenario.publisher}
          <div class="meta-item">
            <span class="meta-label">Publisher:</span>
            <span class="meta-value">{scenario.publisher.name}</span>
          </div>
        {/if}
        {#if scenario.published_date}
          <div class="meta-item">
            <span class="meta-label">Published:</span>
            <span class="meta-value">{formatDate(scenario.published_date)}</span>
          </div>
        {/if}
        {#if scenario.target_year}
          <div class="meta-item">
            <span class="meta-label">Target Year:</span>
            <span class="meta-value">{scenario.target_year}</span>
          </div>
        {/if}
      </div>
      
      <div class="scenario-description">
        <h3>Description</h3>
        <p>{scenario.description || 'No description provided.'}</p>
      </div>
      
      <div class="scenario-sections">
        <div class="section">
          <h3>Regions</h3>
          {#if scenario.regions && scenario.regions.length > 0}
            <div class="tags">
              {#each scenario.regions as region}
                <span class="tag">{region.name}</span>
              {/each}
            </div>
          {:else}
            <p>No regions specified.</p>
          {/if}
        </div>
        
        <div class="section">
          <h3>Stakeholders</h3>
          {#if scenario.stakeholders && scenario.stakeholders.length > 0}
            <div class="tags">
              {#each scenario.stakeholders as stakeholder}
                <span class="tag">{stakeholder.name}</span>
              {/each}
            </div>
          {:else}
            <p>No stakeholders specified.</p>
          {/if}
        </div>
        
        <div class="section">
          <h3>Sectors</h3>
          {#if scenario.sectors && scenario.sectors.length > 0}
            <div class="tags">
              {#each scenario.sectors as sector}
                <span class="tag">{sector.name}</span>
              {/each}
            </div>
          {:else}
            <p>No sectors specified.</p>
          {/if}
        </div>
      </div>
    </div>
  {:else}
    <div class="error card">
      <p>Scenario not found.</p>
      <a href="/" class="btn">Back to Home</a>
    </div>
  {/if}
</div>

<style>
  .scenario-detail-page {
    margin-bottom: 2rem;
  }
  
  .back-link {
    margin-bottom: 1.5rem;
  }
  
  .scenario-header {
    margin-bottom: 1.5rem;
  }
  
  .scenario-type {
    margin-bottom: 0.5rem;
  }
  
  .scenario-meta {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }
  
  .meta-item {
    display: flex;
    flex-direction: column;
  }
  
  .meta-label {
    font-weight: bold;
    color: var(--color-text-light);
    font-size: 0.9rem;
  }
  
  .meta-value {
    font-size: 1.1rem;
  }
  
  .scenario-description {
    margin-bottom: 2rem;
  }
  
  .scenario-sections {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
  }
  
  .section h3 {
    margin-bottom: 1rem;
    color: var(--color-primary);
    border-bottom: 1px solid var(--color-border);
    padding-bottom: 0.5rem;
  }
  
  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  
  .error {
    color: #d32f2f;
    text-align: center;
    padding: 2rem;
  }
</style>