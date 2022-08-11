module.exports = {
    // **optional** default: `{}`
    // override vscode settings
    // Notice: It only affects the settings used by Vetur.
    settings: {
        "vetur.useWorkspaceDependencies": true,
        "vetur.experimental.templateInterpolationService": true,
        "eslint.validate": [
            "javascript",
            "javascriptreact",
            "vue"
        ]
    },
    projects: [
        {
            root: './ui', // root of your vue project (should contain package.json)
            package: './package.json', // Relative to root property, don't change this.
            tsconfig: './tsconfig.json',  // Relative to root property, don't change this.
            // **optional** default: `[]`
            // Register globally Vue component glob.
            // If you set it, you can get completion by that components.
            // It is relative to root property.
            // Notice: It won't actually do it. You need to use `require.context` or `Vue.component`
            globalComponents: [
                './src/components/**/*.vue'
            ]
        }
    ]
}