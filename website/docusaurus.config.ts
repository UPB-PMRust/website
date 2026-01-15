import {themes as prismThemes} from "prism-react-renderer";
import type {Config} from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";

const config: Config = {
  title: "Microprocessor Architecture",
  tagline: "Use software to control hardware",
  favicon: "img/favicon.ico",

  // Set the production url of your site here
  url: "https://pmrust.pages.upb.ro",
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often "/<projectName>/"
  baseUrl: "/",

  // GitHub pages deployment config.
  // If you aren"t using GitHub pages, you don"t need these.
  organizationName: "PMRust", // Usually your GitHub org/user name.
  projectName: "pmrust.pages.upb.ro", // Usually your repo name.
  trailingSlash: false,

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don"t use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          // onlyIncludeVersions: ['fils_en', 'acs_cc'],
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: ({docPath, versionDocsDirPath}):string => {
            const PATH = "https://www.github.com/upb-pmrust/website/edit/main/website";
            if (docPath.startsWith ("lab/") || docPath.startsWith ("tutorial/")) {
              return PATH+"/"+docPath;
            } else {
              return PATH+"/"+versionDocsDirPath+"/"+docPath;
            }
          },
          remarkPlugins: [remarkMath],
          rehypePlugins: [rehypeKatex],
          versions: {
            "fils_en": {
              label: "FILS English",
              banner: "none",
              path: "fils_en"
            },
            "acs_cc": {
              label: "ACS CC",
              banner: "none",
              path: "acs_cc"
            }
          },
          includeCurrentVersion: false,
        },
        // blog: {
        //   showReadingTime: true,
        //   // Please change this to your repo.
        //   // Remove this to remove the "edit this page" links.
        //   editUrl:
        //     "https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/",
        // },
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
      
    ],
  ],

  plugins: [
    function webpackConfigPlugin() {
      return {
        name: 'custom-webpack-plugin',
        configureWebpack() {
          return {
            resolve: {
              symlinks: false, 
            },
          };
        },
      };
    },  
  ],

  stylesheets: [
    {
      href: "https://cdn.jsdelivr.net/npm/katex@0.13.24/dist/katex.min.css",
      type: "text/css",
      integrity:
        "sha384-odtC+0UGzzFL/6PNoE8rX/SPcQDXBJ+uRepguP4QkPCm2LBxH3FA3y+fKSiJ+AmM",
      crossorigin: "anonymous",
    },
  ],

  markdown: {
    mermaid: true,
  },

  themes: ["@docusaurus/theme-mermaid"],

  themeConfig: {
    // Replace with your project"s social card
    image: "img/docusaurus-social-card.png",
    navbar: {
      title: "Microprocessor Architecture",
      logo: {
        alt: "Microprocessor Architecture",
        src: "img/logo.svg",
      },
      items: [
        {
          type: "doc",
          position: "left",
          label: "Lecture",
          docId: "/category/lecture"
        },
        {
          type: "doc",
          position: "left",
          label: "Lab",
          docId: "/category/lab"
        },
        {
          type: "doc",
          position: "left",
          label: "Project",
          docId: "/category/project"
        },
        {
          type: "doc",
          position: "left",
          label: "Tutorials",
          docId: "/category/tutorials"
        },
        {
          type: 'docsVersionDropdown',
          docsPluginId: 'default',
          position: "right",
        },
        {
          href: "https://www.github.com/upb-pmrust/website",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      copyright: `Copyright © ${new Date().getFullYear()} Politehnica Bucharest &amp; <a href="https://wyliodrin.com" target="_blank">Wyliodrin SRL</a>, licensed under CC BY-SA 4.0.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ["linker-script", "bash", "toml"],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
