# \PolicyIntrusionPreventionRuleDetailsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**describe_intrusion_prevention_rule_on_policy**](PolicyIntrusionPreventionRuleDetailsApi.md#describe_intrusion_prevention_rule_on_policy) | **GET** /policies/{policyID}/intrusionprevention/rules/{intrusionPreventionRuleID} | Describe an intrusion prevention rule
[**list_intrusion_prevention_rules_on_policy**](PolicyIntrusionPreventionRuleDetailsApi.md#list_intrusion_prevention_rules_on_policy) | **GET** /policies/{policyID}/intrusionprevention/rules | List intrusion prevention rules
[**modify_intrusion_prevention_rule_on_policy**](PolicyIntrusionPreventionRuleDetailsApi.md#modify_intrusion_prevention_rule_on_policy) | **POST** /policies/{policyID}/intrusionprevention/rules/{intrusionPreventionRuleID} | Modify an intrusion prevention rule
[**reset_intrusion_prevention_rule_on_policy**](PolicyIntrusionPreventionRuleDetailsApi.md#reset_intrusion_prevention_rule_on_policy) | **DELETE** /policies/{policyID}/intrusionprevention/rules/{intrusionPreventionRuleID} | Reset intrusion prevention rule overrides



## describe_intrusion_prevention_rule_on_policy

> crate::models::IntrusionPreventionRule describe_intrusion_prevention_rule_on_policy(policy_id, intrusion_prevention_rule_id, api_version, overrides)
Describe an intrusion prevention rule

Describe an intrusion prevention rule including policy-level overrides. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.describeIntrusionPreventionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.describe_intrusion_prevention_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.describeIntrusionPreventionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**intrusion_prevention_rule_id** | **i32** | The ID number of the intrusion prevention rule. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::IntrusionPreventionRule**](intrusionPreventionRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_intrusion_prevention_rules_on_policy

> crate::models::IntrusionPreventionRules list_intrusion_prevention_rules_on_policy(policy_id, api_version, overrides)
List intrusion prevention rules

Lists all intrusion prevention rules assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.listIntrusionPreventionRulesOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.list_intrusion_prevention_rules_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.listIntrusionPreventionRulesOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only rules assigned to the current policy. |  |

### Return type

[**crate::models::IntrusionPreventionRules**](IntrusionPreventionRules.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_intrusion_prevention_rule_on_policy

> crate::models::IntrusionPreventionRule modify_intrusion_prevention_rule_on_policy(policy_id, intrusion_prevention_rule_id, api_version, intrusion_prevention_rule, overrides)
Modify an intrusion prevention rule

Modify an intrusion prevention rule assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.modifyIntrusionPreventionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.modify_intrusion_prevention_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.modifyIntrusionPreventionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**intrusion_prevention_rule_id** | **i32** | The ID number of the intrusion prevention rule to modify. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**intrusion_prevention_rule** | [**IntrusionPreventionRule**](IntrusionPreventionRule.md) | The settings of the intrusion prevention rule to modify. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::IntrusionPreventionRule**](intrusionPreventionRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_intrusion_prevention_rule_on_policy

> crate::models::IntrusionPreventionRule reset_intrusion_prevention_rule_on_policy(policy_id, intrusion_prevention_rule_id, api_version, overrides)
Reset intrusion prevention rule overrides

Remove all overrides for an intrusion prevention rule from a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.resetIntrusionPreventionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.reset_intrusion_prevention_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyIntrusionPreventionRuleDetailsApi.resetIntrusionPreventionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**intrusion_prevention_rule_id** | **i32** | The ID number of the intrusion prevention rule to reset. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::IntrusionPreventionRule**](intrusionPreventionRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

