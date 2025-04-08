<script>
  import '../app.css';
</script>

<div class="app">
  <header>
    <div class="container">
      <h1>Climate Scenarios Database</h1>
    </div>
  </header>
  
  <main>
    <div class="container">
      <slot />
    </div>
  </main>
  
  <footer>
    <div class="container">
      <p>© {new Date().getFullYear()} Climate Scenarios Database</p>
    </div>
  </footer>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  
  header {
    background-color: #1c3a5e;
    color: white;
    padding: 1rem 0;
  }
  
  main {
    flex: 1;
    padding: 2rem 0;
  }
  
  footer {
    background-color: #f5f5f5;
    padding: 1rem 0;
    text-align: center;
    color: #666;
  }
  
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
  }
  
  h1 {
    margin: 0;
    font-size: 1.8rem;
  }
</style>