<script lang="ts">
  import { onMount } from 'svelte';
  import Select from 'svelte-select';
  import { scenariosApi } from '$lib/api';
  import type { ScenarioListItem, FilterOptions, ScenarioFilters } from '$lib/types';

  // State variables
  let scenarios: ScenarioListItem[] = [];
  let filterOptions: FilterOptions | null = null;
  let loading = true;
  let error: string | null = null;
  
  // Filter state
  let filters: ScenarioFilters = {};
  
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

  // Load scenarios with current filters
  async function loadScenarios() {
    loading = true;
    error = null;
    
    try {
      scenarios = await scenariosApi.getScenarios(filters);
    } catch (err) {
      console.error('Error loading scenarios:', err);
      error = 'Failed to load scenarios. Please try again later.';
      scenarios = [];
    } finally {
      loading = false;
    }
  }
  
  // Load filter options
  async function loadFilterOptions() {
    try {
      filterOptions = await scenariosApi.getFilterOptions();
    } catch (err) {
      console.error('Error loading filter options:', err);
      filterOptions = null;
    }
  }
  
  // Apply filters when they change
  function applyFilters() {
    loadScenarios();
  }
  
  // Reset filters to default
  function resetFilters() {
    filters = {};
    loadScenarios();
  }
  
  // Initialize component
  onMount(() => {
    loadFilterOptions();
    loadScenarios();
  });
</script>

<svelte:head>
  <title>Climate Scenarios Database</title>
</svelte:head>

<div class="page-home">
  <section class="filter-section">
    <div class="card filter-panel">
      <h2>Filter Scenarios</h2>
      
      {#if filterOptions}
        <div class="filter-grid">
          <div class="form-group">
            <label for="publisher">Publisher</label>
            <Select
              id="publisher"
              items={filterOptions.publishers}
              labelField="name"
              valueField="id"
              placeholder="All Publishers"
              on:change={({ detail }) => { 
                filters.publisher_id = detail ? detail.id : undefined;
              }}
            />
          </div>
          
          <div class="form-group">
            <label for="region">Region</label>
            <Select
              id="region"
              items={filterOptions.regions}
              labelField="name"
              valueField="id"
              placeholder="All Regions"
              on:change={({ detail }) => { 
                filters.region_id = detail ? detail.id : undefined;
              }}
            />
          </div>
          
          <div class="form-group">
            <label for="stakeholder">Stakeholder</label>
            <Select
              id="stakeholder"
              items={filterOptions.stakeholders}
              labelField="name"
              valueField="id"
              placeholder="All Stakeholders"
              on:change={({ detail }) => { 
                filters.stakeholder_id = detail ? detail.id : undefined;
              }}
            />
          </div>
          
          <div class="form-group">
            <label for="sector">Sector</label>
            <Select
              id="sector"
              items={filterOptions.sectors}
              labelField="name"
              valueField="id"
              placeholder="All Sectors"
              on:change={({ detail }) => { 
                filters.sector_id = detail ? detail.id : undefined;
              }}
            />
          </div>
          
          <div class="form-group">
            <label for="type">Scenario Type</label>
            <Select
              id="type"
              items={filterOptions.types.map(t => ({ value: t, label: t }))}
              placeholder="All Types"
              on:change={({ detail }) => { 
                filters.type_name = detail ? detail.value : undefined;
              }}
            />
          </div>
          
          <div class="form-group">
            <label for="temperature">Temperature Target</label>
            <Select
              id="temperature"
              items={filterOptions.temperature_targets.map(t => ({ value: t, label: t }))}
              placeholder="All Targets"
              on:change={({ detail }) => { 
                filters.temperature_target = detail ? detail.value : undefined;
              }}
            />
          </div>
          
          <div class="form-group">
            <label for="yearFrom">Year From</label>
            <input 
              type="number" 
              class="form-control" 
              id="yearFrom" 
              placeholder="Any"
              bind:value={filters.year_from}
            >
          </div>
          
          <div class="form-group">
            <label for="yearTo">Year To</label>
            <input 
              type="number" 
              class="form-control" 
              id="yearTo" 
              placeholder="Any"
              bind:value={filters.year_to}
            >
          </div>
        </div>
        
        <div class="filter-actions">
          <button class="btn" on:click={applyFilters}>Apply Filters</button>
          <button class="btn btn-secondary" on:click={resetFilters}>Reset</button>
        </div>
      {:else}
        <div class="loading">Loading filter options...</div>
      {/if}
    </div>
  </section>
  
  <section class="scenarios-section">
    <h2>Climate Scenarios</h2>
    
    {#if loading}
      <div class="loading">Loading scenarios...</div>
    {:else if error}
      <div class="error card">
        <p>{error}</p>
        <button class="btn" on:click={loadScenarios}>Try Again</button>
      </div>
    {:else if scenarios.length === 0}
      <div class="empty-state card">
        <p>No scenarios found that match your filters. Try adjusting your search criteria.</p>
        <button class="btn" on:click={resetFilters}>Reset Filters</button>
      </div>
    {:else}
      <div class="scenarios-list">
        {#each scenarios as scenario (scenario.id)}
          <div class="scenario-card">
            <div class="scenario-card__header">
              <div class="scenario-card__header-content">
                <div class="scenario-card__type">
                  <span class="emoji">{getScenarioEmoji(scenario.type_name)}</span>
                  <span class="badge badge--{scenario.type_name.toLowerCase()}">{scenario.type_name}</span>
                  {#if scenario.temperature_target}
                    <span class="badge badge--temperature">{scenario.temperature_target}</span>
                  {/if}
                </div>
                <h3 class="scenario-card__title">
                  {scenario.title}
                </h3>
              </div>
            </div>
            <div class="scenario-card__content">
              <div class="scenario-card__body">
                <p>{scenario.description || 'No description provided.'}</p>
                <a href="/scenarios/{scenario.id}" class="btn">View Details</a>
              </div>
              <div class="scenario-card__metadata">
                {#if scenario.publisher}
                  <div><strong>Publisher:</strong> {scenario.publisher}</div>
                {/if}
                {#if scenario.published_date}
                  <div><strong>Published:</strong> {formatDate(scenario.published_date)}</div>
                {/if}
                {#if scenario.target_year}
                  <div><strong>Target Year:</strong> {scenario.target_year}</div>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </section>
</div>

<style>
  .page-home {
    margin-bottom: 2rem;
  }
  
  .filter-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }
  
  .filter-actions {
    margin-top: 1.5rem;
    display: flex;
    gap: 1rem;
  }
  
  .btn-secondary {
    background-color: #9e9e9e;
  }
  
  .scenarios-section {
    margin-top: 2rem;
  }
  
  .error {
    color: #d32f2f;
    text-align: center;
    padding: 2rem;
  }
  
  .empty-state {
    text-align: center;
    padding: 2rem;
  }
  
  .scenario-card__description {
    flex-grow: 1;
    margin-bottom: 1rem;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .badge--temperature {
    background-color: #fff3e0;
    color: #e65100;
  }
</style>
