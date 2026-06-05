Upstream fixture register
========================
total fixtures: 459

By bucket
- packages/pug-filters/test/cases: 1
- packages/pug-lexer/test/cases: 118
- packages/pug-lexer/test/errors: 26
- packages/pug-linker/test/cases-src: 40
- packages/pug-linker/test/errors-src: 3
- packages/pug-linker/test/fixtures: 17
- packages/pug-linker/test/special-cases-src: 3
- packages/pug/examples: 19
- packages/pug/test/anti-cases: 22
- packages/pug/test/browser: 1
- packages/pug/test/cases: 137
- packages/pug/test/cases-es2015: 1
- packages/pug/test/dependencies: 7
- packages/pug/test/duplicate-block: 2
- packages/pug/test/eachOf/error: 4
- packages/pug/test/eachOf/passing: 2
- packages/pug/test/extends-not-top-level: 3
- packages/pug/test/fixtures: 39
- packages/pug/test/markdown-it: 2
- packages/pug/test/regression-2436: 6
- packages/pug/test/shadowed-block: 3
- packages/pug/test/temp: 3

By role
- example: 19
- case: 308
- anti-case: 55
- support: 77

Format outcomes by role
- example / idempotent: 3
- example / rewritten: 16
- case / idempotent: 66
- case / rewritten: 242
- anti-case / idempotent: 13
- anti-case / rewritten: 42
- support / idempotent: 12
- support / rewritten: 65

Structure coverage by role
- example / fully-structured: 10
- example / mixed: 9
- case / no-statements: 4
- case / fully-structured: 230
- case / mixed: 70
- case / raw-only: 4
- anti-case / fully-structured: 39
- anti-case / mixed: 6
- anti-case / raw-only: 10
- support / no-statements: 1
- support / fully-structured: 74
- support / mixed: 1
- support / raw-only: 1

Diagnostics by role
- case / warnings: 2
- anti-case / warnings: 11

Rewritten anti-cases
- packages/pug/test/anti-cases/attrs.unescaped.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/case-when.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/case-without-with.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/else-condition.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/else-without-if.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/inlining-a-mixin-after-a-tag.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/key-char-ending-badly.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/mixins-blocks-with-bodies.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/multiple-non-nested-tags-on-a-line.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/non-existant-filter.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/non-mixin-block.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/open-brace-in-attributes.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/self-closing-tag-with-body.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/tabs-and-spaces.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/unclosed-interpolated-call.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/unclosed-interpolated-tag.pug [packages/pug/test/anti-cases]
- packages/pug/test/anti-cases/unclosed-interpolation.pug [packages/pug/test/anti-cases]
- packages/pug-lexer/test/errors/attribute-invalid-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/case-with-no-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/default-with-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/else-with-condition.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/extends-no-path.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-filter-no-path-2.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-filter-no-path.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-filter-no-space.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/include-no-path.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/inconsistent-indentation.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/interpolated-call.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-class-name-1.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-class-name-2.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-class-name-3.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/invalid-id.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/malformed-each.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/malformed-extend.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/malformed-include.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/multi-line-interpolation.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/open-interpolation.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/when-with-no-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-lexer/test/errors/while-with-no-expression.pug [packages/pug-lexer/test/errors]
- packages/pug-linker/test/errors-src/child-with-tags.pug [packages/pug-linker/test/errors-src]
- packages/pug-linker/test/errors-src/extends-not-first.pug [packages/pug-linker/test/errors-src]
- packages/pug-linker/test/errors-src/unexpected-block.pug [packages/pug-linker/test/errors-src]

Warned case fixtures
- packages/pug/test/cases/code.conditionals.pug [packages/pug/test/cases] warnings=1
- packages/pug-lexer/test/cases/code.conditionals.pug [packages/pug-lexer/test/cases] warnings=1

Warned anti-cases
- packages/pug/test/anti-cases/else-condition.pug [packages/pug/test/anti-cases] warnings=1
- packages/pug/test/anti-cases/else-without-if.pug [packages/pug/test/anti-cases] warnings=1
- packages/pug-lexer/test/errors/case-with-no-expression.pug [packages/pug-lexer/test/errors] warnings=1
- packages/pug-lexer/test/errors/default-with-expression.pug [packages/pug-lexer/test/errors] warnings=2
- packages/pug-lexer/test/errors/else-with-condition.pug [packages/pug-lexer/test/errors] warnings=1
- packages/pug-lexer/test/errors/extends-no-path.pug [packages/pug-lexer/test/errors] warnings=1
- packages/pug-lexer/test/errors/include-no-path.pug [packages/pug-lexer/test/errors] warnings=1
- packages/pug-lexer/test/errors/inconsistent-indentation.pug [packages/pug-lexer/test/errors] warnings=1
- packages/pug-lexer/test/errors/when-with-no-expression.pug [packages/pug-lexer/test/errors] warnings=1
- packages/pug-lexer/test/errors/while-with-no-expression.pug [packages/pug-lexer/test/errors] warnings=1

Most opaque case fixtures
- raw 8/10 | mixed | packages/pug-lexer/test/cases/html.pug
- raw 8/16 | mixed | packages/pug-lexer/test/cases/tags.self-closing.pug
- raw 8/10 | mixed | packages/pug/test/cases/html.pug
- raw 8/16 | mixed | packages/pug/test/cases/tags.self-closing.pug
- raw 6/41 | mixed | packages/pug-lexer/test/cases/mixin.attrs.pug
- raw 6/41 | mixed | packages/pug/test/cases/mixin.attrs.pug
- raw 5/10 | mixed | packages/pug-lexer/test/cases/code.pug
- raw 5/24 | mixed | packages/pug-lexer/test/cases/mixins.pug
- raw 5/18 | mixed | packages/pug-lexer/test/cases/tag.interpolation.pug
- raw 5/10 | mixed | packages/pug/test/cases/code.pug
- raw 5/24 | mixed | packages/pug/test/cases/mixins.pug
- raw 5/18 | mixed | packages/pug/test/cases/tag.interpolation.pug
- raw 4/39 | mixed | packages/pug-lexer/test/cases/each.else.pug
- raw 4/6 | mixed | packages/pug-lexer/test/cases/escaping-class-attribute.pug
- raw 4/14 | mixed | packages/pug-lexer/test/cases/mixin.merge.pug
- raw 4/16 | mixed | packages/pug-lexer/test/cases/styles.pug
- raw 4/6 | mixed | packages/pug/test/cases/escaping-class-attribute.pug
- raw 4/14 | mixed | packages/pug/test/cases/mixin.merge.pug
- raw 4/16 | mixed | packages/pug/test/cases/styles.pug
- raw 3/10 | mixed | packages/pug-lexer/test/cases/block-code.pug
