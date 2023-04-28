const allResourceTypes = Object.values(chrome.declarativeNetRequest.ResourceType);

const rules: chrome.declarativeNetRequest.Rule[] = [
    {
        id: 1,
        priority: 1,

        action: {
            type: chrome.declarativeNetRequest.RuleActionType.MODIFY_HEADERS,

            requestHeaders: [
                {
                    operation: chrome.declarativeNetRequest.HeaderOperation.SET,
                    header: "X-Mayhem-Education-Token",
                    value: "%%_MAYHEM_EDUCATION_TOKEN_%%",
                },
            ],
        },

        condition: {
            resourceTypes: allResourceTypes,
        },
    },
];

export default rules;
