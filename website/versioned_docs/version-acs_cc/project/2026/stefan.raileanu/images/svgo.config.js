module.exports = {
  multipass: true,
  plugins: [
    {
      name: 'preset-default',
      params: {
        overrides: {
          removeViewBox: false,
          removeTitle: false,
          removeDesc: false,
          cleanupIds: false,
          mergePaths: false,
          collapseGroups: false,
          moveElemsAttrsToGroup: false,
          moveGroupAttrsToElems: false,
          convertShapeToPath: false,
        },
      },
    },
  ],
};
