use crate::ticker_data::ticker_data::{Price, TickerData, Value};
use crate::ticker_data_scraper::{ticker_data_parser, ticker_data_parser_if::TickerDataParserIf};

const HTML_DATA: &str = r##"
<!DOCTYPE html>
<html lang="en" class=" dark">
<head>
<title>AACT - Ares Acquisition Corporation II Stock Price and Quote</title>
<meta charset="UTF-8"><meta name="viewport" content="width=1024"><meta name="description" content="AACT - Ares Acquisition Corporation II - Stock screener for investors and traders, financial visualizations.">

            <link rel="preload" href="/fonts/lato-v17-latin-ext_latin-regular.woff2" as="font" crossorigin>
            <link rel="preload" href="/fonts/lato-v17-latin-ext_latin-700.woff2" as="font" crossorigin>
            <link rel="preload" href="/fonts/lato-v17-latin-ext_latin-900.woff2" as="font" crossorigin>
            <link rel="preload" href="/fonts/inter-latin.woff2" as="font" crossorigin>
        
            <script>
                window.notificationsArray = [];
                window.renderScriptNotLoaded = function () {};
                window.handleScriptNotLoaded = function (element) {
                    window.notificationsArray.push(element);
                    window.sentryDisabled = true;
                    window.handleScriptNotLoaded = function () {};
                };
            </script>
        <link rel="stylesheet" href="/assets/dist-legacy/redesign.6273f5a6.css" type="text/css" onerror="window.handleScriptNotLoaded(this)">
<link rel="stylesheet" href="/assets/dist-legacy/main.a7d1d792.css" type="text/css" onerror="window.handleScriptNotLoaded(this)">
<link rel="icon" type="image/png" href="/favicon_2x.png" sizes="32x32">
<link rel="icon" type="image/png" href="/favicon.png" sizes="16x16">
<link rel="canonical" href="/quote.ashx?t=AACT">
  <script type="text/javascript" async=true>
  (function() {
    var host = 'finviz.com';
    var element = document.createElement('script');
    var firstScript = document.getElementsByTagName('script')[0];
    var url = 'https://cmp.inmobi.com'
      .concat('/choice/', 'c2W8esUZ6Q8oA', '/', host, '/choice.js?tag_version=V3');
    var uspTries = 0;
    var uspTriesLimit = 3;
    element.async = true;
    element.type = 'text/javascript';
    element.src = url;

    firstScript.parentNode.insertBefore(element, firstScript);

    function makeStub() {
      var TCF_LOCATOR_NAME = '__tcfapiLocator';
      var queue = [];
      var win = window;
      var cmpFrame;

      function addFrame() {
        var doc = win.document;
        var otherCMP = !!(win.frames[TCF_LOCATOR_NAME]);

        if (!otherCMP) {
          if (doc.body) {
            var iframe = doc.createElement('iframe');

            iframe.style.cssText = 'display:none';
            iframe.name = TCF_LOCATOR_NAME;
            doc.body.appendChild(iframe);
          } else {
            setTimeout(addFrame, 5);
          }
        }
        return !otherCMP;
      }

      function tcfAPIHandler() {
        var gdprApplies;
        var args = arguments;

        if (!args.length) {
          return queue;
        } else if (args[0] === 'setGdprApplies') {
          if (
            args.length > 3 &&
            args[2] === 2 &&
            typeof args[3] === 'boolean'
          ) {
            gdprApplies = args[3];
            if (typeof args[2] === 'function') {
              args[2]('set', true);
            }
          }
        } else if (args[0] === 'ping') {
          var retr = {
            gdprApplies: gdprApplies,
            cmpLoaded: false,
            cmpStatus: 'stub'
          };

          if (typeof args[2] === 'function') {
            args[2](retr);
          }
        } else {
          if(args[0] === 'init' && typeof args[3] === 'object') {
            args[3] = Object.assign(args[3], { tag_version: 'V3' });
          }
          queue.push(args);
        }
      }

      function postMessageEventHandler(event) {
        var msgIsString = typeof event.data === 'string';
        var json = {};

        try {
          if (msgIsString) {
            json = JSON.parse(event.data);
          } else {
            json = event.data;
          }
        } catch (ignore) {}

        var payload = json.__tcfapiCall;

        if (payload) {
          window.__tcfapi(
            payload.command,
            payload.version,
            function(retValue, success) {
              var returnMsg = {
                __tcfapiReturn: {
                  returnValue: retValue,
                  success: success,
                  callId: payload.callId
                }
              };
              if (msgIsString) {
                returnMsg = JSON.stringify(returnMsg);
              }
              if (event && event.source && event.source.postMessage) {
                event.source.postMessage(returnMsg, '*');
              }
            },
            payload.parameter
          );
        }
      }

      while (win) {
        try {
          if (win.frames[TCF_LOCATOR_NAME]) {
            cmpFrame = win;
            break;
          }
        } catch (ignore) {}

        if (win === window.top) {
          break;
        }
        win = win.parent;
      }
      if (!cmpFrame) {
        addFrame();
        win.__tcfapi = tcfAPIHandler;
        win.addEventListener('message', postMessageEventHandler, false);
      }
    };

    makeStub();

    function makeGppStub() {
      const CMP_ID = 10;
      const SUPPORTED_APIS = [
        '2:tcfeuv2',
        '6:uspv1',
        '7:usnatv1',
        '8:usca',
        '9:usvav1',
        '10:uscov1',
        '11:usutv1',
        '12:usctv1'
      ];

      window.__gpp_addFrame = function (n) {
        if (!window.frames[n]) {
          if (document.body) {
            var i = document.createElement("iframe");
            i.style.cssText = "display:none";
            i.name = n;
            document.body.appendChild(i);
          } else {
            window.setTimeout(window.__gpp_addFrame, 10, n);
          }
        }
      };
      window.__gpp_stub = function () {
        var b = arguments;
        __gpp.queue = __gpp.queue || [];
        __gpp.events = __gpp.events || [];

        if (!b.length || (b.length == 1 && b[0] == "queue")) {
          return __gpp.queue;
        }

        if (b.length == 1 && b[0] == "events") {
          return __gpp.events;
        }

        var cmd = b[0];
        var clb = b.length > 1 ? b[1] : null;
        var par = b.length > 2 ? b[2] : null;
        if (cmd === "ping") {
          clb(
            {
              gppVersion: "1.1", // must be “Version.Subversion”, current: “1.1”
              cmpStatus: "stub", // possible values: stub, loading, loaded, error
              cmpDisplayStatus: "hidden", // possible values: hidden, visible, disabled
              signalStatus: "not ready", // possible values: not ready, ready
              supportedAPIs: SUPPORTED_APIS, // list of supported APIs
              cmpId: CMP_ID, // IAB assigned CMP ID, may be 0 during stub/loading
              sectionList: [],
              applicableSections: [-1],
              gppString: "",
              parsedSections: {},
            },
            true
          );
        } else if (cmd === "addEventListener") {
          if (!("lastId" in __gpp)) {
            __gpp.lastId = 0;
          }
          __gpp.lastId++;
          var lnr = __gpp.lastId;
          __gpp.events.push({
            id: lnr,
            callback: clb,
            parameter: par,
          });
          clb(
            {
              eventName: "listenerRegistered",
              listenerId: lnr, // Registered ID of the listener
              data: true, // positive signal
              pingData: {
                gppVersion: "1.1", // must be “Version.Subversion”, current: “1.1”
                cmpStatus: "stub", // possible values: stub, loading, loaded, error
                cmpDisplayStatus: "hidden", // possible values: hidden, visible, disabled
                signalStatus: "not ready", // possible values: not ready, ready
                supportedAPIs: SUPPORTED_APIS, // list of supported APIs
                cmpId: CMP_ID, // list of supported APIs
                sectionList: [],
                applicableSections: [-1],
                gppString: "",
                parsedSections: {},
              },
            },
            true
          );
        } else if (cmd === "removeEventListener") {
          var success = false;
          for (var i = 0; i < __gpp.events.length; i++) {
            if (__gpp.events[i].id == par) {
              __gpp.events.splice(i, 1);
              success = true;
              break;
            }
          }
          clb(
            {
              eventName: "listenerRemoved",
              listenerId: par, // Registered ID of the listener
              data: success, // status info
              pingData: {
                gppVersion: "1.1", // must be “Version.Subversion”, current: “1.1”
                cmpStatus: "stub", // possible values: stub, loading, loaded, error
                cmpDisplayStatus: "hidden", // possible values: hidden, visible, disabled
                signalStatus: "not ready", // possible values: not ready, ready
                supportedAPIs: SUPPORTED_APIS, // list of supported APIs
                cmpId: CMP_ID, // CMP ID
                sectionList: [],
                applicableSections: [-1],
                gppString: "",
                parsedSections: {},
              },
            },
            true
          );
        } else if (cmd === "hasSection") {
          clb(false, true);
        } else if (cmd === "getSection" || cmd === "getField") {
          clb(null, true);
        }
        //queue all other commands
        else {
          __gpp.queue.push([].slice.apply(b));
        }
      };
      window.__gpp_msghandler = function (event) {
        var msgIsString = typeof event.data === "string";
        try {
          var json = msgIsString ? JSON.parse(event.data) : event.data;
        } catch (e) {
          var json = null;
        }
        if (typeof json === "object" && json !== null && "__gppCall" in json) {
          var i = json.__gppCall;
          window.__gpp(
            i.command,
            function (retValue, success) {
              var returnMsg = {
                __gppReturn: {
                  returnValue: retValue,
                  success: success,
                  callId: i.callId,
                },
              };
              event.source.postMessage(msgIsString ? JSON.stringify(returnMsg) : returnMsg, "*");
            },
            "parameter" in i ? i.parameter : null,
            "version" in i ? i.version : "1.1"
          );
        }
      };
      if (!("__gpp" in window) || typeof window.__gpp !== "function") {
        window.__gpp = window.__gpp_stub;
        window.addEventListener("message", window.__gpp_msghandler, false);
        window.__gpp_addFrame("__gppLocator");
      }
    };

    makeGppStub();

    var uspStubFunction = function() {
      var arg = arguments;
      if (typeof window.__uspapi !== uspStubFunction) {
        setTimeout(function() {
          if (typeof window.__uspapi !== 'undefined') {
            window.__uspapi.apply(window.__uspapi, arg);
          }
        }, 500);
      }
    };

    var checkIfUspIsReady = function() {
      uspTries++;
      if (window.__uspapi === uspStubFunction && uspTries < uspTriesLimit) {
        console.warn('USP is not accessible');
      } else {
        clearInterval(uspInterval);
      }
    };

    if (typeof window.__uspapi === 'undefined') {
      window.__uspapi = uspStubFunction;
      var uspInterval = setInterval(checkIfUspIsReady, 6000);
    }
  })();
  </script><script>
            FinvizSettings = {
                versionImages: 19,
                hasUserPremium: false,
                name: "",
                email: "",
                nodeChartsDomain: "https://charts2-node.finviz.com",
                hasUserStickyHeader: true,
                adsProvider: 1,
                hasRedesignEnabled: true,
                hasRedesignPortfolio: false,
                hasDarkTheme: true,
                hasEliteRedesign: true,
                quoteSearchExt: '',
                isJoinBannerVisible: false,
                hasCustomExtendedHoursEnabled: true,
            };
        </script><script src="/assets/dist-legacy/script/browser_check.7d9dede5.js"></script><script src="/assets/dist-legacy/script/vendor/boxover.202b25a7.js" defer></script>
<script src="/assets/dist-legacy/runtime.3d43022e.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/libs_init.841ee9e4.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/1150.919eca9c.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/5021.5b34e2f1.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/3962.1b0b4421.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/5738.ce83a902.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/513.12c82959.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/3115.9d91003c.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/header.2d7b887c.js" onerror="window.handleScriptNotLoaded(this)"></script><link rel="preload" as="script" href="/assets/dist-legacy/2332.dfa05e89.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/7833.b201bd22.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/7789.2052facc.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/9240.fa3428a9.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/6195.f00bc443.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/3160.1e1edcdc.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/3493.6f8c7229.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/4821.3b1fc6c2.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/255.e5361aca.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/8733.77fb88ce.js" data-chunk-id="quote-portal-components"><link rel="preload" as="script" href="/assets/dist-legacy/quote-portal-components.93158dd6.js" data-chunk-id="quote-portal-components"><link rel="stylesheet" href="/assets/dist-legacy/charts_layout.39f4e93c.css" onerror="window.handleScriptNotLoaded(this)"></head>

          <script>
            const channelIdToLabel = {
                '1': 'MarketWatch',
                '2': 'WSJ',
                '3': 'Reuters',
                '4': 'Yahoo Finance',
                '5': 'CNN',
                '6': 'The New York Times',
                '7': 'Bloomberg',
                '9': 'BBC',
                '10': 'CNBC',
                '11': 'Fox Business',
                '102': 'Mish\'s Global Economic Trend Analysis',
                '105': 'Trader Feed',
                '113': 'Howard Lindzon',
                '114': 'Seeking Alpha',
                '123': 'Fallond Stock Picks',
                '132': 'Zero Hedge',
                '133': 'market folly',
                '136': 'Daily Reckoning',
                '141': 'Abnormal Returns',
                '142': 'Calculated Risk',
            }
            function trackAndOpenNews(event, channel, url) {
              event.preventDefault()
              window.open(url, '_blank')

              let channelLabel
              if (typeof channel === 'string') {
                const isInternalNewsUrl = url.startsWith('/news/')
                channelLabel = isInternalNewsUrl ? 'internal-' + channel : channel
              } else {
                const label = channelIdToLabel[channel]
                channelLabel = label !== undefined ? label : channel
              }
              window.gtag && window.gtag('event', 'click', {
                send_to: 'G-ZT9VQEWD4N',
                non_interaction: true,
                event_category: 'news',
                event_label: channelLabel,
                value: 1 });
            }
          </script>
          <body class="m-0 yellow-tooltip is-quote min-w-[1009px] chart-tooltip table w-full">
            <script>
                window.adLayoutVersion = 'control';
                window.adLoggedIn = 'NotLoggedIn';

                var cookieName = 'fv_block';
                var selector = '[data-google-query-id]';
                var selectorFrame = selector + ' iframe, ' + selector + ' [id*=aax]';
                var cookieExpiry = 5 * 60 * 1000; // 5min
                var checkTimeout = 20 * 1000; // 20sec

                function getCookie(value) {
                    var expiration = +new Date() + cookieExpiry;
                    return cookieName + '=' + value + '; expires=' + (new Date(expiration)).toUTCString() + '; path=/';
                }

                var finvizBannersLoaded = false;
                function loadFinvizBanners(setCookie) {
                    
                    if (setCookie) document.cookie = getCookie('block');
                    finvizBannersLoaded = true;
                    var s = document.createElement('script');
                    s.type = 'text/javascript';
                    s.async = true;
                    s.src = '/assets/dist-legacy/script/finviz_b.7f5c6041.js';
                    document.head.appendChild(s);
                }

                function checkBannersLoaded() {
                    var checkEnd = +new Date() + checkTimeout;
                    function asyncCheckIfExists(selector, resolve) {
                        var now = +new Date();
                        var container = document.querySelector(selector);
                        if (!container && checkEnd > now) return setTimeout(function () { asyncCheckIfExists(selector, resolve) }, 1000)
                        resolve(!!container);
                    }

                    asyncCheckIfExists(selector, function (exists) {
                        if (!exists) return loadFinvizBanners(true);

                        asyncCheckIfExists(selectorFrame, function (hasIframe) {
                            if (!hasIframe) return loadFinvizBanners(true);
                        })
                    })
                }

                if (document.cookie.indexOf(cookieName) >= 0) {
                    loadFinvizBanners(false);
                } else {
                    var s = document.createElement('script');
                    s.type = 'text/javascript';
                    s.async = true;
                    s.onerror = loadFinvizBanners;
                    s.onload = checkBannersLoaded;
                    s.src = 'https://u5.investingchannel.com/static/uat.js';
                    document.head.appendChild(s);

                    InvestingChannelQueue = window.InvestingChannelQueue || [];
                    var ic_page;

                    function refreshAd(container, refreshes) {
                        var placementTag, adslot;
                        window.InvestingChannelQueue.push(function () {
                            var pubTags = ic_page.getPubTag.call(ic_page, container.id);
                            if (!pubTags) return;
                            var pubTag = pubTags[0];
                            placementTag = pubTag.mPlacements[0].mTagToRender;
                            adslot = pubTag.mPlacements[0].mPublisherKval.adslot[0];
                            // Update div ID
                            var id = container.id.split('_');
                            var numberOfDivs = document.querySelectorAll('[id*=' + id.slice(0, id.length - 1).join('_') + ']').length;
                            var newDivNumber = Number(id.pop()) + numberOfDivs * refreshes;
                            container.setAttribute('id', id.join('_') + '_' + newDivNumber);
                            // Destroy previous pubtag & reset container html (loading span)
                            pubTag.destroy();
                            container.innerHTML = '';
                        });
                        window.InvestingChannelQueue.push(function () {
                            if (!placementTag || !adslot) return
                            // Create new pub tag
                            var newTag;
                            var layoutId = placementTag.mNativeLayout ? placementTag.mNativeLayout.nativelayoutid : null;
                            if (layoutId) {
                                newTag = ic_page.defineNativeTag('finviz/' + placementTag.mTarget.dfpkeyname, placementTag.mAdSize, container.id, layoutId);
                                var nativeLayout, layoutData

                                try {
                                  nativeLayout = newTag.mPlacements[0].mTags[0].mNativeLayout;
                                } catch (e) {
                                    console.log(e.message)
                                }

                                try {
                                  layoutData = newTag.mTemplate.mNativeLayout[layoutId].Data
                                  if (layoutData && nativeLayout && !nativeLayout.layout) {
                                    newTag.mPlacements[0].mTags[0].mNativeLayout = layoutData
                                  }
                                } catch (e) {
                                    console.log(e.message)
                                }
                            } else {
                                newTag = ic_page.defineTag('finviz/' + placementTag.mTarget.dfpkeyname, placementTag.mAdSize, container.id);
                            }
                            // Set adslot param
                            newTag.setKval({ adslot: adslot });
                            newTag.setKval({ kw: 'ajax' });
                            newTag.render();
                        });
                    }

                    var refreshCount = 1;
                    function refreshAds(selectors) {
                        if (window.ic_page) {
                            document.querySelectorAll(selectors).forEach(function (element) {
                                try {
                                    refreshAd(element, refreshCount);
                                } catch (e) {
                                    console.log('Ad refresh error for:', element, e);
                                }
                            });
                            window.ic_page.loadMore();
                            refreshCount++;
                        }
                    }


                    InvestingChannelQueue.push(function() {
                        var icConfig = window['FINVIZ_IC_UAT_CONFIG'] = {};
                        
                        ic_page = InvestingChannel.UAT.Run('df0d0d52-cc7f-11e8-82a5-0abbb61c4a6a', icConfig);
                    });

                    var hash = null;
                    if (typeof hash === 'string') {
                      InvestingChannelQueue.push(function() {
                          if (ic_page) {
                              ic_page.setUserId('email', hash, true);
                          }
                      });
                    }
                }
            </script>
          <script>
            function checkMediaQuery(matches) {
              if (matches) {
                FinvizSettings.hasUserStickyHeader = true;
                document.body.classList.add('is-header-sticky');
              } else {
                FinvizSettings.hasUserStickyHeader = false;
                document.body.classList.remove('is-header-sticky');
              }
            }
            var mediaMatch = window.matchMedia('(min-width: 1025px) and (min-height: 650px)');
            checkMediaQuery(mediaMatch.matches);
            mediaMatch.addListener(function (ev) { checkMediaQuery(ev.matches) });
          </script>
          
            <script>
                (function () {
                    var detectionEl = document.createElement('div');
                    detectionEl.style.position='absolute';
                    detectionEl.style.overflow='scroll';
                    document.body.appendChild(detectionEl);
                    document.documentElement.style.setProperty('--fv-scrollbar-width', `${detectionEl.offsetWidth}px`);
                    document.body.removeChild(detectionEl);
                })()
            </script>
        <div id="notifications-container"></div><table class="header">
    <tr class="align-top">
        <td>
            <table class="header-container">
                <tr>
                    <td class="w-[30%]">
                        <table class="w-full">
                            <tr>
                                <td class="h-[50px] align-middle">
                                    <a href="/" class="logo"><svg width="225" height="32" class="block">
  <use href="/img/logo.svg#free" class="dark:hidden" />
  <use href="/img/logo.svg#free-dark" class="hidden dark:block" />
</svg></a>
                                </td>
                            </tr>
                            <tr>
                                <td id="search" style="padding-top: 7px">
                                    <div class="navbar-search-placeholder">
    <span class="icon-wrapper">
        <svg xmlns="http://www.w3.org/2000/svg" class="icon" fill="none" viewBox="0 0 24 24">
            <path d="M16.9 15.5l4 4c.2.2.1.5 0 .7l-.7.7a.5.5 0 01-.8 0l-4-4c0-.2-.2-.3-.3-.4l-.7-1a7 7 0 01-11.2-4 7 7 0 1112.2 3l1 .6.5.4zM5 10a5 5 0 1010 0 5 5 0 00-10 0z" />
        </svg>
    </span>
    <input placeholder="Search ticker, company or profile" class="search-input is-free"/>
</div>
                                </td>
                            </tr>
                        </table>
                    </td>
                    <td class="align-bottom pb-1">
                        <div id="microbar_position" class="hidden xl:flex items-center h-[37px] pl-2"><div>
                        <div id="IC_D_88x31_1"class="relative overflow-hidden flex items-center justify-center w-full mx-auto" style="width:88px;height:31px;max-height:31px"></div>
                        </div></div>
                    </td>
                    <td class="relative w-[730px] text-right">
                        <div id="banner_position" class="overflow-hidden absolute top-0 right-0 w-full h-[96px]">
                        <div id="IC_D_728x90_1"class="relative overflow-hidden flex items-center justify-center w-full mx-auto" style="width:728px;height:90px;max-height:90px"></div>
                        </div>
                    </td>
                </tr>
            </table>
        </td>
    </tr>
    <tr>
        <td class="w-[994px] leading-none" style="font-size:0">
            <img src="/gfx/nic2x2.gif" class="w-[994px] h-px" alt="">
        </td>
    </tr>
</table>
            <table class="navbar">
                <tr>
                    <td class="h-[30px]">
                        <table class="header-container">
                            <tr><td><a class="nav-link is-first" href="/">Home</a></td><td><a class="nav-link" href="/news.ashx">News</a></td><td><a class="nav-link" href="/screener.ashx">Screener</a></td><td><a class="nav-link" href="/map.ashx">Maps</a></td><td><a class="nav-link" href="/groups.ashx">Groups</a></td><td><a class="nav-link" href="/portfolio.ashx">Portfolio</a></td><td><a class="nav-link" href="/insidertrading.ashx">Insider</a></td><td><a class="nav-link" href="/futures.ashx">Futures</a></td><td><a class="nav-link" href="/forex.ashx">Forex</a></td><td><a class="nav-link" href="/crypto.ashx">Crypto</a></td><td><a class="nav-link" href="/elite.ashx?utm_source=finviz&utm_medium=banner&utm_campaign=main-navbar-backtests">Backtests</a></td><td><a class="nav-link is-elite" href="/elite.ashx">Pricing</a></td><td class="w-full relative"><div class="absolute bottom-0 left-0 right-0 top-0"><div id="time" class="pr-1"></div></div></td>
                    <td class="nav relative">
        <a data-testid="chart-layout-theme" href="#" class="!flex !bg-transparent !border-b-0 mt-1 !py-0 !px-1" style='border-left: 1px solid #444a57' title="Toggle Light/Dark mode" onclick="setChartThemeCookie('light', true)">
            <div class='relative box-content flex rounded-full w-10 h-5 border border-gray-750 bg-gray-800 text-white justify-end'>
                <div class='box-border w-1/2 rounded-full p-px border border-gray-800 bg-[#4c5261] flex justify-center items-center'>
                    <svg width="16" height="16" class="fill-current text-white inline-block -ml-px">
    <use href="/assets/dist-icons/icons.svg?rev=19#moonOutlined"/>
</svg>
                </div>
            </div>
            <span class='ml-1 select-none font-medium text-xs text-white'>Theme</span>
        </a>
    </td>
    
                <td>
                    <a href="/help/screener.ashx" class="nav-link is-help" style="border-left: 1px solid #444a57"><span class="fa fa-question-circle"></span>Help</a>
                </td>
                <td><a href="/login.ashx" class="nav-link sign-in">Login</a></td>
                <td><a href="/register.ashx" class="nav-link sign-up">Register</a></td>
            
                            </tr>
                        </table>
                    </td>
                </tr>
            </table>
        
          <script>
            function reloadPage () { location.reload() }
            function setChartThemeCookie(chartsTheme) {
              fetch('/api/set_cookie.ashx?cookie=chartsTheme&value=' + chartsTheme ).catch(function(){}).then(function(){
                window.gtag && window.gtag('event', 'click', { event_category: 'theme', event_label: 'toggle', value: chartsTheme, event_callback: reloadPage });
                setTimeout(reloadPage,1000);
              })
            }
          </script><div class="content ">    <div class="my-2.5 flex justify-center" data-fv-notice=promo-quote-4am>
        <div class="relative flex w-[916px] font-sm text-white rounded-md" style="background: url('data:image/jpeg;base64,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')">
            <div class="p-2 pr-10 grow text-left" style="text-shadow: rgba(34, 38, 47, 0.8) 0.5px 0.5px 0px;">
                    <p>
        <b class="text-base">NEW: Premarket Now Starts at 4AM!</b>
    </p>
    <div class="flex gap-3">
        <div>
            <img style="height:120px; border-radius: 6px" src="/img/notice/4am.png" />
        </div>
        <div>
            <p class="text-sm">We’re excited to announce the extension of premarket and aftermarket hours! Our members now have access to quotes daily from <b>4:00 AM to 8:00 PM ET</b> — offering greater flexibility and deeper insights into market trends beyond regular trading hours.</p>
            <p class="text-sm mt-1"><a  target="_blank" class="hover:underline" href="/elite.ashx?utm_source=finviz&utm_medium=banner&utm_campaign=4am"><b>Upgrade to FINVIZ*Elite to access premarket quotes »</b></a></p>
        </div>
    </div>
            </div>
            <a title="Close" class="absolute right-2 top-1.5 flex h-6 w-6 items-center justify-center" style="filter: drop-shadow(0.5px 0.5px 0px rgba(34, 38, 47, 0.8));" href="javascript:void(0)" onclick="window.FinvizCloseNotice('promo-quote-4am');">
                <svg width="20" height="20" class="shrink-0">
    <use href="/assets/dist-icons/icons.svg?rev=19#clear"/>
</svg>
            </a>
        </div>
    </div><script src="/assets/dist-legacy/script/notice.2a487c3d.js" async onerror="window.handleScriptNotLoaded(this)" onload="notice(document.querySelectorAll('[data-fv-notice]'))"></script><div class="ticker-wrapper gradient-fade" data-ticker="AACT">
<div class="fv-container py-2.5 ">    <div class="quote-header-wrapper">
        <div class="quote-header">
            <div class="quote-header_left">
                
                <div class="quote-header_ticker-wrapper">
                    <h1 class="js-recent-quote-ticker quote-header_ticker-wrapper_ticker" data-ticker="AACT">AACT</h1>
                    <h2 class="quote-header_ticker-wrapper_company text-xl">
                        <a class="tab-link block truncate" href="http://www.aresacquisitioncorporationii.com" target="_blank" rel="nofollow">
                        Ares Acquisition Corporation II
                        </a>
                    </h2>
                </div>
            </div>
                <div class="quote-header_right js-quote-price-static">
            <div class="quote-price">
        <div class="sr-only">Last Close</div>
        <span class="quote-price_date">May 05 <span class="text-muted-3">•</span> 10:59AM ET</span>
        <div class="quote-price_wrapper">
            <strong class="quote-price_wrapper_price">11.27</strong>
                <table class="quote-price_wrapper_change text-negative">
                <tr>
                    <td>
                        <div class="sr-only">Dollar change</div>
                        -0.01
                    </td>
                    <td class="align-top"><svg width="12" height="12" class="fill-current">
    <use href="/assets/dist-icons/icons.svg?rev=19#arrowDownShort"/>
</svg></td>
                </tr>
                <tr>
                    <td>
                        <div class="sr-only">Percentage change</div>
                        -0.04
                    </td>
                    <td class="font-normal">%</td>
                </tr>
            </table>
        </div>
    </div>
        
    </div>
            <div class="js-quote-price-root quote-header_right hidden"></div>
        </div>
        <div class="quote-links whitespace-nowrap gap-8">
            <div class="flex space-x-0.5 overflow-hidden">
                <a href="screener.ashx?v=111&f=sec_financial" class="tab-link">Financial</a>
                <span class="text-muted-3">•</span>
                <a href="screener.ashx?v=111&f=ind_shellcompanies" class="tab-link truncate" title="Shell Companies">Shell Companies</a>
                <span class="text-muted-3">•</span>
                <a href="screener.ashx?v=111&f=geo_usa" class="tab-link">USA</a>
                <span class="text-muted-3">•</span>
                <a href="screener.ashx?v=111&f=exch_nyse" class="tab-link">NYSE</a>
            </div>
            <div>
                <div class="js-quote-navigation-static flex space-x-0.5 shrink-0">
                    <a href="/quote.ashx?t=AACT&ta=1&p=d" class="tab-link font-semibold">
                        <span class="xl:hidden">Chart</span>
                        <span class="hidden xl:inline">Stock Detail</span>
                    </a>
                    <span class="text-muted-3">•</span>
                    <a href="elite.ashx?utm_source=finviz&utm_medium=banner&utm_campaign=quote-compare-perf" class="tab-link" data-testid="quote-compare-perf-link-static">
                        <span class="xl:hidden">Compare</span>
                        <span class="hidden xl:inline">Compare Perf.</span>
                    </a>
                        <span class="text-muted-3">•</span>
    <a href="quote.ashx?t=AACT&ta=1&p=d&ty=si" class="tab-link">Short Interest</a>
                    <span class="text-muted-3">•</span>
                    <a href="quote.ashx?t=AACT&ta=1&p=d&ty=ea" class="tab-link">Financials</a>
                    <span class="text-muted-3">•</span>
                    <a href="quote.ashx?t=AACT&ta=1&p=d&ty=tr" class="tab-link">Traffic</a>
                    <span class="text-muted-3">•</span>
                    <a data-testid="options-chain-link-static" href="quote.ashx?t=AACT&ta=1&p=d&ty=oc" class="tab-link">Options</a>
                    <span class="text-muted-3">•</span>
                    <a href="quote.ashx?t=AACT&ta=1&p=d&ty=lf" class="tab-link">
                        <span class="xl:hidden">Filings</span>
                        <span class="hidden xl:inline">Latest Filings</span>
                    </a>
                    <span class="text-muted-3">•</span>
                    <a href="/save_to_portfolio.ashx?t=AACT" class="tab-link">Add to Portfolio</a>
                    <span class="text-muted-3">•</span>
                    <a href="elite.ashx?utm_source=finviz&utm_medium=banner&utm_campaign=quote-create-alert" class="tab-link">Set Alert</a>
                </div>
                <div class="js-quote-navigation-root hidden flex space-x-0.5 shrink-0 whitespace-nowrap" data-shortinterest="true" data-isfund="false"></div>
            </div>
        </div>
    </div></div><div id="root"></div>
<script>
            FinvizSettings.TA = {"style":"candlestick","overlays":[{"name":"sma","parameters":"20","color":"rgba(220,  50, 179, 0.39)"},{"name":"sma","parameters":"50","color":"rgba(255, 143,  51, 0.78)"},{"name":"sma","parameters":"200","color":"rgba(220, 179,  50, 0.43)"},{"name":"patterns","parameters":"","color":"rgba(135, 206, 239, 1)|rgba(220, 159, 229, 1)"}],"indicators":[]};
            window.FinvizQuoteTypeCurrent = FinvizSettings.TA.style;
        </script><div class="flex flex-col items-center min-w-[1009px]">
            <div id="app" class="interactive-chart" style="min-height: 462px;">
                <div id="chart" style="opacity: 0;min-width: 990px; position: relative"></div>
            </div>
            <div id="js-charts-modal"><div class="overlay modal hidden has-footer"></div></div>
            <div class="context-menu hidden"></div>
            <script type="text/javascript">
                    window.globalChartConfig = {
          "layout":"1h1v",
          "height": 400,
          "scrollable": true,
          "colors": undefined,
          "ideas": true,
          "editable": false,
          "editors": ['tools', 'ideas', 'publish', 'timeframe', 'settings'],
          "charts":[{
                "height": 400,
                "timeframe": "d",
                "dateRange": "",
                "scale": "linear",
                "ticker": "AACT",
                "instrument": "stock",
                "refreshData": true,
                "premarket": 0,
                "aftermarket": 0,
                "hasChartEvents": true,
                "panes": []
              }],
        };
                    
                    window.FinvizQuoteTypeCurrent = FinvizSettings.TA.style;
                </script><script>
                        var data = {"ticker":"AACT","timeframe":"d","volume":[17958,57522,54054,115130,1511,13460,10110,11039,1800,154566,82398,59951,3508,83518,3851,21908,29322,200007,231949,58680,48516,33285,37565,400,203,599,246,1897,57366,54216,3901348,874997,50002,59112,4195,550279,39055,45580,281152,31974,34504,56845,7971,47077,14040,23034,240435,2305,1308,104531,144002,33413,260094,1201,604324,17003,11740,4992,11456,2701,112,40730,1919,22026,128759,8916,17447,457465,647555,178590,11071,1180,2301,454337,104893,370667,204311,707893,3860,835791,25762,1860,26650,41698,2620,105353,25415,108783,38436,27208,140552,299594,22938,65123,675,5322,31732,15208,9760,261933,2485,463010,10271,47147,102926,108750,54081,11675,10766,64726,145,67528,2380,6445,2140,4874,6433,120,379135,479,6206,46834,2305,26188,942249,148119,103734,5208,349543,143813,25000,25092,1898,111895,124603,296582,151707,54740,670097,1300523,315230,4050,97516,130785,326831,2424,143900,307792,7354,3297,764024,15608,837940,3954,43936,8750,39897,107965,33104,31713,10316,968884,2301,282622,14682,156263,27202,7677,115247,6686,81233,55041,55077,131,10299,155090,2427,335103,15046,171756,47236,43366,5588,2499,24185,23869,117053,804210,8979,70293,41658,13734,162678,59181,30125,556124,33053,2952163,12225,29260,34976,68205,12991,3197,10848,32417,23504,6128,4431,8480,1389847,85913,102729,359243,1342576,15274,578627,944170,277315,578523,113116,1355101,2382,186147,6274,1014,139491,143495,1263366,258478,188804,38752,59709,751221,161014,1139432,265035,5013,176214,894544,53006,464838,6926,6142,57392,438655,28029,816077,58114,63184,10975,501311,5757,5257,6074,232667,525258,84929,557465,22655,53450,81276,278316,3811,30426,110246,20508,5131,76000,415336,33306,9842,103890,6601,3820,56036,2535741,1784897,68201,315398,5610,146904,462676,327785,269,53145,584605,26617,9920,2122,35074,718,351907,338041,564337,47752,47110,263068,94165,2476,5799,11083,2532,2971,24842,2618,11285,315549,55377,66766,8238,40998,3851,176002,20727,43055,531040,9001,65012,2142,33360,62462,28043,53203,55446,26869,228682,628723,17125,241341,2892,83428,136749,123062,1885983,6654,569611,352570,387673,75446,11309,1774,214324,95056,108106,15136,5049,3075,34649,4594,595,32374,17423,36634,11849,241234,28649,39813,230378,248395,4269,9844,300,12848,82107,9367,146048,9506,246147,14192,4076,20678,1126,295800,105077,50095,98791,9682,22115,11678,11554,617818,30900,15008,569787,3524,269961,32264,150015,5706,11910,66021,627277,10273,5085,551743,28855,3590,16654,1606,4300,926759,2501,17816,67478,49581,4889651,11589,100918,4753052,1981091,3557515,135763,706764,14970,34226,1635625,18187,28778,59831,66479,612163,122182,15384,6381,34089,3887,686084,3476739,433703,2833543,1901713,773976,3859314,669546,7372757,3807558,54464,962099,124730,545610,1574072,1586328,307356,469777,977620,136157,20312],"date":[1688734800,1688994000,1689080400,1689253200,1689339600,1689598800,1689771600,1689858000,1689944400,1690203600,1690290000,1690376400,1690462800,1690549200,1690808400,1690894800,1690981200,1691067600,1691154000,1691499600,1691586000,1691672400,1691758800,1692018000,1692190800,1692277200,1692363600,1692622800,1692709200,1692795600,1692968400,1693227600,1693314000,1693400400,1693486800,1693573200,1693918800,1694005200,1694091600,1694178000,1694437200,1694523600,1694610000,1694696400,1694782800,1695042000,1695128400,1695214800,1695301200,1695387600,1695646800,1695733200,1695819600,1695906000,1695992400,1696251600,1696338000,1696424400,1696510800,1696856400,1696942800,1697029200,1697115600,1697202000,1697461200,1697547600,1697634000,1697720400,1697806800,1698066000,1698152400,1698238800,1698325200,1698670800,1698757200,1698843600,1698930000,1699016400,1699279200,1699365600,1699452000,1699538400,1699884000,1699970400,1700056800,1700143200,1700488800,1700575200,1700661600,1700834400,1701093600,1701180000,1701266400,1701352800,1701439200,1701698400,1701784800,1701871200,1701957600,1702044000,1702303200,1702389600,1702476000,1702562400,1702648800,1702908000,1702994400,1703080800,1703167200,1703253600,1703599200,1703685600,1703772000,1703858400,1704204000,1704290400,1704376800,1704463200,1704722400,1704808800,1704895200,1704981600,1705068000,1705413600,1705500000,1705586400,1705672800,1705932000,1706018400,1706104800,1706191200,1706277600,1706536800,1706623200,1706709600,1706796000,1706882400,1707141600,1707228000,1707314400,1707400800,1707487200,1707746400,1707832800,1707919200,1708005600,1708092000,1708437600,1708610400,1708696800,1708956000,1709042400,1709128800,1709215200,1709301600,1709560800,1709647200,1709733600,1709820000,1709906400,1710162000,1710248400,1710334800,1710421200,1710507600,1710766800,1710853200,1710939600,1711026000,1711112400,1711371600,1711458000,1711544400,1711630800,1711976400,1712062800,1712149200,1712235600,1712322000,1712581200,1712667600,1712754000,1712840400,1712926800,1713186000,1713272400,1713358800,1713445200,1713531600,1713790800,1713877200,1713963600,1714050000,1714136400,1714395600,1714482000,1714568400,1714654800,1714741200,1715000400,1715086800,1715173200,1715259600,1715346000,1715605200,1715691600,1715778000,1715864400,1715950800,1716210000,1716296400,1716382800,1716469200,1716555600,1716901200,1716987600,1717074000,1717160400,1717419600,1717506000,1717592400,1717678800,1717765200,1718024400,1718110800,1718197200,1718283600,1718370000,1718629200,1718715600,1718888400,1718974800,1719234000,1719320400,1719406800,1719493200,1719579600,1719838800,1719925200,1720011600,1720184400,1720443600,1720530000,1720616400,1720702800,1720789200,1721048400,1721134800,1721221200,1721307600,1721394000,1721653200,1721739600,1721826000,1721912400,1721998800,1722258000,1722344400,1722430800,1722517200,1722603600,1722862800,1722949200,1723035600,1723122000,1723208400,1723467600,1723554000,1723640400,1723726800,1723813200,1724072400,1724158800,1724245200,1724331600,1724418000,1724677200,1724763600,1724850000,1724936400,1725022800,1725368400,1725454800,1725541200,1725627600,1725886800,1725973200,1726059600,1726146000,1726232400,1726491600,1726578000,1726664400,1726750800,1726837200,1727096400,1727182800,1727269200,1727355600,1727442000,1727701200,1727787600,1727874000,1727960400,1728046800,1728306000,1728392400,1728478800,1728565200,1728651600,1728910800,1728997200,1729083600,1729170000,1729256400,1729515600,1729602000,1729688400,1729774800,1729861200,1730120400,1730206800,1730293200,1730379600,1730466000,1730728800,1730815200,1730901600,1730988000,1731074400,1731333600,1731420000,1731506400,1731592800,1731679200,1731938400,1732024800,1732111200,1732197600,1732284000,1732543200,1732629600,1732716000,1732888800,1733148000,1733234400,1733320800,1733407200,1733493600,1733752800,1733839200,1733925600,1734012000,1734098400,1734357600,1734444000,1734530400,1734616800,1734703200,1734962400,1735048800,1735308000,1735567200,1735653600,1735826400,1735912800,1736172000,1736258400,1736344800,1736517600,1736776800,1736863200,1736949600,1737036000,1737122400,1737468000,1737554400,1737640800,1737727200,1737986400,1738072800,1738159200,1738245600,1738332000,1738591200,1738677600,1738764000,1738850400,1738936800,1739196000,1739282400,1739368800,1739455200,1739541600,1739887200,1739973600,1740060000,1740146400,1740405600,1740492000,1740578400,1740664800,1740751200,1741010400,1741096800,1741183200,1741269600,1741356000,1741611600,1741698000,1741784400,1741870800,1741957200,1742216400,1742302800,1742389200,1742475600,1742562000,1742821200,1742907600,1742994000,1743080400,1743166800,1743426000,1743512400,1743598800,1743685200,1743771600,1744030800,1744117200,1744203600,1744290000,1744376400,1744635600,1744722000,1744808400,1744894800,1745240400,1745326800,1745413200,1745499600,1745586000,1745845200,1745931600,1746018000,1746104400,1746190800,1746450000],"open":[10.17,10.15,10.15,10.16,10.17,10.165,10.16,10.17,10.177,10.18,10.17,10.19,10.17,10.19,10.17,10.2,10.18,10.18,10.18,10.18,10.19,10.21,10.21,10.2,10.23,10.23,10.21,10.22,10.21,10.24,10.2,10.21,10.21,10.23,10.23,10.23,10.24,10.23,10.23,10.23,10.24,10.26,10.235,10.24,10.24,10.25,10.28,10.269,10.27,10.26,10.26,10.26,10.27,10.265,10.26,10.28,10.28,10.3,10.28,10.271,10.27,10.28,10.28,10.29,10.3,10.29,10.3,10.3,10.29,10.295,10.3,10.29,10.31,10.29,10.29,10.29,10.31,10.31,10.32,10.32,10.33,10.32,10.32,10.34,10.34,10.34,10.4,10.36,10.36,10.36,10.35,10.37,10.37,10.37,10.37,10.395,10.42,10.38,10.37,10.38,10.39,10.42,10.41,10.41,10.41,10.54,10.41,10.5,10.5,10.5,10.45,10.42,10.42,10.45,10.43,10.42,10.44,10.48,10.48,10.45,10.455,10.45,10.45,10.455,10.46,10.465,10.5,10.5,10.5,10.47,10.48,10.49,10.5,10.49,10.5,10.51,10.5,10.51,10.51,10.51,10.5,10.505,10.501,10.51,10.5,10.52,10.52,10.53,10.55,10.55,10.54,10.53,10.54,10.54,10.53,10.48,10.55,10.54,10.54,10.55,10.55,10.54,10.56,10.56,10.56,10.55,10.57,10.558,10.55,10.57,10.568,10.58,10.56,10.57,10.57,10.57,10.57,10.56,10.56,10.58,10.58,10.59,10.62,10.64,10.61,10.64,10.61,10.6,10.64,10.61,10.65,10.64,10.62,10.61,10.61,10.68,10.64,10.62,10.62,10.64,10.63,10.622,10.635,10.625,10.64,10.58,10.61,10.64,10.62,10.64,10.625,10.65,10.645,10.64,10.63,10.65,10.64,10.64,10.64,10.64,10.656,10.67,10.65,10.67,10.67,10.67,10.65,10.66,10.65,10.65,10.65,10.66,10.66,10.65,10.66,10.7,10.67,10.66,10.7,10.68,10.7,10.7,10.685,10.69,10.7,10.685,10.71,10.71,10.705,10.7,10.75,10.71,10.7,10.75,10.72,10.72,10.73,10.75,10.73,10.745,10.75,10.73,10.73,10.75,10.75,10.74,10.74,10.75,10.76,10.751,10.77,10.77,10.76,10.75,10.75,10.76,10.74,10.76,10.76,10.76,10.75,10.77,10.771,10.77,10.78,10.77,10.8,10.8,10.79,10.78,10.79,10.82,10.808,10.82,10.8,10.84,10.81,10.84,10.81,10.81,10.84,11,10.84,10.839,10.82,10.9,10.9,10.8,10.835,10.9,10.9,10.89,10.82,10.84,10.85,10.87,10.9,10.88,10.85,10.87,10.88,10.883,10.88,10.9,10.88,10.88,10.87,10.87,10.87,10.865,10.87,10.875,10.88,10.88,10.89,10.89,10.91,10.94,10.89,10.91,10.88,10.9,10.9,10.9,10.91,10.94,10.95,10.924,10.928,10.95,10.95,10.97,10.93,10.941,10.93,10.92,10.98,10.95,10.94,10.92,11,10.955,10.95,10.97,10.97,10.96,10.96,11,11,11,10.98,10.98,11,10.99,11.02,11.02,10.99,11.02,11.011,11.015,11.05,11.05,11.01,11.02,11.01,11,11.03,11.01,11.03,11.04,11.04,11.05,11.04,11.06,11.06,11.06,11.06,11.06,11.07,11.09,11.05,11.09,11.073,11.12,11.08,11.12,11.1,11.099,11.09,11.09,11.15,11.18,11.2,11.15,11.16,11.14,11.15,11.16,11.17,11.16,11.16,11.16,11.16,11.18,11.16,11.13,11.142,11.14,11.13,11.15,11.17,11.15,11.18,11.2,11.16,11.22,11.21,11.21,11.21,11.24,11.22,11.22,11.26,11.27,11.27,11.27,11.26,11.28],"high":[10.175,10.17,10.17,10.16,10.17,10.17,10.16,10.17,10.18,10.18,10.19,10.19,10.19,10.19,10.19,10.22,10.18,10.18,10.18,10.2,10.19,10.21,10.21,10.2,10.23,10.24,10.21,10.225,10.21,10.24,10.23,10.22,10.21,10.23,10.23,10.24,10.24,10.24,10.23,10.24,10.29,10.26,10.24,10.25,10.25,10.37,10.28,10.269,10.27,10.265,10.27,10.27,10.27,10.27,10.27,10.28,10.28,10.3,10.281,10.271,10.27,10.28,10.283,10.29,10.3,10.3,10.3,10.3,10.3,10.31,10.31,10.3,10.31,10.31,10.3,10.33,10.32,10.33,10.32,10.33,10.33,10.321,10.33,10.34,10.35,10.375,10.4,10.4,10.36,10.36,10.39,10.37,10.37,10.37,10.395,10.4,10.42,10.38,10.38,10.4,10.4,10.425,10.41,10.42,10.645,10.755,10.615,10.537,10.5,10.5,10.45,10.42,10.42,10.47,10.43,10.48,10.452,10.48,10.48,10.45,10.46,10.46,10.45,10.455,10.465,10.47,10.5,10.5,10.5,10.49,10.48,10.49,10.5,10.5,10.52,10.51,10.51,10.52,10.515,10.52,10.52,10.505,10.515,10.51,10.53,10.525,10.521,10.56,10.55,10.55,10.55,10.55,10.54,10.54,10.54,10.54,10.55,10.54,10.54,10.55,10.55,10.56,10.56,10.56,10.57,10.57,10.57,10.56,10.57,10.57,10.57,10.58,10.565,10.57,10.57,10.57,10.57,10.58,10.575,10.585,10.59,10.6,10.62,10.64,10.62,10.64,10.62,10.63,10.64,10.625,10.65,10.65,10.625,10.62,10.625,10.68,10.64,10.64,10.64,10.64,10.63,10.63,10.635,10.625,10.64,10.63,10.63,10.64,10.63,10.64,10.64,10.66,10.645,10.645,10.66,10.65,10.645,10.648,10.65,10.66,10.665,10.67,10.67,10.67,10.67,10.68,10.68,10.662,10.66,10.66,10.66,10.66,10.66,10.67,10.66,10.7,10.67,10.665,10.7,10.68,10.7,10.7,10.685,10.69,10.7,10.71,10.71,10.71,10.71,10.72,10.75,10.72,10.72,10.75,10.74,10.74,10.74,10.75,10.75,10.75,10.75,10.74,10.76,10.755,10.76,10.75,10.76,10.76,10.76,10.77,10.77,10.77,10.77,10.77,10.78,10.76,10.76,10.765,10.77,10.77,10.765,10.78,10.78,10.78,10.78,10.78,10.8,10.8,10.8,10.8,10.81,10.82,10.82,10.82,10.82,10.84,10.82,10.84,10.83,10.83,10.84,11,10.84,10.839,10.835,10.9,10.9,10.84,10.84,10.9,10.9,10.89,10.869,10.86,10.86,10.87,10.9,10.88,10.87,10.88,10.895,10.9,10.9,10.9,10.9,10.9,10.88,10.88,10.875,10.87,10.87,10.89,10.889,10.9,10.89,10.89,10.95,10.94,10.93,10.92,10.91,10.91,10.905,10.91,10.93,10.94,10.95,10.95,10.928,10.95,10.95,10.97,10.95,10.945,10.95,10.95,10.98,10.96,10.955,10.99,11,10.96,10.95,10.98,10.97,10.977,10.98,11,11,11,10.98,10.99,11,10.99,11.02,11.02,11.02,11.02,11.04,11.015,11.05,11.05,11.051,11.02,11.015,11.03,11.03,11.02,11.03,11.04,11.05,11.07,11.06,11.06,11.06,11.06,11.06,11.06,11.07,11.09,11.07,11.09,11.08,11.12,11.11,11.12,11.1,11.1,11.1,11.16,11.15,11.18,11.2,11.18,11.18,11.18,11.2,11.2,11.2,11.18,11.19,11.18,11.17,11.19,11.16,11.16,11.142,11.17,11.17,11.17,11.18,11.17,11.18,11.22,11.19,11.22,11.22,11.21,11.23,11.24,11.25,11.26,11.264,11.28,11.279,11.29,11.28,11.28],"low":[10.17,10.15,10.15,10.16,10.166,10.16,10.16,10.165,10.169,10.17,10.17,10.17,10.17,10.17,10.17,10.17,10.18,10.18,10.18,10.18,10.19,10.19,10.2,10.2,10.21,10.2,10.21,10.21,10.2,10.2,10.19,10.21,10.21,10.22,10.23,10.23,10.23,10.23,10.23,10.23,10.24,10.23,10.235,10.235,10.24,10.25,10.26,10.269,10.265,10.26,10.26,10.26,10.27,10.265,10.26,10.265,10.27,10.275,10.28,10.27,10.27,10.28,10.28,10.29,10.29,10.29,10.29,10.27,10.28,10.28,10.29,10.29,10.29,10.285,10.29,10.29,10.3,10.31,10.32,10.32,10.325,10.32,10.32,10.33,10.34,10.34,10.35,10.36,10.36,10.35,10.35,10.36,10.37,10.37,10.37,10.38,10.37,10.37,10.37,10.38,10.39,10.4,10.41,10.41,10.41,10.4,10.41,10.42,10.415,10.43,10.45,10.42,10.42,10.42,10.43,10.42,10.44,10.48,10.45,10.45,10.45,10.45,10.45,10.45,10.46,10.465,10.46,10.48,10.48,10.47,10.48,10.49,10.49,10.49,10.49,10.49,10.5,10.51,10.5,10.5,10.5,10.505,10.5,10.5,10.5,10.52,10.52,10.52,10.55,10.54,10.53,10.53,10.48,10.53,10.53,10.48,10.535,10.53,10.53,10.53,10.54,10.54,10.55,10.555,10.56,10.55,10.55,10.55,10.55,10.56,10.56,10.56,10.56,10.57,10.56,10.56,10.56,10.56,10.56,10.573,10.58,10.59,10.59,10.61,10.61,10.6,10.61,10.6,10.61,10.61,10.61,10.615,10.61,10.61,10.61,10.62,10.61,10.62,10.62,10.62,10.62,10.62,10.62,10.62,10.62,10.58,10.61,10.6,10.62,10.62,10.62,10.63,10.63,10.62,10.63,10.64,10.64,10.64,10.64,10.64,10.656,10.65,10.65,10.66,10.66,10.67,10.65,10.65,10.65,10.65,10.645,10.65,10.65,10.65,10.655,10.655,10.65,10.66,10.67,10.67,10.67,10.68,10.68,10.685,10.68,10.68,10.69,10.69,10.69,10.7,10.7,10.7,10.7,10.72,10.72,10.72,10.73,10.73,10.73,10.74,10.74,10.73,10.73,10.745,10.73,10.74,10.74,10.75,10.75,10.75,10.76,10.76,10.76,10.75,10.75,10.745,10.74,10.75,10.75,10.75,10.75,10.76,10.76,10.76,10.77,10.77,10.77,10.78,10.78,10.78,10.79,10.805,10.8,10.8,10.8,10.8,10.8,10.8,10.81,10.81,10.83,10.82,10.82,10.835,10.82,10.83,10.83,10.8,10.82,10.8,10.82,10.83,10.82,10.84,10.84,10.84,10.86,10.86,10.845,10.87,10.86,10.88,10.88,10.88,10.87,10.86,10.87,10.855,10.86,10.86,10.87,10.87,10.88,10.88,10.88,10.88,10.899,10.89,10.89,10.9,10.88,10.9,10.9,10.9,10.91,10.91,10.93,10.92,10.92,10.92,10.93,10.93,10.93,10.93,10.93,10.92,10.94,10.94,10.94,10.92,10.96,10.95,10.95,10.96,10.95,10.96,10.96,10.98,10.98,10.98,10.98,10.98,10.98,10.98,10.98,10.98,10.99,11,11,11,11,11,11.01,11.01,11.01,11,11.02,11.01,11.02,11.03,11.03,11.03,11.04,11.04,11.04,11.05,11.04,11.05,11.05,11.06,11.04,11.06,11.07,11.061,11.07,11.07,11.07,11.09,11.09,11.09,11.13,11.141,11.15,11.15,11.16,11.14,11.15,11.16,11.17,11.16,11.16,11.14,11.15,11.15,11.14,11.13,11.14,11.14,11.13,11.15,11.16,11.15,11.16,11.15,11.16,11.175,11.19,11.2,11.205,11.21,11.22,11.22,11.24,11.26,11.25,11.25,11.26,11.26],"close":[10.17,10.15,10.16,10.16,10.166,10.16,10.16,10.17,10.17,10.17,10.19,10.19,10.19,10.18,10.19,10.18,10.18,10.18,10.18,10.2,10.19,10.2,10.2,10.2,10.21,10.2,10.21,10.21,10.2,10.2,10.22,10.21,10.21,10.23,10.23,10.24,10.23,10.23,10.23,10.23,10.29,10.23,10.235,10.24,10.25,10.27,10.26,10.269,10.265,10.26,10.27,10.26,10.27,10.27,10.27,10.27,10.27,10.275,10.28,10.27,10.27,10.28,10.283,10.29,10.29,10.3,10.29,10.27,10.28,10.31,10.29,10.29,10.29,10.3,10.3,10.3,10.32,10.32,10.32,10.33,10.33,10.321,10.33,10.33,10.34,10.375,10.35,10.36,10.36,10.35,10.36,10.36,10.37,10.37,10.395,10.38,10.37,10.37,10.38,10.4,10.4,10.425,10.41,10.41,10.41,10.4,10.44,10.42,10.415,10.43,10.45,10.42,10.42,10.44,10.43,10.44,10.44,10.48,10.45,10.45,10.46,10.45,10.45,10.455,10.46,10.465,10.47,10.48,10.48,10.48,10.48,10.49,10.5,10.49,10.5,10.49,10.51,10.51,10.5,10.5,10.51,10.505,10.5,10.5,10.525,10.525,10.52,10.54,10.55,10.55,10.54,10.55,10.48,10.53,10.53,10.535,10.54,10.54,10.54,10.55,10.545,10.55,10.56,10.56,10.565,10.57,10.57,10.55,10.57,10.57,10.57,10.56,10.565,10.57,10.56,10.57,10.57,10.58,10.575,10.585,10.59,10.59,10.62,10.61,10.61,10.6,10.61,10.63,10.61,10.61,10.61,10.615,10.615,10.615,10.62,10.62,10.61,10.63,10.62,10.62,10.62,10.62,10.62,10.62,10.625,10.58,10.622,10.625,10.62,10.62,10.63,10.66,10.64,10.62,10.65,10.64,10.645,10.64,10.645,10.66,10.665,10.67,10.665,10.665,10.66,10.67,10.66,10.66,10.65,10.65,10.655,10.66,10.65,10.65,10.66,10.67,10.67,10.66,10.68,10.68,10.69,10.68,10.685,10.69,10.685,10.7,10.69,10.7,10.69,10.72,10.7,10.7,10.715,10.72,10.725,10.72,10.74,10.73,10.75,10.745,10.75,10.74,10.74,10.745,10.73,10.74,10.76,10.755,10.755,10.77,10.76,10.76,10.76,10.75,10.77,10.75,10.75,10.76,10.75,10.75,10.76,10.775,10.775,10.78,10.77,10.77,10.79,10.79,10.78,10.78,10.8,10.805,10.805,10.815,10.81,10.8,10.81,10.8,10.81,10.83,10.83,10.83,10.825,10.835,10.835,10.84,10.84,10.82,10.82,10.8,10.83,10.83,10.835,10.85,10.84,10.86,10.86,10.875,10.86,10.88,10.895,10.88,10.88,10.88,10.875,10.87,10.87,10.87,10.87,10.861,10.87,10.87,10.88,10.9,10.88,10.89,10.927,10.89,10.91,10.9,10.9,10.91,10.9,10.91,10.93,10.93,10.93,10.93,10.925,10.93,10.94,10.94,10.95,10.94,10.93,10.94,10.95,10.95,10.95,10.97,10.96,10.96,10.95,10.98,10.97,10.965,10.97,10.98,10.98,10.98,10.98,10.98,11,10.98,10.99,11,11.01,11.005,11,11.01,11,11.01,11.01,11.01,11.01,11.02,11.02,11.02,11.03,11.04,11.03,11.03,11.05,11.045,11.05,11.05,11.06,11.06,11.06,11.07,11.04,11.06,11.07,11.07,11.08,11.08,11.095,11.1,11.1,11.16,11.14,11.16,11.15,11.17,11.17,11.17,11.19,11.18,11.18,11.17,11.17,11.18,11.17,11.15,11.14,11.15,11.14,11.17,11.16,11.17,11.16,11.15,11.18,11.16,11.16,11.2,11.22,11.2,11.23,11.21,11.22,11.26,11.24,11.26,11.25,11.27,11.27,11.265],"lastOpen":11.279999732971191,"lastHigh":11.279999732971191,"lastLow":11.260000228881836,"lastClose":11.265000343322754,"lastVolume":20312,"dataId":"20312|11.265","lastDate":20250505,"lastTime":1746457192,"prevClose":11.270000457763672,"afterClose":null,"afterChange":null,"afterTime":null,"updateOhlcVersion":90,"chartEvents":[],"patterns":[{"kind":2,"strength":477.19055,"status":0,"bounces":2,"x1":159,"y1":128,"x2":600,"y2":14,"x3":0,"y3":0,"x4":0,"y4":0,"ticker":""},{"kind":3,"strength":13374.77,"status":1,"bounces":6,"x1":183,"y1":214,"x2":600,"y2":49,"x3":0,"y3":0,"x4":0,"y4":0,"ticker":""},{"kind":4,"strength":2526.3242,"status":0,"bounces":3,"x1":159,"y1":128,"x2":600,"y2":14,"x3":183,"y3":214,"x4":600,"y4":49,"ticker":""}],"patternsMinRange":10.600000381469727,"patternsMaxRange":11.300000190734863};
                        data.instrument = 'stock';
                        data.premarket = 0;
                        data.aftermarket = 0;
                        data.hasPatterns = true;
                        data.events = true;
                        data.financialAttachments = [];
                        window.globalChartConfig.quoteData = data;
                    </script></div><div class="fv-container">
<table width="100%" cellpadding="0" cellspacing="0" border="0">
<tr>
<td align="center" valign="top">
<div class="content" data-testid="quote-data-content"><table style="table-layout:fixed" width="100%"><tr>
<td align="center" valign="top">
<div class="mt-1"><div id="IC_D_970x91_1"class="relative overflow-hidden flex items-center justify-center w-full mx-auto" style="width:970px;height:100px;max-height:100px"></div></div><table width="100%" cellpding="0" cellspacing="0" class="fullview-links table-fixed">
<tbody>
<tr>
<td class="js-quote-correlation-links-container" align="left" height="20">
<div class="flex">
<div class="flex-1 max-w-max truncate">
<a class="tab-link" href="screener.ashx?t=SPC,SPAQ,SPCZ">Held by</a>:<span style="font-size:12px"> <span class="inline-flex" data-boxover="cssbody=[hoverchart] cssheader=[tabchrthdr] body=[<img src='https://charts2-node.finviz.com/chart.ashx?cs=m&t=SPC&tf=d&s=linear&pm=0&am=0&ct=candle_stick&tm=d' srcset='https://charts2-node.finviz.com/chart.ashx?cs=m&t=SPC&tf=d&s=linear&pm=0&am=0&ct=candle_stick&tm=d 1x, https://charts2-node.finviz.com/chart.ashx?cs=m&t=SPC&tf=d&s=linear&pm=0&am=0&ct=candle_stick&tm=d&sf=2 2x' width='324' height='180' alt='' referrerPolicy='no-referrer-when-downgrade' loading='lazy'><div><b>CrossingBridge Pre-Merger SPAC ETF</b>Exchange Traded Fund <span>•</span> USA <span>•</span> AUM: 30.51M </div>] offsetx=[0] offsety=[0] delay=[250]""><a href="quote.ashx?t=SPC&ty=c&ta=1&p=d" class="tab-link">SPC</a></span> <span class="inline-flex" data-boxover="cssbody=[hoverchart] cssheader=[tabchrthdr] body=[<img src='https://charts2-node.finviz.com/chart.ashx?cs=m&t=SPAQ&tf=d&s=linear&pm=0&am=0&ct=candle_stick&tm=d' srcset='https://charts2-node.finviz.com/chart.ashx?cs=m&t=SPAQ&tf=d&s=linear&pm=0&am=0&ct=candle_stick&tm=d 1x, https://charts2-node.finviz.com/chart.ashx?cs=m&t=SPAQ&tf=d&s=linear&pm=0&am=0&ct=candle_stick&tm=d&sf=2 2x' width='324' height='180' alt='' referrerPolicy='no-referrer-when-downgrade' loading='lazy'><div><b>Horizon Kinetics SPAC Active ETF</b>Exchange Traded Fund <span>•</span> USA <span>•</span> AUM: 17.02M </div>] offsetx=[0] offsety=[0] delay=[250]""><a href="quote.ashx?t=SPAQ&ty=c&ta=1&p=d" class="tab-link">SPAQ</a></span> <span class="inline-flex" "><a href="quote.ashx?t=SPCZ&ty=c&ta=1&p=d" class="tab-link">SPCZ</a></span></span></div><div class="ml-auto flex-none pl-2">
    <a class="tab-link whitespace-nowrap" href="#statements">Scroll to Statements<svg width="12" height="12" class="fill-current inline-block ml-0.5">
    <use href="/assets/dist-icons/icons.svg?rev=19#arrowDown"/>
</svg></a></div>
</div>
</td>
</tr>
</tbody>
</table>
<div style="overflow:hidden;" class="screener_snapshot-table-wrapper js-snapshot-table-wrapper"><table width="100%" cellpadding="3" cellspacing="0" border="0" class="js-snapshot-table snapshot-table2 screener_snapshot-table-body">
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Major index membership] offsetx=[10] offsety=[20] delay=[300]">Index</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Price-to-Earnings (ttm)] offsetx=[10] offsety=[20] delay=[300]">P/E</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>27.09</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Diluted EPS (ttm)] offsetx=[10] offsety=[20] delay=[300]">EPS (ttm)</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.42</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Insider ownership] offsetx=[10] offsety=[20] delay=[300]">Insider Own</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">40.16%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Shares outstanding] offsetx=[10] offsety=[20] delay=[300]">Shs Outstand</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>50.00M</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Performance (Week)] offsetx=[10] offsety=[20] delay=[300]">Perf Week</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">0.22%</span></b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Market capitalization] offsetx=[10] offsety=[20] delay=[300]">Market Cap</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>696.85M</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Forward Price-to-Earnings (next fiscal year)] offsetx=[10] offsety=[20] delay=[300]">Forward P/E</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[EPS estimate for next year] offsetx=[10] offsety=[20] delay=[300]">EPS next Y</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Insider transactions (6-Month change in Insider Ownership)] offsetx=[10] offsety=[20] delay=[300]">Insider Trans</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.00%</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Shares float] offsetx=[10] offsety=[20] delay=[300]">Shs Float</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>37.02M</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Performance (Month)] offsetx=[10] offsety=[20] delay=[300]">Perf Month</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">1.12%</span></b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Income (ttm)] offsetx=[10] offsety=[20] delay=[300]">Income</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>25.99M</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Price-to-Earnings-to-Growth] offsetx=[10] offsety=[20] delay=[300]">PEG</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[EPS estimate for next quarter] offsetx=[10] offsety=[20] delay=[300]">EPS next Q</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Institutional ownership] offsetx=[10] offsety=[20] delay=[300]">Inst Own</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>76.31%</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Short interest share] offsetx=[10] offsety=[20] delay=[300]"><a href="quote.ashx?t=AACT&ta=1&p=d&ty=si" class="hover:underline">Short Float</a></td><td class="snapshot-td2 w-[8%] " align="left" style=""><a href="quote.ashx?t=AACT&ta=1&p=d&ty=si" class="hover:underline"><b>1.29%</b></a></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Performance (Quarter)] offsetx=[10] offsety=[20] delay=[300]">Perf Quarter</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">2.32%</span></b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Revenue (ttm)] offsetx=[10] offsety=[20] delay=[300]">Sales</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.00M</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Price-to-Sales (ttm)] offsetx=[10] offsety=[20] delay=[300]">P/S</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[EPS growth this year] offsetx=[10] offsety=[20] delay=[300]">EPS this Y</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Institutional transactions (3-Month change in Institutional Ownership)] offsetx=[10] offsety=[20] delay=[300]">Inst Trans</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Short interest ratio] offsetx=[10] offsety=[20] delay=[300]"><a href="quote.ashx?t=AACT&ta=1&p=d&ty=si" class="hover:underline">Short Ratio</a></td><td class="snapshot-td2 w-[8%] " align="left" style=""><a href="quote.ashx?t=AACT&ta=1&p=d&ty=si" class="hover:underline"><b>0.55</b></a></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Performance (Half Year)] offsetx=[10] offsety=[20] delay=[300]">Perf Half Y</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">3.54%</span></b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Book value per share (mrq)] offsetx=[10] offsety=[20] delay=[300]">Book/sh</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>8.46</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Price-to-Book (mrq)] offsetx=[10] offsety=[20] delay=[300]">P/B</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>1.33</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[EPS growth next year] offsetx=[10] offsety=[20] delay=[300]">EPS next Y</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Return on Assets (ttm)] offsetx=[10] offsety=[20] delay=[300]">ROA</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>4.82%</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Short interest] offsetx=[10] offsety=[20] delay=[300]"><a href="quote.ashx?t=AACT&ta=1&p=d&ty=si" class="hover:underline">Short Interest</a></td><td class="snapshot-td2 w-[8%] " align="left" style=""><a href="quote.ashx?t=AACT&ta=1&p=d&ty=si" class="hover:underline"><b data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Settlement Date: 4/15/2025] offsetx=[10] offsety=[20] delay=[500]">0.48M</b></a></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Performance (Year)] offsetx=[10] offsety=[20] delay=[300]">Perf Year</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">6.07%</span></b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Cash per share (mrq)] offsetx=[10] offsety=[20] delay=[300]">Cash/sh</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.02</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Price to cash per share (mrq)] offsetx=[10] offsety=[20] delay=[300]">P/C</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-negative">711.07</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Long term annual growth estimate (5 years)] offsetx=[10] offsety=[20] delay=[300]">EPS next 5Y</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Return on Equity (ttm)] offsetx=[10] offsety=[20] delay=[300]">ROE</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>5.04%</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[52-Week trading range] offsetx=[10] offsety=[20] delay=[300]">52W Range</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><small>10.58 - 11.29</small></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Performance (Year To Date)] offsetx=[10] offsety=[20] delay=[300]">Perf YTD</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">2.60%</span></b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Analysts' Dividend Estimate (Fiscal Year)] offsetx=[10] offsety=[20] delay=[300]">Dividend Est.</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Price to Free Cash Flow (ttm)] offsetx=[10] offsety=[20] delay=[300]">P/FCF</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Annual EPS growth past 5 years] offsetx=[10] offsety=[20] delay=[300]">EPS past 5Y</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Return on Investment (ttm)] offsetx=[10] offsety=[20] delay=[300]">ROI</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>4.87%</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Distance from 52-Week High] offsetx=[10] offsety=[20] delay=[300]">52W High</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-negative">-0.22%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Beta] offsetx=[10] offsety=[20] delay=[300]">Beta</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-0.01</b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Trailing 12 Months Dividend] offsetx=[10] offsety=[20] delay=[300]">Dividend TTM</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Quick Ratio (mrq)] offsetx=[10] offsety=[20] delay=[300]">Quick Ratio</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>1.49</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Annual sales growth past 5 years] offsetx=[10] offsety=[20] delay=[300]">Sales past 5Y</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.00%</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Gross Margin (ttm)] offsetx=[10] offsety=[20] delay=[300]">Gross Margin</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Distance from 52-Week Low] offsetx=[10] offsety=[20] delay=[300]">52W Low</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">6.47%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Average True Range (14)] offsetx=[10] offsety=[20] delay=[300]">ATR (14)</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.03</b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Ex-Dividend Date] offsetx=[10] offsety=[20] delay=[300]">Dividend Ex-Date</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Current Ratio (mrq)] offsetx=[10] offsety=[20] delay=[300]">Current Ratio</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>1.49</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[EPS growth TTM] offsetx=[10] offsety=[20] delay=[300]">EPS Y/Y TTM</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">53.66%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Operating Margin (ttm)] offsetx=[10] offsety=[20] delay=[300]">Oper. Margin</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Relative Strength Index] offsetx=[10] offsety=[20] delay=[300]">RSI (14)</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>64.35</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Volatility (Week, Month)] offsetx=[10] offsety=[20] delay=[300]">Volatility</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><small>0.24% 0.25%</small></b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Full time employees] offsetx=[10] offsety=[20] delay=[300]">Employees</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>4</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Total Debt to Equity (mrq)] offsetx=[10] offsety=[20] delay=[300]">Debt/Eq</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">0.01</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Sales growth TTM] offsetx=[10] offsety=[20] delay=[300]">Sales Y/Y TTM</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Net Profit Margin (ttm)] offsetx=[10] offsety=[20] delay=[300]">Profit Margin</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Analysts' mean recommendation (1=Buy 5=Sell)] offsetx=[10] offsety=[20] delay=[300]">Recom</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Analysts' mean target price] offsetx=[10] offsety=[20] delay=[300]">Target Price</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Stock has options trading on a market exchange / Stock is avaiable to sell short] offsetx=[10] offsety=[20] delay=[300]">Option/Short</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>No / Yes</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Long Term Debt to Equity (mrq)] offsetx=[10] offsety=[20] delay=[300]">LT Debt/Eq</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">0.01</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Quarterly earnings growth (YoY)] offsetx=[10] offsety=[20] delay=[300]">EPS Q/Q</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-negative">-13.04%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Dividend Payout Ratio (ttm)] offsetx=[10] offsety=[20] delay=[300]">Payout</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.00%</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Relative volume] offsetx=[10] offsety=[20] delay=[300]">Rel Volume</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>0.09</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Previous close] offsetx=[10] offsety=[20] delay=[300]">Prev Close</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>11.27</b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Last quarter revenue surprise] offsetx=[10] offsety=[20] delay=[300]">Sales Surprise</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Last quarter EPS surprise] offsetx=[10] offsety=[20] delay=[300]">EPS Surprise</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Quarterly revenue growth (YoY)] offsetx=[10] offsety=[20] delay=[300]">Sales Q/Q</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Earnings date<br><br>BMO = Before Market Open<br>AMC = After Market Close] offsetx=[10] offsety=[20] delay=[300]">Earnings</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>-</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Average volume (3 month)] offsetx=[10] offsety=[20] delay=[300]">Avg Volume</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>869.28K</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Current stock price] offsetx=[10] offsety=[20] delay=[300]">Price</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>11.27</b></td>
</tr>
<tr class="table-dark-row">
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Distance from 20-Day Simple Moving Average] offsetx=[10] offsety=[20] delay=[300]">SMA20</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">0.47%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Distance from 50-Day Simple Moving Average] offsetx=[10] offsety=[20] delay=[300]">SMA50</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">0.91%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Distance from 200-Day Simple Moving Average] offsetx=[10] offsety=[20] delay=[300]">SMA200</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-positive">2.92%</span></b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Trades] offsetx=[10] offsety=[20] delay=[300]">Trades</td><td class="snapshot-td2 w-[8%] " align="left" style=""><a href="elite.ashx?utm_source=finviz&utm_medium=banner&utm_campaign=etf-fundamentals"><svg width="16" height="16" class="text-muted -ml-0.5">
    <use href="/assets/dist-icons/icons.svg?rev=19#lockOutline"/>
</svg></a></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Volume] offsetx=[10] offsety=[20] delay=[300]">Volume</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b>20,312</b></td>
<td class="snapshot-td2 cursor-pointer w-[7%]" align="left" data-boxover="cssbody=[tooltip_short_bdy] cssheader=[tooltip_short_hdr] body=[Performance (today)] offsetx=[10] offsety=[20] delay=[300]">Change</td><td class="snapshot-td2 w-[8%] " align="left" style=""><b><span class="color-text is-negative">-0.04%</span></b></td>
</tr>
</table>
</div>
</td>
</tr>
</table>
<table width="100%" cellpadding="0" cellspacing="0" border="0">
<tr><td style="height:10px;font-size:0"><img src="gfx/nic2x2.gif" style="width:685px;height:10px"></td></tr>
<tr><td align="center"><div id="IC_D_3x6_1"class="relative overflow-hidden flex items-center justify-center w-full mx-auto" style="width:970px;height:315px;max-height:315px"></div></td></tr><tr><td style="height:10px;font-size:0"><img src="gfx/nic2x2.gif" style="width:685px;height:10px"></td></tr>
<tr>
<td>
<table width="100%" cellpadding="0" cellspacing="0"><tr><td><div class="body-table-news-wrapper news-table_wrapper"><table width="100%" cellpadding="1" cellspacing="0" border="0" id="news-table" class="fullview-news-outer news-table">
    <tr class="cursor-pointer has-label" onclick="trackAndOpenNews(event, 'Business Wire', 'https://www.businesswire.com/news/home/20250421250558/en/');">
        <td width="130" align="right">
            Apr-22-25 06:30AM
        </td>
        <td align="left">
            <div class="news-link-container">
                <div class="news-link-left">
                    <a class="tab-link-news" href="https://www.businesswire.com/news/home/20250421250558/en/" target="_blank" rel="nofollow">Ares Acquisition Corporation II Announces Preliminary Redemption Results of Approximately 1.3% of Public Shares</a>
                </div>
                <div class="news-link-right">
                    <span>(Business Wire)</span></div></div></td></tr>
    <tr class="cursor-pointer has-label" onclick="trackAndOpenNews(event, 'Business Wire', 'https://www.businesswire.com/news/home/20250416568874/en/');">
        <td width="130" align="right">
            Apr-16-25 08:45AM
        </td>
        <td align="left">
            <div class="news-link-container">
                <div class="news-link-left">
                    <a class="tab-link-news" href="https://www.businesswire.com/news/home/20250416568874/en/" target="_blank" rel="nofollow">Ares Acquisition Corporation II Announces Monthly Contribution to Trust Account in Connection With Proposed Extension</a>
                </div>
                <div class="news-link-right">
                    <span>(Business Wire)</span></div></div></td></tr>
    <tr class="cursor-pointer has-label" onclick="trackAndOpenNews(event, 'The Wall Street Journal', 'https://finance.yahoo.com/m/74f4a502-86c4-3229-9ab6-060dcc17cbc8/self-driving-truck-startup.html');">
        <td width="130" align="right">
            Apr-14-25 01:52PM
        </td>
        <td align="left">
            <div class="news-link-container">
                <div class="news-link-left">
                    <a class="tab-link-news" href="https://finance.yahoo.com/m/74f4a502-86c4-3229-9ab6-060dcc17cbc8/self-driving-truck-startup.html" target="_blank" rel="nofollow">Self-Driving Truck Startup Kodiak to Go Public in $2.5 Billion SPAC Deal</a>
                </div>
                <div class="news-link-right">
                    <span>(The Wall Street Journal)</span></div></div></td></tr>
    <tr class="cursor-pointer has-label" onclick="trackAndOpenNews(event, 'PR Newswire', 'https://www.prnewswire.com/news-releases/kodiak-a-leading-provider-of-ai-powered-autonomous-vehicle-technology-to-go-public-via-business-combination-with-ares-acquisition-corporation-ii-302427427.html');">
        <td width="130" align="right">
            08:00AM
        </td>
        <td align="left">
            <div class="news-link-container">
                <div class="news-link-left">
                    <a class="tab-link-news" href="https://www.prnewswire.com/news-releases/kodiak-a-leading-provider-of-ai-powered-autonomous-vehicle-technology-to-go-public-via-business-combination-with-ares-acquisition-corporation-ii-302427427.html" target="_blank" rel="nofollow">Kodiak, a Leading Provider of AI-Powered Autonomous Vehicle Technology, to Go Public Via Business Combination With Ares Acquisition Corporation II</a>
                </div>
                <div class="news-link-right">
                    <span>(PR Newswire)</span></div></div></td></tr>
</table>
</div>
</td><td width="300" valign="top" style="padding-left: 10px"><div id="stocktwits-widget-news" class="fullview-news-outer quote_stocktwits box-content w-[300px]" data-testid="quote-stocktwits-widget-news-container">
    <div class="quote_stocktwits-bottom-cover"></div>
    <div class="quote_stocktwits-right-cover"></div>
</div>
<script type="text/javascript">
    function StocktwitsInit() {
        var StocktwitsHeight = document.getElementById('news-table').clientHeight - 0;
        if(typeof STWT !== 'undefined') {
            var isNewLayout = true;
            var isDarkTheme = true;
            var MIN_TWIT_HEIGHT = 71;
            var quoteTicker = 'AACT';
            var bgColor = isNewLayout ? (isDarkTheme ? '22262F' : 'FFFFFF') : 'transparent';
            var text_color = isNewLayout ? (isDarkTheme ? 'C3C6D0' : '4C5263') : '000000';
            var link_color = isNewLayout ? (isDarkTheme ? '57aefb' : '306DCA') : '4871a8';
            var divider_color = isNewLayout ? (isDarkTheme ? '353945' : 'DEDFE5') : 'd3d3d3';
            var time_color = isNewLayout ? '868EA5' : '999999';
            var username_color = isNewLayout ? (isDarkTheme ? 'F3F3F5' : '22262F') : '600D0B';
            var font_size = isNewLayout ? 12 : 11;
            STWT.Widget({
                container: 'stocktwits-widget-news',
                symbol: quoteTicker,
                width: '300',
                height: StocktwitsHeight,
                limit: Math.ceil(StocktwitsHeight / MIN_TWIT_HEIGHT),
                scrollbars: 0,
                header: 0,
                streaming: 'true',
                style: { link_color, link_hover_color: link_color, header_text_color: text_color, border_color: bgColor, border_color_2: bgColor, divider_color, divider_type: 'solid', box_color: bgColor, stream_color: bgColor, text_color, time_color, font: 'Verdana, Arial, Tahoma', font_size, time_font_size: 10, username_font: 'Verdana, Arial, Tahoma', username_size: font_size, username_color, username_hover_color: username_color }
            });
        }
    }

    function loadStocktwitsScript() {
        var s = document.createElement('script');
        s.addEventListener('load', StocktwitsInit)
        s.setAttribute('async', true)
        s.src = '//api.stocktwits.com/addon/widget/2/widget-loader.min.js'
        document.body.appendChild(s)
    }

    if (document.readyState != 'loading'){
        loadStocktwitsScript();
    } else {
        document.addEventListener('DOMContentLoaded', loadStocktwitsScript);
    }
</script></td></tr></table></td>
</tr>
<tr><td style="height:10px;font-size:0"><img src="gfx/nic2x2.gif" style="width:685px;height:10px"></td></tr>
<tr><td><table cellspacing="0" cellpading="0" border="0" width="100%" class="h-px">
<tr class="table-light3-row" height="1px">
<td class="fullview-profile quote_profile" align="left" valign="top"><div class="quote_profile-bio">Ares Acquisition Corp. II is a blank check company. It was formed for the purpose of effecting a merger, share exchange, asset acquisition, share purchase, reorganization or similar business combination with one or more businesses. The company was founded on March 15, 2021 and is headquartered in New York, NY.</div></td><td width="8"><img src="gfx/nic2x2.gif" style="width:8px;height:1px"></td><td class="fullview-profile quote_profile" align="left" valign="top" width="302" style="max-width: 302px;padding: 0;"><div class="managers-and-funds h-full"></div>
</td>
<script id="institutional-ownership-init-data-0" type="application/json">{"managersOwnership":[{"investorId":"1604488","name":"First Trust Capital Management L.P.","percOwnership":7.0560896},{"investorId":"1512805","name":"Westchester Capital Management, LLC","percOwnership":4.9578992},{"investorId":"1167456","name":"AQR Arbitrage LLC","percOwnership":4.8811232},{"investorId":"1951586","name":"Schechter Investment Advisors, LLC","percOwnership":4.8460496},{"investorId":"1048703","name":"Karpus Management, Inc.","percOwnership":4.7168864},{"investorId":"1009207","name":"D. E. Shaw \u0026 Co., Inc.","percOwnership":3.96},{"investorId":"1279396","name":"LINDEN ADVISORS LP","percOwnership":3.52},{"investorId":"1054587","name":"Sculptor Capital LP","percOwnership":3.3390736},{"investorId":"1539041","name":"PICTON MAHONEY ASSET MANAGEMENT","percOwnership":3.1744},{"investorId":"947263","name":"TORONTO DOMINION BANK","percOwnership":3.1357679999999997}],"fundsOwnership":[{"investorId":"S000048195","name":"First Trust Merger Arbitrage Fund","percOwnership":6.659504},{"investorId":"S000005158","name":"The Merger Fund","percOwnership":4.3895952},{"investorId":"S000056101","name":"Destinations Multi Strategy Alternatives Fund","percOwnership":0.4793984},{"investorId":"0001501072","name":"RiverNorth Opportunities Fund Inc.","percOwnership":0.4174816},{"investorId":"S000055376","name":"First Trust Multi-Strategy Fund","percOwnership":0.3965856},{"investorId":"S000040177","name":"Driehaus Event Driven Fund","percOwnership":0.3206016},{"investorId":"0000897802","name":"Special Opportunities Fund, Inc.","percOwnership":0.2970832},{"investorId":"0000810943","name":"High Income Securities Fund","percOwnership":0.27367359999999996},{"investorId":"S000059351","name":"Virtus Westchester Credit Event Fund","percOwnership":0.2428416},{"investorId":"S000048717","name":"JNL Multi-Manager Alternative Fund","percOwnership":0.2124576}]}</script></tr>
</table>
</td>
</tr>
<tr><td style="height:10px;font-size:0"><img src="gfx/nic2x2.gif" style="width:685px;height:10px"></td></tr>
<tr><td width="100%"><div id="statements"></div></td></tr><tr><td style="height:10px;font-size:0"><img src="gfx/nic2x2.gif" style="width:685px;height:10px"></td></tr>

            <tr><td>
                <script id="insider-init-data-0" type="application/json">[{"date":1715745600000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1718424000000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1721016000000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1723694400000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1726372800000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1728964800000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1731646800000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1734238800000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1736917200000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1739595600000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1742011200000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1744689600000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0},{"date":1747281600000,"saleAggregated":0,"saleTransactionCount":0,"buyAggregated":0,"buyTransactionCount":0}]</script>
                <div class="insider-trading-chart"></div>
            </td></tr>
        <table style="margin: 21px auto 14px auto" cellpadding="0" cellspacing="0" border="0">
<tr class="flex">
<td class="fullview-links flex items-center" align="left" valign="center">
<a target="_blank" class="tab-link" href="https://finance.yahoo.com/quote/AACT" rel="nofollow" onclick="window.gtag && window.gtag('event', 'click', { event_category: 'fvlinksQuote', event_label: 'yahoo' });">open in yahoo</a>&nbsp;|&nbsp<a target="_blank" class="tab-link" href="https://www.reuters.com/companies/AACT.N" rel="nofollow" onclick="window.gtag && window.gtag('event', 'click', { event_category: 'fvlinksQuote', event_label: 'reuters' });">open in reuters</a>&nbsp;|&nbsp<a target="_blank" class="tab-link" href="https://www.marketwatch.com/investing/stock/aact" rel="nofollow" onclick="window.gtag && window.gtag('event', 'click', { event_category: 'fvlinksQuote', event_label: 'marketwatch' });">open in marketwatch</a>&nbsp;|&nbsp<a target="_blank" class="tab-link" href="https://www.google.com/finance/quote/AACT:NYSE" rel="nofollow" onclick="window.gtag && window.gtag('event', 'click', { event_category: 'fvlinksQuote', event_label: 'google' });">open in google</a>&nbsp;|&nbsp<a target="_blank" class="tab-link" href="https://www.sec.gov/edgar/browse/?CIK=1853138" rel="nofollow" onclick="window.gtag && window.gtag('event', 'click', { event_category: 'fvlinksQuote', event_label: 'edgar' });">open in EDGAR</a></td>
</tr>
</table>
</table>
</td></tr>
</table>
</div>
</div>
</div><div class="my-3"><div id="IC_D_3x3_1"class="relative overflow-hidden flex items-center justify-center w-full mx-auto" style="width:970px;height:250px;max-height:250px"></div></div>
            <div class="footer" style="margin-top: 20px;padding-bottom: 215px">
                <div class="footer_links">
                    <a class="tab-link" href="/affiliate.ashx">affiliate</a>
                    <span class="footer_dot"> • </span>
                    <a class="tab-link" href="/advertise.ashx">advertise</a>
                    <span class="footer_dot"> • </span><a class="tab-link" href="/careers">careers</a>
                    <span class="footer_dot"> • </span><a class="tab-link" href="/theme">theme settings</a>
                    <span class="footer_dot"> • </span>
                    <a class="tab-link" href="/contact.ashx">contact</a>
                    <span class="footer_dot"> • </span>
                    <a class="tab-link" href="/help/screener.ashx">help</a>
                    <span class="footer_dot"> • </span>
                    <a class="tab-link" href="/privacy.ashx">privacy</a>    </div>
    Quotes delayed 15 minutes for NASDAQ, NYSE and AMEX.
    <br>
    Copyright © 2007-2025 FINVIZ.com. All Rights Reserved.
    
</div><script>SearchFocus();</script><script src="/assets/dist-legacy/2898.58fd3bdb.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/954.1180711c.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/7062.f805eba0.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/5021.5b34e2f1.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/3962.1b0b4421.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/5738.ce83a902.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/513.12c82959.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/3115.9d91003c.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/7522.ff4720ae.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/6019.ed14f4e4.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/8763.fc95b157.js" onerror="window.handleScriptNotLoaded(this)"></script><script src="/assets/dist-legacy/main.18da042b.js" onerror="window.handleScriptNotLoaded(this)"></script><link rel="preload" as="script" href="/assets/dist-legacy/recent_quotes.e00a99ee.js" data-chunk-id="recent_quotes"><script>
            function FinvizReady(fn) {
              if (document.readyState != 'loading'){
                fn();
              } else {
                document.addEventListener('DOMContentLoaded', fn);
              }
            }

            (typeof StocktwitsInit === 'function' && FinvizReady(StocktwitsInit));
            (typeof FinvizSettings === 'object' && (FinvizSettings.quoteSearchExt = '&ty=c&ta=1&p=d'));
        </script>
        <div id="js-set-search-ext-argument" class="hidden" data-set-search-ext-argument="&ty=c&ta=1&p=d"></div>
            <script>
                window.dataLayer = window.dataLayer || [];
            </script>
            <script>(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
            new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
            j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
            'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
            })(window,document,'script','dataLayer','GTM-537M973G');</script>
            <script>
              function getSystemTheme() {
                if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
                  return 'Dark';
                }
                else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: light)').matches) {
                  return 'Light';
                }
                return 'No Preference';
              }

              var systemTheme = getSystemTheme()

              const headlessChrome = navigator.userAgent.includes('HeadlessChrome')
              const webdriver = navigator.webdriver

              let cdp = false
              try {
                let accessed = false
                const e = new window.Error('ignore')
                window.Object.defineProperty(e, 'stack', {
                  configurable: false,
                  enumerable: false,
                  get: function () {
                    accessed = true
                    return ''
                  },
                })
                // This is part of the detection and shouldn't be deleted
                window.console.debug(e)
                cdp = accessed
              } catch {}

              window.dataLayer = window.dataLayer || [];
              function gtag(){dataLayer.push(arguments);}
              gtag('js', new Date());

              var fGaM = {
                'dimension1': 'NotLoggedIn',
                'dimension3': window.devicePixelRatio,
                'layoutTheme': 'dark',
                'systemTheme': systemTheme,
                'bundle': 'legacy',
                'prefTheme': 'dark',
                'themeFlag': 'legacy',
                innerWidth: window.innerWidth,
                innerHeight: window.innerHeight,
                webdriver: webdriver,
                cdp: cdp,
                isBot: headlessChrome || webdriver || cdp,
                icAdsVariant: 'control',
              };

              gtag('config', 'G-ZT9VQEWD4N', fGaM);
              
            </script>
            <script type="text/javascript">
        window._qevents = window._qevents || [];

        (function() {
            var elem = document.createElement('script');
            elem.src = (document.location.protocol == "https:" ? "https://secure" : "http://edge") + ".quantserve.com/quant.js";
            elem.async = true;
            elem.type = "text/javascript";
            var scpt = document.getElementsByTagName('script')[0];
            scpt.parentNode.insertBefore(elem, scpt);
        })();

        window._qevents.push({
            qacct:"p-c2W8esUZ6Q8oA"
        });
    </script>
    <noscript>
        <div style="display:none;">
            <img src="//pixel.quantserve.com/pixel/p-c2W8esUZ6Q8oA.gif" border="0" height="1" width="1" alt="Quantcast"/>
        </div>
    </noscript><div id="IC_D_1x1_1"class="relative overflow-hidden flex items-center justify-center w-full mx-auto"></div><div id="modal-elite-ad" class="modal-elite-ad">
                            <div id="modal-elite-ad_content" class="modal-elite-ad_content">
			                    <button id="modal-elite-ad-close" type="button" class="modal-elite-ad_close">×</button>

                                <!--<div id="modal-elite-ad-content-0" style="display: none">
			                        <h2>Ever heard of Finviz*Elite?</h2>
                                    <p>
                                        Our premium service offers you real-time quotes, advanced visualizations, technical studies, and much more.<br>
                                        Become Elite and make informed financial decisions.
                                    </p>
                                    <a href="/elite.ashx?utm_source=finviz&utm_medium=banner&utm_campaign=modal-0" id="modal-elite-ad-btn-0" class="" target="_blank">Find out more</a>
                                </div>-->

                                <div id="modal-elite-ad-content-1" style="display: block">
			                        <h2>Upgrade your FINVIZ experience</h2>
                                    <p>
                                        Join thousands of traders who make more informed decisions with&nbsp;our&nbsp;premium features.
                                        Real-time quotes, advanced&nbsp;visualizations, backtesting, and much more.
                                    </p>
                                    <a href="/elite.ashx?utm_source=finviz&utm_medium=banner&utm_campaign=modal-1" id="modal-elite-ad-btn-1" class="modal-elite_button" target="_blank">Learn more about FINVIZ*Elite</a>
                                </div>
                            </div>
                         </div><script src="/assets/dist-legacy/script/pv.cc68272e.js" async></script><script defer>window.renderScriptNotLoaded();</script>
</body>
</html>
"##;

#[test]
fn test_ticker_parser_normal_one() {
    assert_eq!(
        ticker_data_parser::TickerDataParser.extract_data(HTML_DATA, "AACT".to_string()),
        TickerData::new(
            Some(Price::new(Value::Positive(1127), -2)),
            Some(27.09),
            None,
            "AACT".to_string()
        ),
    );
}
