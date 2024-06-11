import rules from "./rules";

chrome.webRequest.onBeforeSendHeaders.addListener(
    (ev) => {
        const headers = ev.requestHeaders;

        headers?.push({
            name: "X-Mayhem-Education-Token",
            value: "%%_MAYHEM_EDUCATION_TOKEN_%%",
        });

        return { requestHeaders: headers };
    },
    { urls: ["<all_urls>"] },
    ["blocking", "requestHeaders", "extraHeaders"]
);

chrome.declarativeNetRequest.updateDynamicRules({
    removeRuleIds: rules.map((rule) => rule.id),
    addRules: rules,
});

export {};
