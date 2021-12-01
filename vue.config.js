const path = require('path');
const resolve = dir => {
    return path.join(__dirname, dir);
};

module.exports = {
    devServer: {
        overlay: {
            warnings: false,
            errors: false
        }
    },
    lintOnSave: false,
    chainWebpack: config => {
        config.resolve.alias.set('@', resolve('src'));
    },
    transpileDependencies: [
        'vuetify'
    ]
}