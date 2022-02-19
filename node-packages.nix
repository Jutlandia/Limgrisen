# This file has been generated by node2nix 1.10.0. Do not edit!

{nodeEnv, fetchurl, fetchgit, nix-gitignore, stdenv, lib, globalBuildInputs ? []}:

let
  sources = {
    "@discordjs/builders-0.11.0" = {
      name = "_at_discordjs_slash_builders";
      packageName = "@discordjs/builders";
      version = "0.11.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/@discordjs/builders/-/builders-0.11.0.tgz";
        sha512 = "ZTB8yJdJKrKlq44dpWkNUrAtEJEq0gqpb7ASdv4vmq6/mZal5kOv312hQ56I/vxwMre+VIkoHquNUAfnTbiYtg==";
      };
    };
    "@discordjs/builders-0.6.0" = {
      name = "_at_discordjs_slash_builders";
      packageName = "@discordjs/builders";
      version = "0.6.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/@discordjs/builders/-/builders-0.6.0.tgz";
        sha512 = "mH3Gx61LKk2CD05laCI9K5wp+a3NyASHDUGx83DGJFkqJlRlSV5WMJNY6RS37A5SjqDtGMF4wVR9jzFaqShe6Q==";
      };
    };
    "@discordjs/collection-0.1.6" = {
      name = "_at_discordjs_slash_collection";
      packageName = "@discordjs/collection";
      version = "0.1.6";
      src = fetchurl {
        url = "https://registry.npmjs.org/@discordjs/collection/-/collection-0.1.6.tgz";
        sha512 = "utRNxnd9kSS2qhyivo9lMlt5qgAUasH2gb7BEOn6p0efFh24gjGomHzWKMAPn2hEReOPQZCJaRKoURwRotKucQ==";
      };
    };
    "@discordjs/collection-0.4.0" = {
      name = "_at_discordjs_slash_collection";
      packageName = "@discordjs/collection";
      version = "0.4.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/@discordjs/collection/-/collection-0.4.0.tgz";
        sha512 = "zmjq+l/rV35kE6zRrwe8BHqV78JvIh2ybJeZavBi5NySjWXqN3hmmAKg7kYMMXSeiWtSsMoZ/+MQi0DiQWy2lw==";
      };
    };
    "@discordjs/rest-0.1.0-canary.0" = {
      name = "_at_discordjs_slash_rest";
      packageName = "@discordjs/rest";
      version = "0.1.0-canary.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/@discordjs/rest/-/rest-0.1.0-canary.0.tgz";
        sha512 = "d+s//ISYVV+e0w/926wMEeO7vju+Pn11x1JM4tcmVMCHSDgpi6pnFCNAXF1TEdnDcy7xf9tq5cf2pQkb/7ySTQ==";
      };
    };
    "@sapphire/async-queue-1.2.0" = {
      name = "_at_sapphire_slash_async-queue";
      packageName = "@sapphire/async-queue";
      version = "1.2.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/@sapphire/async-queue/-/async-queue-1.2.0.tgz";
        sha512 = "O5ND5Ljpef86X5oy8zXorQ754TMjWALcPSAgPBu4+76HLtDTrNoDyzU2uGE2G4A8Wv51u0MXHzGQ0WZ4GMtpIw==";
      };
    };
    "@sapphire/snowflake-1.3.6" = {
      name = "_at_sapphire_slash_snowflake";
      packageName = "@sapphire/snowflake";
      version = "1.3.6";
      src = fetchurl {
        url = "https://registry.npmjs.org/@sapphire/snowflake/-/snowflake-1.3.6.tgz";
        sha512 = "QnzuLp+p9D7agynVub/zqlDVriDza9y3STArBhNiNBUgIX8+GL5FpQxstRfw1jDr5jkZUjcuKYAHxjIuXKdJAg==";
      };
    };
    "@sindresorhus/is-4.4.0" = {
      name = "_at_sindresorhus_slash_is";
      packageName = "@sindresorhus/is";
      version = "4.4.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/@sindresorhus/is/-/is-4.4.0.tgz";
        sha512 = "QppPM/8l3Mawvh4rn9CNEYIU9bxpXUCRMaX9yUpvBk1nMKusLKpfXGDEKExKaPhLzcn3lzil7pR6rnJ11HgeRQ==";
      };
    };
    "@types/node-17.0.18" = {
      name = "_at_types_slash_node";
      packageName = "@types/node";
      version = "17.0.18";
      src = fetchurl {
        url = "https://registry.npmjs.org/@types/node/-/node-17.0.18.tgz";
        sha512 = "eKj4f/BsN/qcculZiRSujogjvp5O/k4lOW5m35NopjZM/QwLOR075a8pJW5hD+Rtdm2DaCVPENS6KtSQnUD6BA==";
      };
    };
    "@types/node-fetch-2.6.1" = {
      name = "_at_types_slash_node-fetch";
      packageName = "@types/node-fetch";
      version = "2.6.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@types/node-fetch/-/node-fetch-2.6.1.tgz";
        sha512 = "oMqjURCaxoSIsHSr1E47QHzbmzNR5rK8McHuNb11BOM9cHcIK3Avy0s/b2JlXHoQGTYS3NsvWzV1M0iK7l0wbA==";
      };
    };
    "@types/ws-8.2.3" = {
      name = "_at_types_slash_ws";
      packageName = "@types/ws";
      version = "8.2.3";
      src = fetchurl {
        url = "https://registry.npmjs.org/@types/ws/-/ws-8.2.3.tgz";
        sha512 = "ahRJZquUYCdOZf/rCsWg88S0/+cb9wazUBHv6HZEe3XdYaBe2zr/slM8J28X07Hn88Pnm4ezo7N8/ofnOgrPVQ==";
      };
    };
    "abort-controller-3.0.0" = {
      name = "abort-controller";
      packageName = "abort-controller";
      version = "3.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/abort-controller/-/abort-controller-3.0.0.tgz";
        sha512 = "h8lQ8tacZYnR3vNQTgibj+tODHI5/+l06Au2Pcriv/Gmet0eaj4TwWH41sO9wnHDiQsEj19q0drzdWdeAHtweg==";
      };
    };
    "asynckit-0.4.0" = {
      name = "asynckit";
      packageName = "asynckit";
      version = "0.4.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/asynckit/-/asynckit-0.4.0.tgz";
        sha1 = "c79ed97f7f34cb8f2ba1bc9790bcc366474b4b79";
      };
    };
    "callsites-3.1.0" = {
      name = "callsites";
      packageName = "callsites";
      version = "3.1.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/callsites/-/callsites-3.1.0.tgz";
        sha512 = "P8BjAsXvZS+VIDUI11hHCQEv74YT67YUi5JJFNWIqL235sBmjX4+qx9Muvls5ivyNENctx46xQLQ3aTuE7ssaQ==";
      };
    };
    "combined-stream-1.0.8" = {
      name = "combined-stream";
      packageName = "combined-stream";
      version = "1.0.8";
      src = fetchurl {
        url = "https://registry.npmjs.org/combined-stream/-/combined-stream-1.0.8.tgz";
        sha512 = "FQN4MRfuJeHf7cBbBMJFXhKSDq+2kAArBlmRBvcvFE5BB1HZKXtSFASDhdlz9zOYwxh8lDdnvmMOe/+5cdoEdg==";
      };
    };
    "delayed-stream-1.0.0" = {
      name = "delayed-stream";
      packageName = "delayed-stream";
      version = "1.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/delayed-stream/-/delayed-stream-1.0.0.tgz";
        sha1 = "df3ae199acadfb7d440aaae0b29e2272b24ec619";
      };
    };
    "discord-api-types-0.18.1" = {
      name = "discord-api-types";
      packageName = "discord-api-types";
      version = "0.18.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/discord-api-types/-/discord-api-types-0.18.1.tgz";
        sha512 = "hNC38R9ZF4uaujaZQtQfm5CdQO58uhdkoHQAVvMfIL0LgOSZeW575W8H6upngQOuoxWd8tiRII3LLJm9zuQKYg==";
      };
    };
    "discord-api-types-0.22.0" = {
      name = "discord-api-types";
      packageName = "discord-api-types";
      version = "0.22.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/discord-api-types/-/discord-api-types-0.22.0.tgz";
        sha512 = "l8yD/2zRbZItUQpy7ZxBJwaLX/Bs2TGaCthRppk8Sw24LOIWg12t9JEreezPoYD0SQcC2htNNo27kYEpYW/Srg==";
      };
    };
    "discord-api-types-0.23.1" = {
      name = "discord-api-types";
      packageName = "discord-api-types";
      version = "0.23.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/discord-api-types/-/discord-api-types-0.23.1.tgz";
        sha512 = "igWmn+45mzXRWNEPU25I/pr8MwxHb767wAr51oy3VRLRcTlp5ADBbrBR0lq3SA1Rfw3MtM4TQu1xo3kxscfVdQ==";
      };
    };
    "discord-api-types-0.26.1" = {
      name = "discord-api-types";
      packageName = "discord-api-types";
      version = "0.26.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/discord-api-types/-/discord-api-types-0.26.1.tgz";
        sha512 = "T5PdMQ+Y1MEECYMV5wmyi9VEYPagEDEi4S0amgsszpWY0VB9JJ/hEvM6BgLhbdnKky4gfmZEXtEEtojN8ZKJQQ==";
      };
    };
    "discord.js-13.6.0" = {
      name = "discord.js";
      packageName = "discord.js";
      version = "13.6.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/discord.js/-/discord.js-13.6.0.tgz";
        sha512 = "tXNR8zgsEPxPBvGk3AQjJ9ljIIC6/LOPjzKwpwz8Y1Q2X66Vi3ZqFgRHYwnHKC0jC0F+l4LzxlhmOJsBZDNg9g==";
      };
    };
    "dot-prop-6.0.1" = {
      name = "dot-prop";
      packageName = "dot-prop";
      version = "6.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/dot-prop/-/dot-prop-6.0.1.tgz";
        sha512 = "tE7ztYzXHIeyvc7N+hR3oi7FIbf/NIjVP9hmAt3yMXzrQ072/fpjGLx2GxNxGxUl5V73MEqYzioOMoVhGMJ5cA==";
      };
    };
    "event-target-shim-5.0.1" = {
      name = "event-target-shim";
      packageName = "event-target-shim";
      version = "5.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/event-target-shim/-/event-target-shim-5.0.1.tgz";
        sha512 = "i/2XbnSz/uxRCU6+NdVJgKWDTM427+MqYbkQzD321DuCQJUqOuJKIA0IM2+W2xtYHdKOmZ4dR6fExsd4SXL+WQ==";
      };
    };
    "form-data-3.0.1" = {
      name = "form-data";
      packageName = "form-data";
      version = "3.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/form-data/-/form-data-3.0.1.tgz";
        sha512 = "RHkBKtLWUVwd7SqRIvCZMEvAMoGUp0XU+seQiZejj0COz3RI3hWP4sCv3gZWWLjJTd7rGwcsF5eKZGii0r/hbg==";
      };
    };
    "form-data-4.0.0" = {
      name = "form-data";
      packageName = "form-data";
      version = "4.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/form-data/-/form-data-4.0.0.tgz";
        sha512 = "ETEklSGi5t0QMZuiXoA/Q6vcnxcLQP5vdugSpuAyi6SVGi2clPPp+xgEhuMaHC+zGgn31Kd235W35f7Hykkaww==";
      };
    };
    "is-obj-2.0.0" = {
      name = "is-obj";
      packageName = "is-obj";
      version = "2.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/is-obj/-/is-obj-2.0.0.tgz";
        sha512 = "drqDG3cbczxxEJRoOXcOjtdp1J/lyp1mNn0xaznRs8+muBhgQcrnbspox5X5fOw0HnMnbfDzvnEMEtqDEJEo8w==";
      };
    };
    "lodash.isequal-4.5.0" = {
      name = "lodash.isequal";
      packageName = "lodash.isequal";
      version = "4.5.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/lodash.isequal/-/lodash.isequal-4.5.0.tgz";
        sha1 = "415c4478f2bcc30120c22ce10ed3226f7d3e18e0";
      };
    };
    "mime-db-1.51.0" = {
      name = "mime-db";
      packageName = "mime-db";
      version = "1.51.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/mime-db/-/mime-db-1.51.0.tgz";
        sha512 = "5y8A56jg7XVQx2mbv1lu49NR4dokRnhZYTtL+KGfaa27uq4pSTXkwQkFJl4pkRMyNFz/EtYDSkiiEHx3F7UN6g==";
      };
    };
    "mime-types-2.1.34" = {
      name = "mime-types";
      packageName = "mime-types";
      version = "2.1.34";
      src = fetchurl {
        url = "https://registry.npmjs.org/mime-types/-/mime-types-2.1.34.tgz";
        sha512 = "6cP692WwGIs9XXdOO4++N+7qjqv0rqxxVvJ3VHPh/Sc9mVZcQP+ZGhkKiTvWMQRr2tbHkJP/Yn7Y0npb3ZBs4A==";
      };
    };
    "node-fetch-2.6.7" = {
      name = "node-fetch";
      packageName = "node-fetch";
      version = "2.6.7";
      src = fetchurl {
        url = "https://registry.npmjs.org/node-fetch/-/node-fetch-2.6.7.tgz";
        sha512 = "ZjMPFEfVx5j+y2yF35Kzx5sF7kDzxuDj6ziH4FFbOp87zKDZNx8yExJIb05OGF4Nlt9IHFIMBkRl41VdvcNdbQ==";
      };
    };
    "ow-0.27.0" = {
      name = "ow";
      packageName = "ow";
      version = "0.27.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/ow/-/ow-0.27.0.tgz";
        sha512 = "SGnrGUbhn4VaUGdU0EJLMwZWSupPmF46hnTRII7aCLCrqixTAC5eKo8kI4/XXf1eaaI8YEVT+3FeGNJI9himAQ==";
      };
    };
    "tr46-0.0.3" = {
      name = "tr46";
      packageName = "tr46";
      version = "0.0.3";
      src = fetchurl {
        url = "https://registry.npmjs.org/tr46/-/tr46-0.0.3.tgz";
        sha1 = "8184fd347dac9cdc185992f3a6622e14b9d9ab6a";
      };
    };
    "ts-mixer-6.0.0" = {
      name = "ts-mixer";
      packageName = "ts-mixer";
      version = "6.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/ts-mixer/-/ts-mixer-6.0.0.tgz";
        sha512 = "nXIb1fvdY5CBSrDIblLn73NW0qRDk5yJ0Sk1qPBF560OdJfQp9jhl+0tzcY09OZ9U+6GpeoI9RjwoIKFIoB9MQ==";
      };
    };
    "tslib-2.3.1" = {
      name = "tslib";
      packageName = "tslib";
      version = "2.3.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/tslib/-/tslib-2.3.1.tgz";
        sha512 = "77EbyPPpMz+FRFRuAFlWMtmgUWGe9UOG2Z25NqCwiIjRhOf5iKGuzSe5P2w1laq+FkRy4p+PCuVkJSGkzTEKVw==";
      };
    };
    "type-fest-1.4.0" = {
      name = "type-fest";
      packageName = "type-fest";
      version = "1.4.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/type-fest/-/type-fest-1.4.0.tgz";
        sha512 = "yGSza74xk0UG8k+pLh5oeoYirvIiWo5t0/o3zHHAO2tRDiZcxWP7fywNlXhqb6/r6sWvwi+RsyQMWhVLe4BVuA==";
      };
    };
    "vali-date-1.0.0" = {
      name = "vali-date";
      packageName = "vali-date";
      version = "1.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/vali-date/-/vali-date-1.0.0.tgz";
        sha1 = "1b904a59609fb328ef078138420934f6b86709a6";
      };
    };
    "webidl-conversions-3.0.1" = {
      name = "webidl-conversions";
      packageName = "webidl-conversions";
      version = "3.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/webidl-conversions/-/webidl-conversions-3.0.1.tgz";
        sha1 = "24534275e2a7bc6be7bc86611cc16ae0a5654871";
      };
    };
    "whatwg-url-5.0.0" = {
      name = "whatwg-url";
      packageName = "whatwg-url";
      version = "5.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/whatwg-url/-/whatwg-url-5.0.0.tgz";
        sha1 = "966454e8765462e37644d3626f6742ce8b70965d";
      };
    };
    "ws-8.5.0" = {
      name = "ws";
      packageName = "ws";
      version = "8.5.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/ws/-/ws-8.5.0.tgz";
        sha512 = "BWX0SWVgLPzYwF8lTzEy1egjhS4S4OEAHfsO8o65WOVsrnSRGaSiUaa9e0ggGlkMTtBlmOpEXiie9RUcBO86qg==";
      };
    };
    "zod-3.11.6" = {
      name = "zod";
      packageName = "zod";
      version = "3.11.6";
      src = fetchurl {
        url = "https://registry.npmjs.org/zod/-/zod-3.11.6.tgz";
        sha512 = "daZ80A81I3/9lIydI44motWe6n59kRBfNzTuS2bfzVh1nAXi667TOTWWtatxyG+fwgNUiagSj/CWZwRRbevJIg==";
      };
    };
  };
  args = {
    name = "kimbot2";
    packageName = "kimbot2";
    version = "0.0.1";
    src = ./.;
    dependencies = [
      (sources."@discordjs/builders-0.6.0" // {
        dependencies = [
          sources."discord-api-types-0.22.0"
        ];
      })
      sources."@discordjs/collection-0.1.6"
      (sources."@discordjs/rest-0.1.0-canary.0" // {
        dependencies = [
          sources."discord-api-types-0.18.1"
        ];
      })
      sources."@sapphire/async-queue-1.2.0"
      sources."@sapphire/snowflake-1.3.6"
      sources."@sindresorhus/is-4.4.0"
      sources."@types/node-17.0.18"
      (sources."@types/node-fetch-2.6.1" // {
        dependencies = [
          sources."form-data-3.0.1"
        ];
      })
      sources."@types/ws-8.2.3"
      sources."abort-controller-3.0.0"
      sources."asynckit-0.4.0"
      sources."callsites-3.1.0"
      sources."combined-stream-1.0.8"
      sources."delayed-stream-1.0.0"
      sources."discord-api-types-0.23.1"
      (sources."discord.js-13.6.0" // {
        dependencies = [
          sources."@discordjs/builders-0.11.0"
          sources."@discordjs/collection-0.4.0"
          sources."discord-api-types-0.26.1"
        ];
      })
      sources."dot-prop-6.0.1"
      sources."event-target-shim-5.0.1"
      sources."form-data-4.0.0"
      sources."is-obj-2.0.0"
      sources."lodash.isequal-4.5.0"
      sources."mime-db-1.51.0"
      sources."mime-types-2.1.34"
      (sources."node-fetch-2.6.7" // {
        dependencies = [
          sources."tr46-0.0.3"
          sources."webidl-conversions-3.0.1"
          sources."whatwg-url-5.0.0"
        ];
      })
      (sources."ow-0.27.0" // {
        dependencies = [
          sources."type-fest-1.4.0"
        ];
      })
      sources."ts-mixer-6.0.0"
      sources."tslib-2.3.1"
      sources."vali-date-1.0.0"
      sources."ws-8.5.0"
      sources."zod-3.11.6"
    ];
    buildInputs = globalBuildInputs;
    meta = {
      description = "A bot for managing CTFs";
      license = "AGPL-3.0-or-later";
    };
    production = true;
    bypassCache = true;
    reconstructLock = false;
  };
in
{
  args = args;
  sources = sources;
  tarball = nodeEnv.buildNodeSourceDist args;
  package = nodeEnv.buildNodePackage args;
  shell = nodeEnv.buildNodeShell args;
  nodeDependencies = nodeEnv.buildNodeDependencies (lib.overrideExisting args {
    src = stdenv.mkDerivation {
      name = args.name + "-package-json";
      src = nix-gitignore.gitignoreSourcePure [
        "*"
        "!package.json"
        "!package-lock.json"
      ] args.src;
      dontBuild = true;
      installPhase = "mkdir -p $out; cp -r ./* $out;";
    };
  });
}
