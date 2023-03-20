<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    <div
        class="w-full lg:w-5/12 md:w-6/12 m-auto leading-6 md:leading-8 indent-6"
    >
        <h1 class="text-4xl mt-24 mb-8">About</h1>

        <h2 class="text-xl my-4">What</h2>
        <section class="text-body hyphens">
            <p>
                This is the web version of the PhD thesis of <a
                    href="https://pierredepaz.net"
                    target="_blank">Pierre Depaz</a
                >. It is also an inquiry into publishing research work online.
                As such, it integrates modular systems of epistemic software (<a
                    href="http://zotero.org"
                    target="_blank"
                    rel="noopener noreferrer">Zotero</a
                >,
                <a
                    href="http://hypothes.is"
                    target="_blank"
                    rel="noopener noreferrer">Hypothes.is</a
                >), hyperlinking of labelled components (headings, figures,
                listings, citations), document layout (user-accessible line
                height, line length, font family, font size, as well as
                responsive design and dark mode), asset delivery (optimizing
                requests for size and speed, manifesting as a
                <a
                    href="https://developer.mozilla.org/en-US/docs/Web/Progressive_web_apps"
                    target="_blank"
                    rel="noopener noreferrer">progressive web app</a
                > ) and publishing pipeline (parsing, processing and templating).
            </p>
            <p>It is also a practical attempt at translating the ideas of this thesis in a different computational medium. As a different material, emphasis is being put on the referencing graph of different components (semantic space) and on more traditional document layout principles. Patterns apply, but literature and mathematics less, due to a more evasive assessment of the executed text and its function. That is, it tries to be habitable.</p>
        </section>

        <h2 class="text-xl my-4">Why</h2>
        <section class="text-body hyphens">
            <p>
                While one of the requirement to fulfill this degree is to hand
                in a specifically formatted PDF document, I quickly realized
                that people tend not to casually read PDFs, and that tendency
                decreases as the number of pages increases. I tend to think that
                research creation is, in an era of information overload, more
                and more dependent on the means of diffusion; so I started to
                wonder how could a PDF, a static document, be turned into a
                website, a dynamic document.
            </p>
            <p>
                One of the conclusions of <a href="/introduction">this thesis</a> is that a form of beauty of
                source code is the extent to which it is shaped in a way that it
                is true to its material. This quickly led to the question of
                media-specificity. This quickly led to the question of
                media-specificity: of the relation between the content of the
                document, its medium of representation, and its representation
                through this medium.
            </p>
        </section>

        <h2 class="text-xl my-4">How</h2>
        <section class="text-body hyphens">
            <p>
                The original document is written in <a href="https://www.latex-project.org/" target="_blank" rel="noopener noreferrer">LaTeX</a>. The parsing is done in <a href="https://doc.rust-lang.org/book/" target="_blank" rel="noopener noreferrer">Rust</a>, then a series of bash commands in a <a href="https://www.gnu.org/software/make/" target="_blank" rel="noopener noreferrer">Makefile</a> copy the <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON" target="_blank" rel="noopener noreferrer">JSON</a> output and required assets to a Svelte application; the <a href="https://svelte.dev/" target="_blank" rel="noopener noreferrer">Svelte</a> app is built and deployed to a virtual machine rented from Digital Ocean.
            </p>
            <p>It's all on <a href="https://gitlab.com/periode/webthesis" target="_blank" rel="noopener noreferrer">this repo</a>.</p>
        </section>
    </div>
</div>
