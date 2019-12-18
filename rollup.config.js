import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"

export default {
    input: './target/deploy/pong.js',
    output: {
        name: 'pong',
        file: './release/pong.js',
        format: 'es',
    },
    plugins: [
        babel({
            exclude: 'node_modules/**'
        }),
        uglify
    ]
};
