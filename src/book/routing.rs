pub fn text() -> String {
r#"
<h1 id="routing">Routing</h1>
<p>Seed includes routing: You can trigger state changes that update the address bar, and can be nagivated to/from using forward and back buttons. This works for landing-page routing as well, provided your server is configured to support. For an example of routes in use, see the <a href="https://github.com/David-OConnor/seed/tree/master/examples/homepage">homepage</a> or <a href="https://github.com/David-OConnor/seed/tree/master/examples/todomvc">todomvc</a> examples.</p>
<p>As an example, let's say our site has three pages: a home page, a guide, and a changelog, accessible by <code>http://seed-rs.org/</code>, <code>http://seed-rs.org/guide</code>, and <code>http://seed-rs.org/changelog</code> respectively. We describe the page by a <code>page</code> field in our model, which is an integer: 0 for homepage, 1 for guide, or 2 for changelog. (An enum would work as well).</p>
<p>To set up the initial routing, we pass a <code>HashMap&lt;String, Msg&gt;</code> describing the possible routings as the last parameter of <a href="https://docs.rs/seed/0.2.4/seed/fn.run.html">Seed::run</a>. We can create it using the <code>routes!</code> macro, which is for convenience: Rust doesn't include a HashMap literal syntax, and this macro automatically converts the keys to Strings, eg in the case of the &amp;strs we use in the example below:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb1-1" title="1"><span class="at">#[</span>wasm_bindgen<span class="at">]</span></a>
<a class="sourceLine" id="cb1-2" title="2"><span class="kw">pub</span> <span class="kw">fn</span> render() <span class="op">{</span></a>
<a class="sourceLine" id="cb1-3" title="3">    <span class="kw">let</span> routes = <span class="pp">routes!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb1-4" title="4">        <span class="st">&quot;guide&quot;</span> =&gt; <span class="pp">Msg::</span>RoutePage(<span class="pp">Page::</span>Guide),</a>
<a class="sourceLine" id="cb1-5" title="5">        <span class="st">&quot;changelog&quot;</span> =&gt; <span class="pp">Msg::</span>RoutePage(<span class="pp">Page::</span>Changelog),</a>
<a class="sourceLine" id="cb1-6" title="6">    <span class="op">}</span>;</a>
<a class="sourceLine" id="cb1-7" title="7"></a>
<a class="sourceLine" id="cb1-8" title="8">    <span class="pp">seed::App::</span>build(<span class="pp">Model::</span><span class="kw">default</span>(), update, view)</a>
<a class="sourceLine" id="cb1-9" title="9">        .routes(routes)</a>
<a class="sourceLine" id="cb1-10" title="10">        .finish()</a>
<a class="sourceLine" id="cb1-11" title="11">        .run();</a>
<a class="sourceLine" id="cb1-12" title="12"><span class="op">}</span></a></code></pre></div>
<p>This syntax resembles that of the <code>attrs!</code> and <code>style!</code> macros, but uses commas for separation.</p>
<p>To make landing-page routing work, configure your server so that all three of these paths point towards the app, or that any (sub)path points towards it, instead of returning an error. The <code>serve.py</code> script included in the quickstart repo and examples is set up for this. Once this is configured, intial routing on page load will work as expected: The page will load with the default state, then immediately trigger the update prescribed by the RoutePage message.</p>
<p>In order to trigger our route change through in-app naviation (eg clicking a link or pushing a button), include logic like this in the update function:</p>
<div class="sourceCode" id="cb2"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb2-1" title="1"><span class="at">#[</span>derive<span class="at">(</span><span class="bu">Clone</span><span class="at">)]</span></a>
<a class="sourceLine" id="cb2-2" title="2"><span class="kw">enum</span> Msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-3" title="3">    ChangePage(<span class="pp">seed::</span>App&lt;Msg, Model&gt;, <span class="dt">u32</span>),</a>
<a class="sourceLine" id="cb2-4" title="4">    RoutePage(<span class="dt">u32</span>),</a>
<a class="sourceLine" id="cb2-5" title="5"><span class="op">}</span></a>
<a class="sourceLine" id="cb2-6" title="6"></a>
<a class="sourceLine" id="cb2-7" title="7"><span class="kw">fn</span> update(msg: Msg, model: Model) -&gt; Model <span class="op">{</span></a>
<a class="sourceLine" id="cb2-8" title="8">    <span class="kw">match</span> msg <span class="op">{</span></a>
<a class="sourceLine" id="cb2-9" title="9">        <span class="pp">Msg::</span>ChangePage(state, page) =&gt; <span class="op">{</span></a>
<a class="sourceLine" id="cb2-10" title="10">            <span class="co">// An enum, with a to_string() method might be a more elegant way</span></a>
<a class="sourceLine" id="cb2-11" title="11">            <span class="co">// to store page state.</span></a>
<a class="sourceLine" id="cb2-12" title="12">            <span class="kw">let</span> page_name = <span class="kw">match</span> page <span class="op">{</span></a>
<a class="sourceLine" id="cb2-13" title="13">                <span class="dv">0</span> =&gt; <span class="st">&quot;&quot;</span>,</a>
<a class="sourceLine" id="cb2-14" title="14">                <span class="dv">1</span> =&gt; <span class="st">&quot;guide&quot;</span>,</a>
<a class="sourceLine" id="cb2-15" title="15">                <span class="dv">2</span> =&gt; <span class="st">&quot;changelog&quot;</span></a>
<a class="sourceLine" id="cb2-16" title="16">            <span class="op">}</span>;</a>
<a class="sourceLine" id="cb2-17" title="17">            Render(<span class="pp">seed::</span>push_route(state, page_name, <span class="pp">Msg::</span>RoutePage(page)))</a>
<a class="sourceLine" id="cb2-18" title="18">        <span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-19" title="19">        <span class="co">// This is separate, because in-app naviation needs to call push_route,</span></a>
<a class="sourceLine" id="cb2-20" title="20">        <span class="co">// but we don&#39;t want to call it from browser navigation. (eg back button)</span></a>
<a class="sourceLine" id="cb2-21" title="21">        <span class="pp">Msg::</span>RoutePage(page) =&gt; Model <span class="op">{</span>page, ..model<span class="op">}</span>,</a>
<a class="sourceLine" id="cb2-22" title="22"><span class="op">}</span></a></code></pre></div>
<p><a href="https://docs.rs/seed/0.2.4/seed/fn.push_route.html">seed::push_route</a> accepts three single parameters: a <code>seed::App</code>, a path &amp;str corresponding to what will be appended to the url, and the message that handles the state change. It sets up the routing, updates the model with the message you pass, and returns this updated model. In practice, these page_name, message combos will match your landing page routing config, but they doesn't have to. You can push whatever you'd like dynamically. These will work for page navigation and url display, but won't work for landing pages unless included in <code>.routes(routes)</code> described above.</p>
<p>When a page is loaded or browser naviation occurs (eg back button), Seed searches each of the route map keys for a matching path name (url suffix). If it finds one, it updates the model based on its associated message. If not, no action is taken. In our example, we assume the model initialized to page=0, for the homepage.</p>
<p>Notice how we keep ChangePage and RoutePage separate in our example: RoutePage performs the action associated with routing, while ChangePage updates our route history, then recursively calls RoutePage. If you were to attempt this in the same message, each browser navigation event would add a redundant route history entry, interfering with navigation. <code>seed::push_route</code> calls RoutePage from ChangePage. We call ChangePage from an in-app navigation event, like this:</p>
<div class="sourceCode" id="cb3"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb3-1" title="1"><span class="pp">h2!</span><span class="op">[</span> simple_ev(<span class="pp">Ev::</span>Click, <span class="pp">Msg::</span>ChangePage(state, <span class="dv">1</span>)), <span class="st">&quot;Guide&quot;</span> <span class="op">]</span></a></code></pre></div>
<p>Dynamic landing-page routes are not yet supported, but you may be able to populate the paths you need ahead of time in the route map:</p>
<div class="sourceCode" id="cb4"><pre class="sourceCode rust"><code class="sourceCode rust"><a class="sourceLine" id="cb4-1" title="1"><span class="kw">let</span> <span class="kw">mut</span> routes = <span class="pp">routes!</span><span class="op">{</span></a>
<a class="sourceLine" id="cb4-2" title="2">    <span class="st">&quot;guide&quot;</span> =&gt; <span class="pp">Msg::</span>RoutePage(<span class="pp">Page::</span>Guide),</a>
<a class="sourceLine" id="cb4-3" title="3">    <span class="st">&quot;changelog&quot;</span> =&gt; <span class="pp">Msg::</span>RoutePage(<span class="pp">Page::</span>Changelog),</a>
<a class="sourceLine" id="cb4-4" title="4"><span class="op">}</span>;</a>
<a class="sourceLine" id="cb4-5" title="5"></a>
<a class="sourceLine" id="cb4-6" title="6"><span class="kw">for</span> guide_page <span class="kw">in</span> <span class="dv">0</span>..<span class="dv">12</span> <span class="op">{</span></a>
<a class="sourceLine" id="cb4-7" title="7">    routes.insert(</a>
<a class="sourceLine" id="cb4-8" title="8">        <span class="st">&quot;guide/&quot;</span>.to_string() + &amp;guide_page.to_string(),</a>
<a class="sourceLine" id="cb4-9" title="9">        <span class="pp">Msg::</span>RouteGuidePage(guide_page)</a>
<a class="sourceLine" id="cb4-10" title="10">    );</a>
<a class="sourceLine" id="cb4-11" title="11"><span class="op">}</span></a></code></pre></div>
"#.into()
}