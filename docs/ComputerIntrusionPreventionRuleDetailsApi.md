# \ComputerIntrusionPreventionRuleDetailsApi

All URIs are relative to *https://workload.us-1.cloudone.trendmicro.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**describe_intrusion_prevention_rule_on_computer**](ComputerIntrusionPreventionRuleDetailsApi.md#describe_intrusion_prevention_rule_on_computer) | **GET** /computers/{computerID}/intrusionprevention/rules/{intrusionPreventionRuleID} | Describe an intrusion prevention rule
[**list_intrusion_prevention_rules_on_computer**](ComputerIntrusionPreventionRuleDetailsApi.md#list_intrusion_prevention_rules_on_computer) | **GET** /computers/{computerID}/intrusionprevention/rules | List intrusion prevention rules
[**modify_intrusion_prevention_rule_on_computer**](ComputerIntrusionPreventionRuleDetailsApi.md#modify_intrusion_prevention_rule_on_computer) | **POST** /computers/{computerID}/intrusionprevention/rules/{intrusionPreventionRuleID} | Modify an intrusion prevention rule
[**reset_intrusion_prevention_rule_on_computer**](ComputerIntrusionPreventionRuleDetailsApi.md#reset_intrusion_prevention_rule_on_computer) | **DELETE** /computers/{computerID}/intrusionprevention/rules/{intrusionPreventionRuleID} | Reset intrusion prevention rule overrides



## describe_intrusion_prevention_rule_on_computer

> crate::models::IntrusionPreventionRule describe_intrusion_prevention_rule_on_computer(computer_id, intrusion_prevention_rule_id, api_version, overrides)
Describe an intrusion prevention rule

Describe an intrusion prevention rule including computer-level overrides. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.describeIntrusionPreventionRuleOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.describe_intrusion_prevention_rule_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.describeIntrusionPreventionRuleOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**intrusion_prevention_rule_id** | **i32** | The ID number of the intrusion prevention rule. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current computer. |  |

### Return type

[**crate::models::IntrusionPreventionRule**](intrusionPreventionRule.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_intrusion_prevention_rules_on_computer

> crate::models::IntrusionPreventionRules list_intrusion_prevention_rules_on_computer(computer_id, api_version, overrides)
List intrusion prevention rules

Lists all intrusion prevention rules assigned to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.listIntrusionPreventionRulesOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.list_intrusion_prevention_rules_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.listIntrusionPreventionRulesOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only rules assigned to the current computer. |  |

### Return type

[**crate::models::IntrusionPreventionRules**](IntrusionPreventionRules.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_intrusion_prevention_rule_on_computer

> crate::models::IntrusionPreventionRule modify_intrusion_prevention_rule_on_computer(computer_id, intrusion_prevention_rule_id, api_version, intrusion_prevention_rule, overrides)
Modify an intrusion prevention rule

Modify an intrusion prevention rule assigned to a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.modifyIntrusionPreventionRuleOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.modify_intrusion_prevention_rule_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.modifyIntrusionPreventionRuleOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**intrusion_prevention_rule_id** | **i32** | The ID number of the intrusion prevention rule to modify. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**intrusion_prevention_rule** | [**IntrusionPreventionRule**](IntrusionPreventionRule.md) | The settings of the intrusion prevention rule to modify. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current computer. |  |

### Return type

[**crate::models::IntrusionPreventionRule**](intrusionPreventionRule.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_intrusion_prevention_rule_on_computer

> crate::models::IntrusionPreventionRule reset_intrusion_prevention_rule_on_computer(computer_id, intrusion_prevention_rule_id, api_version, overrides)
Reset intrusion prevention rule overrides

Remove all overrides for an intrusion prevention rule from a computer. <header class=\"param-type\">Related SDK Methods:</header><div _ngcontent-c12=\"\" class=\"params-wrap\"><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Java</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.resetIntrusionPreventionRuleOnComputer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">Python</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.reset_intrusion_prevention_rule_on_computer([param1, param2, ...])</p></span>    </div>  </div></div><div _ngcontent-c12=\"\" class=\"param\">  <div _ngcontent-c12=\"\" class=\"param-name\">    <span _ngcontent-c12=\"\" class=\"param-name-wrap\">JavaScript</span>  </div>  <div _ngcontent-c12=\"\" class=\"param-info\">    <div></div>    <div _ngcontent-c12=\"\" class=\"param-description\">      <span class=\"redoc-markdown-block\"><p>ComputerIntrusionPreventionRuleDetailsApi.resetIntrusionPreventionRuleOnComputer([param1, param2, ...])</p></span>    </div>  </div></div></div>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**computer_id** | **i32** | The ID number of the computer. | [required] |
**intrusion_prevention_rule_id** | **i32** | The ID number of the intrusion prevention rule to reset. | [required] |
**api_version** | **String** | The version of the api being called. | [required] |
**overrides** | Option<**bool**> | Show only overrides defined for the current computer. |  |

### Return type

[**crate::models::IntrusionPreventionRule**](intrusionPreventionRule.md)

### Authorization

[Legacy_API_Key](../README.md#Legacy_API_Key), [Trend_Micro_Cloud_One_API_Key](../README.md#Trend_Micro_Cloud_One_API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

