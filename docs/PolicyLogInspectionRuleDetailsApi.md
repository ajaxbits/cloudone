# \PolicyLogInspectionRuleDetailsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**describe_log_inspection_rule_on_policy**](PolicyLogInspectionRuleDetailsApi.md#describe_log_inspection_rule_on_policy) | **GET** /policies/{policyID}/loginspection/rules/{logInspectionRuleID} | Describe an log inspection rule
[**list_log_inspection_rules_on_policy**](PolicyLogInspectionRuleDetailsApi.md#list_log_inspection_rules_on_policy) | **GET** /policies/{policyID}/loginspection/rules | List log inspection rules
[**modify_log_inspection_rule_on_policy**](PolicyLogInspectionRuleDetailsApi.md#modify_log_inspection_rule_on_policy) | **POST** /policies/{policyID}/loginspection/rules/{logInspectionRuleID} | Modify an log inspection rule
[**reset_log_inspection_rule_on_policy**](PolicyLogInspectionRuleDetailsApi.md#reset_log_inspection_rule_on_policy) | **DELETE** /policies/{policyID}/loginspection/rules/{logInspectionRuleID} | Reset log inspection rule overrides



## describe_log_inspection_rule_on_policy

> crate::models::LogInspectionRule describe_log_inspection_rule_on_policy(policy_id, log_inspection_rule_id, api_version, overrides)
Describe an log inspection rule

Describe an log inspection rule including policy-level overrides. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.describeLogInspectionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.describe_log_inspection_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.describeLogInspectionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**log_inspection_rule_id** | **i32** | The ID number of the log inspection rule. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::LogInspectionRule**](logInspectionRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_log_inspection_rules_on_policy

> crate::models::LogInspectionRules list_log_inspection_rules_on_policy(policy_id, api_version, overrides)
List log inspection rules

Lists all log inspection rules assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.listLogInspectionRulesOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.list_log_inspection_rules_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.listLogInspectionRulesOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only rules assigned to the current policy. |  |

### Return type

[**crate::models::LogInspectionRules**](LogInspectionRules.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_log_inspection_rule_on_policy

> crate::models::LogInspectionRule modify_log_inspection_rule_on_policy(policy_id, log_inspection_rule_id, api_version, log_inspection_rule, overrides)
Modify an log inspection rule

Modify an log inspection rule assigned to a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.modifyLogInspectionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.modify_log_inspection_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.modifyLogInspectionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**log_inspection_rule_id** | **i32** | The ID number of the log inspection rule to modify. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**log_inspection_rule** | [**LogInspectionRule**](LogInspectionRule.md) | The settings of the log inspection rule to modify. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::LogInspectionRule**](logInspectionRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_log_inspection_rule_on_policy

> crate::models::LogInspectionRule reset_log_inspection_rule_on_policy(policy_id, log_inspection_rule_id, api_version, overrides)
Reset log inspection rule overrides

Remove all overrides for an log inspection rule from a policy. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.resetLogInspectionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.reset_log_inspection_rule_on_policy([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>PolicyLogInspectionRuleDetailsApi.resetLogInspectionRuleOnPolicy([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**policy_id** | **i32** | The ID number of the policy. | [required] |
**log_inspection_rule_id** | **i32** | The ID number of the log inspection rule to reset. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current policy. |  |

### Return type

[**crate::models::LogInspectionRule**](logInspectionRule.md)

### Authorization

[Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

