# \PolicyFirewallRuleDetailsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**describe_firewall_rule_on_policy**](PolicyFirewallRuleDetailsApi.md#describe_firewall_rule_on_policy) | **GET** /policies/{policyID}/firewall/rules/{firewallRuleID} | Describe a firewall rule
[**list_firewall_rules_on_policy**](PolicyFirewallRuleDetailsApi.md#list_firewall_rules_on_policy) | **GET** /policies/{policyID}/firewall/rules | List firewall rules
[**modify_firewall_rule_on_policy**](PolicyFirewallRuleDetailsApi.md#modify_firewall_rule_on_policy) | **POST** /policies/{policyID}/firewall/rules/{firewallRuleID} | Modify a firewall rule
[**reset_firewall_rule_on_policy**](PolicyFirewallRuleDetailsApi.md#reset_firewall_rule_on_policy) | **DELETE** /policies/{policyID}/firewall/rules/{firewallRuleID} | Reset firewall rule overrides



## describe_firewall_rule_on_policy

> crate::models::FirewallRule describe_firewall_rule_on_policy(policy_id, firewall_rule_id, api_version, overrides)
Describe a firewall rule

Describe a firewall rule including policy-level overrides. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.describeFirewallRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.describe_firewall_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.describeFirewallRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**firewall_rule_id** | **i32** | The ID number of the firewall rule. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::FirewallRule**](firewallRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_firewall_rules_on_policy

> crate::models::FirewallRules list_firewall_rules_on_policy(policy_id, api_version, overrides)
List firewall rules

Lists all firewall rules assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.listFirewallRulesOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.list_firewall_rules_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.listFirewallRulesOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only rules assigned to the current policy. |  |

### Return type

[**crate::models::FirewallRules**](FirewallRules.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_firewall_rule_on_policy

> crate::models::FirewallRule modify_firewall_rule_on_policy(policy_id, firewall_rule_id, api_version, firewall_rule, overrides)
Modify a firewall rule

Modify a firewall rule assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.modifyFirewallRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.modify_firewall_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.modifyFirewallRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**firewall_rule_id** | **i32** | The ID number of the firewall rule to modify. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**firewall_rule** | [**FirewallRule**](FirewallRule.md) | The settings of the firewall rule to modify. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::FirewallRule**](firewallRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_firewall_rule_on_policy

> crate::models::FirewallRule reset_firewall_rule_on_policy(policy_id, firewall_rule_id, api_version, overrides)
Reset firewall rule overrides

Remove all overrides for a firewall rule from a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.resetFirewallRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.reset_firewall_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyFirewallRuleDetailsApi.resetFirewallRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**firewall_rule_id** | **i32** | The ID number of the firewall rule to reset. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::FirewallRule**](firewallRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

